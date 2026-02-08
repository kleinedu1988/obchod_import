use slint::ComponentHandle; // Removed SharedString as it was unused
use serde::{Deserialize, Serialize};
use std::{fs, thread};
use std::collections::HashMap;
use std::path::Path;
use calamine::{Reader, Xlsx, open_workbook};
use chrono::{Local, NaiveDateTime, Duration};

slint::include_modules!();

// --- DATOVÉ STRUKTURY ---

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Partner {
    id: String,
    nazev: String,
    slozka: String,
    aktualizovano: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Config {
    cesta_archiv: String,
    cesta_vyroba: String,
    interval_synchronizace: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cesta_archiv: String::new(),
            cesta_vyroba: String::new(),
            interval_synchronizace: "1 týden".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Databaze {
    posledni_sync: String,
    partneri: Vec<Partner>,
}

fn main() -> Result<(), slint::PlatformError> {
    let main_window = AppWindow::new()?;
    let progress_window = ProgressWindow::new()?;

    let mut config = nacti_konfiguraci();
    main_window.set_cesta_archiv(config.cesta_archiv.clone().into());
    main_window.set_cesta_vyroba(config.cesta_vyroba.clone().into());
    main_window.set_sync_interval(config.interval_synchronizace.clone().into());
    main_window.set_verze_aplikace(env!("CARGO_PKG_VERSION").into());
    
    aktualizuj_stav_db(&main_window, &config);
    obnov_tabulku_partneru(&main_window);

    // --- CALLBACKY ---
    
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
            config.cesta_archiv = ui.get_cesta_archiv().to_string();
            config.cesta_vyroba = ui.get_cesta_vyroba().to_string();
            config.interval_synchronizace = ui.get_sync_interval().to_string();
            
            uloz_konfiguraci(config.clone());
            aktualizuj_stav_db(&ui, &config);
        }
    });

    let mw_handle = main_window.as_weak();
    let pw_handle = progress_window.as_weak();

    main_window.on_spustit_synchronizaci(move || {
        let file_path = match rfd::FileDialog::new()
            .add_filter("Excel soubory", &["xlsx", "xlsm"])
            .pick_file() {
                Some(path) => path,
                None => return,
            };

        if let Some(progress_ui) = pw_handle.upgrade() {
            progress_ui.set_progress(0.0);
            progress_ui.set_status("Načítám Excel...".into());
            let _ = progress_ui.show();
        }

        let path_to_process = file_path.to_string_lossy().to_string();
        let thread_pw = pw_handle.clone();
        let thread_mw = mw_handle.clone();

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

            let _ = slint::invoke_from_event_loop(move || {
                if let Some(pw) = thread_pw.upgrade() { let _ = pw.hide(); }
                if let Some(mw) = thread_mw.upgrade() {
                    let cfg = nacti_konfiguraci();
                    aktualizuj_stav_db(&mw, &cfg);
                    obnov_tabulku_partneru(&mw);
                }
            });
        });
    });

    main_window.run()
}

// --- POMOCNÉ FUNKCE ---

fn aktualizuj_stav_db(ui: &AppWindow, config: &Config) {
    let cesta = Path::new("partneri.json");
    if cesta.exists() {
        if let Ok(data) = fs::read_to_string(cesta) {
            if let Ok(db) = serde_json::from_str::<Databaze>(&data) {
                ui.set_posledni_sync_cas(db.posledni_sync.clone().into());
                
                let last_sync_res = NaiveDateTime::parse_from_str(&db.posledni_sync, "%d.%m.%Y %H:%M");
                
                if let Ok(last_sync) = last_sync_res {
                    let now = Local::now().naive_local();
                    let diff = now.signed_duration_since(last_sync);
                    
                    let threshold = match config.interval_synchronizace.as_str() {
                        "1 týden" => Duration::days(7),
                        "14 dní" => Duration::days(14),
                        "1 měsíc" => Duration::days(30),
                        "6 měsíců" => Duration::days(180),
                        "teď..." => Duration::seconds(0),
                        _ => Duration::days(7),
                    };

                    if diff > threshold {
                        ui.set_db_status_code(2); 
                        ui.set_stav_text("DATABÁZE JE NEAKTUÁLNÍ".into());
                    } else {
                        ui.set_db_status_code(0); 
                        ui.set_stav_text("DATABÁZE JE AKTUÁLNÍ".into());
                    }
                } else {
                    ui.set_db_status_code(1);
                    ui.set_stav_text("CHYBA FORMÁTU DATA".into());
                }
                return;
            }
        }
    }
    ui.set_db_status_code(1);
    ui.set_stav_text("DATABÁZE NENÍ NAČTENA".into());
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
        if let Ok(data) = fs::read_to_string("partneri.json") {
            if let Ok(db) = serde_json::from_str::<Databaze>(&data) {
                let celkem = db.partneri.len() as i32;
                let ma_slozku_pocet = db.partneri.iter()
                    .filter(|p| !p.slozka.trim().is_empty())
                    .count() as i32;
                let chybi_pocet = celkem - ma_slozku_pocet;

                let raw_data: Vec<PartnerData> = db.partneri.into_iter().map(|p| {
                    PartnerData {
                        id: p.id.into(),
                        nazev: p.nazev.into(),
                        slozka: p.slozka.clone().into(),
                        aktualizovano: p.aktualizovano.into(),
                        ma_slozku: !p.slozka.is_empty(),
                    }
                }).collect();

                let _ = slint::invoke_from_event_loop(move || {
                    let model = std::rc::Rc::new(slint::VecModel::from(raw_data));
                    if let Some(ui) = ui_handle.upgrade() {
                        ui.set_model_partneru(model.into());
                        ui.set_pocet_chybi(chybi_pocet);
                    }
                });
            }
        }
    });
}