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

    // 2. NAČTENÍ KONFIGURACE
    let config = nacti_konfiguraci();
    main_window.set_cesta_archiv(config.cesta_archiv.into());
    main_window.set_cesta_vyroba(config.cesta_vyroba.into());
    main_window.set_verze_aplikace(env!("CARGO_PKG_VERSION").into());
    
    // Kontrola stavu při startu
    aktualizuj_stav_db(&main_window);

    // --- CALLBACKY PRO NASTAVENÍ ---
    
    let mw_handle = main_window.as_weak();
    main_window.on_vybrat_archiv(move || {
        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
            mw_handle.unwrap().set_cesta_archiv(folder.to_string_lossy().to_string().into());
        }
    });

    let mw_handle = main_window.as_weak();
    main_window.on_vybrat_vyrobu(move || {
        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
            mw_handle.unwrap().set_cesta_vyroba(folder.to_string_lossy().to_string().into());
        }
    });

    let mw_handle = main_window.as_weak();
    main_window.on_ulozit_nastaveni(move || {
        let ui = mw_handle.unwrap();
        let cfg = Config {
            cesta_archiv: ui.get_cesta_archiv().to_string(),
            cesta_vyroba: ui.get_cesta_vyroba().to_string(),
        };
        uloz_konfiguraci(cfg);
    });

    // --- HLAVNÍ LOGIKA S DVĚMA OKNY ---
    
    let mw_handle = main_window.as_weak();       // Handle na Hlavní okno
    let pw_handle = progress_window.as_weak();   // Handle na Progress okno

    main_window.on_spustit_synchronizaci(move || {
        // Výběr souboru
        let file_path = match rfd::FileDialog::new()
            .add_filter("Excel soubory", &["xlsx", "xlsm"])
            .pick_file() {
                Some(path) => path,
                None => return,
            };

        // 1. ZOBRAZIT PROGRESS OKNO
        let progress_ui = pw_handle.unwrap();
        progress_ui.set_progress(0.0);
        progress_ui.set_status("Načítám Excel...".into());
        let _ = progress_ui.show(); // Zobrazí malé okno

        let path_to_process = file_path.to_string_lossy().to_string();
        
        // Klonování handle pro vlákno
        let thread_pw = pw_handle.clone();
        let thread_mw = mw_handle.clone();

        // 2. SPUSTIT VLÁKNO NA POZADÍ
        thread::spawn(move || {
            let mut partneri_map: HashMap<String, Partner> = HashMap::new();

            // Načtení existujících dat z JSON
            if let Ok(data) = fs::read_to_string("partneri.json") {
                if let Ok(db) = serde_json::from_str::<Databaze>(&data) {
                    for p in db.partneri { partneri_map.insert(p.id.clone(), p); }
                }
            }

            // Zpracování Excelu
            if let Ok(mut workbook) = open_workbook::<Xlsx<_>, _>(path_to_process) {
                if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
                    let rows: Vec<_> = range.rows().collect();
                    let total_rows = rows.len() as f32;

                    for (idx, row) in rows.iter().enumerate() {
                        if idx == 0 { continue; } // Přeskočit záhlaví
                        
                        let id = row[0].to_string().trim().to_string();
                        let nazev = row[1].to_string().trim().to_string();

                        if !id.is_empty() {
                            let ted = Local::now().format("%d.%m.%Y %H:%M").to_string();
                            
                            if let Some(p) = partneri_map.get_mut(&id) {
                                // Aktualizace existujícího (složka zůstává)
                                if p.nazev != nazev { 
                                    p.nazev = nazev; 
                                    p.aktualizovano = ted; 
                                }
                            } else {
                                // Nový záznam
                                partneri_map.insert(id.clone(), Partner {
                                    id, nazev, slozka: String::new(), aktualizovano: ted,
                                });
                            }
                        }

                        // Aktualizace Progress Baru (každých 5 řádků)
                        if idx % 5 == 0 {
                            let val = idx as f32 / total_rows;
                            let p_ui = thread_pw.clone();
                            let _ = slint::invoke_from_event_loop(move || {
                                let ui = p_ui.unwrap();
                                ui.set_progress(val);
                                ui.set_status(format!("Zpracovávám řádek {}...", idx).into());
                            });
                        }
                    }
                }
            }

            // Uložení výsledků do JSON
            let nyni = Local::now().format("%d.%m.%Y %H:%M").to_string();
            let nova_db = Databaze { 
                posledni_sync: nyni, 
                partneri: partneri_map.values().cloned().collect() 
            };
            
            if let Ok(json) = serde_json::to_string_pretty(&nova_db) {
                let _ = fs::write("partneri.json", json);
            }

            // UKONČENÍ: Skrýt malé okno a aktualizovat hlavní
            let _ = slint::invoke_from_event_loop(move || {
                let _ = thread_pw.unwrap().hide(); 
                aktualizuj_stav_db(&thread_mw.unwrap());
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