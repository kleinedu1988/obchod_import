use slint::ComponentHandle;
use serde::{Deserialize, Serialize};
use std::{fs, thread};
use std::collections::HashMap;
use std::path::Path;
use calamine::{Reader, Xlsx, open_workbook};
use chrono::Local;

slint::include_modules!();

// --- DATOVÉ STRUKTURY ---

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Partner {
    id: String,
    nazev: String,
    slozka: String,
    aktualizovano: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct Config {
    cesta_archiv: String,
    cesta_vyroba: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Databaze {
    posledni_sync: String,
    partneri: Vec<Partner>,
}

fn main() -> Result<(), slint::PlatformError> {
    // 1. VYTVOŘENÍ OBOU OKEN
    let main_window = AppWindow::new()?;
    let progress_window = ProgressWindow::new()?;

    // 2. NAČTENÍ KONFIGURACE A STAVU
    let config = nacti_konfiguraci();
    main_window.set_cesta_archiv(config.cesta_archiv.into());
    main_window.set_cesta_vyroba(config.cesta_vyroba.into());
    main_window.set_verze_aplikace(env!("CARGO_PKG_VERSION").into());
    
    // Kontrola stavu při startu
    aktualizuj_stav_db(&main_window);
    obnov_tabulku_partneru(&main_window);

    // --- CALLBACKY PRO NASTAVENÍ ---
    
    let mw_handle = main_window.as_weak();
    main_window.on_vybrat_archiv(move || {
        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
            let _ = mw_handle.upgrade().map(|ui| ui.set_cesta_archiv(folder.to_string_lossy().to_string().into()));
        }
    });

    let mw_handle = main_window.as_weak();
    main_window.on_vybrat_vyrobu(move || {
        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
            let _ = mw_handle.upgrade().map(|ui| ui.set_cesta_vyroba(folder.to_string_lossy().to_string().into()));
        }
    });

    let mw_handle = main_window.as_weak();
    main_window.on_ulozit_nastaveni(move || {
        if let Some(ui) = mw_handle.upgrade() {
            let cfg = Config {
                cesta_archiv: ui.get_cesta_archiv().to_string(),
                cesta_vyroba: ui.get_cesta_vyroba().to_string(),
            };
            uloz_konfiguraci(cfg);
        }
    });

    // --- HLAVNÍ LOGIKA S DVĚMA OKNY ---
    
    let mw_handle = main_window.as_weak();
    let pw_handle = progress_window.as_weak();

    main_window.on_spustit_synchronizaci(move || {
        // Výběr souboru
        let file_path = match rfd::FileDialog::new()
            .add_filter("Excel soubory", &["xlsx", "xlsm"])
            .pick_file() {
                Some(path) => path,
                None => return,
            };

        // 1. ZOBRAZIT PROGRESS OKNO
        if let Some(progress_ui) = pw_handle.upgrade() {
            progress_ui.set_progress(0.0);
            progress_ui.set_status("Načítám Excel...".into());
            let _ = progress_ui.show();
        }

        let path_to_process = file_path.to_string_lossy().to_string();
        let thread_pw = pw_handle.clone();
        let thread_mw = mw_handle.clone();

        // 2. SPUSTIT VLÁKNO NA POZADÍ
        thread::spawn(move || {
            let mut partneri_map: HashMap<String, Partner> = HashMap::new();

            if let Ok(data) = fs::read_to_string("partneri.json") {
                if let Ok(db) = serde_json::from_str::<Databaze>(&data) {
                    for p in db.partneri { partneri_map.insert(p.id.clone(), p); }
                }
            }

            if let Ok(mut workbook) = open_workbook::<Xlsx<_>, _>(path_to_process) {
                if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
                    let rows: Vec<_> = range.rows().collect();
                    let total_rows = rows.len() as f32;

                    for (idx, row) in rows.iter().enumerate() {
                        if idx == 0 { continue; }
                        
                        let id = row[0].to_string().trim().to_string();
                        let nazev = row[1].to_string().trim().to_string();

                        if !id.is_empty() {
                            let ted = Local::now().format("%d.%m.%Y %H:%M").to_string();
                            if let Some(p) = partneri_map.get_mut(&id) {
                                if p.nazev != nazev { 
                                    p.nazev = nazev; 
                                    p.aktualizovano = ted; 
                                }
                            } else {
                                partneri_map.insert(id.clone(), Partner {
                                    id, nazev, slozka: String::new(), aktualizovano: ted,
                                });
                            }
                        }

                        if idx % 5 == 0 {
                            let val = idx as f32 / total_rows;
                            let p_ui = thread_pw.clone();
                            let _ = slint::invoke_from_event_loop(move || {
                                if let Some(ui) = p_ui.upgrade() {
                                    ui.set_progress(val);
                                    ui.set_status(format!("Zpracovávám řádek {}...", idx).into());
                                }
                            });
                        }
                    }
                }
            }

            let nyni = Local::now().format("%d.%m.%Y %H:%M").to_string();
            let nova_db = Databaze { 
                posledni_sync: nyni, 
                partneri: partneri_map.values().cloned().collect() 
            };
            
            if let Ok(json) = serde_json::to_string_pretty(&nova_db) {
                let _ = fs::write("partneri.json", json);
            }

            // UKONČENÍ: Skrýt malé okno a aktualizovat tabulku v hlavním okně
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(pw) = thread_pw.upgrade() { let _ = pw.hide(); }
                if let Some(mw) = thread_mw.upgrade() {
                    aktualizuj_stav_db(&mw);
                    obnov_tabulku_partneru(&mw);
                }
            });
        });
    });

    main_window.run()
}

// --- POMOCNÉ FUNKCE ---

fn aktualizuj_stav_db(ui: &AppWindow) {
    let cesta = Path::new("partneri.json");
    if cesta.exists() {
        if let Ok(data) = fs::read_to_string(cesta) {
            if let Ok(db) = serde_json::from_str::<Databaze>(&data) {
                ui.set_databaze_ok(true);
                ui.set_stav_text("DATABÁZE JE AKTUÁLNÍ".into());
                ui.set_posledni_sync_cas(db.posledni_sync.into());
                return;
            }
        }
    }
    ui.set_databaze_ok(false);
    ui.set_stav_text("DATABÁZE NENÍ NAČTENA".into());
    ui.set_posledni_sync_cas("Nikdy".into());
}

fn nacti_konfiguraci() -> Config {
    fs::read_to_string("nastaveni.json")
        .and_then(|data| serde_json::from_str(&data).map_err(|e| e.into()))
        .unwrap_or_default()
}

fn uloz_konfiguraci(cfg: Config) {
    if let Ok(json) = serde_json::to_string_pretty(&cfg) {
        let _ = fs::write("nastaveni.json", json);
    }
}

fn obnov_tabulku_partneru(ui: &AppWindow) {
    let ui_handle = ui.as_weak();

    thread::spawn(move || {
        // 1. TĚŽKÁ PRÁCE NA POZADÍ (Načtení a parsování 14k záznamů)
        if let Ok(data) = fs::read_to_string("partneri.json") {
            if let Ok(db) = serde_json::from_str::<Databaze>(&data) {
                
                // Připravíme si čistý vektor dat (tohle je Send, takže to může cestovat)
                let raw_data: Vec<PartnerData> = db.partneri.into_iter().map(|p| {
                    PartnerData {
                        id: p.id.into(),
                        nazev: p.nazev.into(),
                        slozka: p.slozka.clone().into(),
                        aktualizovano: p.aktualizovano.into(),
                        ma_slozku: !p.slozka.is_empty(),
                    }
                }).collect();

                // 2. PŘEPNUTÍ DO HLAVNÍHO VLÁKNA
                let _ = slint::invoke_from_event_loop(move || {
                    // Tady už jsme v UI vlákně. Tady vytvoříme Model.
                    // Vytvoření modelu z hotového vektoru je bleskové (O(1)).
                    let model = std::rc::Rc::new(slint::VecModel::from(raw_data));
                    
                    if let Some(ui) = ui_handle.upgrade() {
                        ui.set_model_partneru(model.into());
                    }
                });
            }
        }
    });
}