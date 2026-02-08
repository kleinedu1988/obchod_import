// MRB Obchodník v0.4.0 - Hub, Import & Attachments
use slint::{ComponentHandle, SharedString};
use serde::{Deserialize, Serialize};
use std::{fs, thread};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex}; // Důležité pro sdílení příloh mezi vlákny
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

    // NOVÉ: Úložiště pro seznam příloh (PDF, STEP...)
    // Používáme Arc<Mutex<>> aby k němu mělo přístup UI (pro přidání/mazání) i pracovní vlákno (pro kopírování)
    let pending_attachments = Arc::new(Mutex::new(Vec::<PathBuf>::new()));

    let config = nacti_konfiguraci();
    main_window.set_cesta_archiv(config.cesta_archiv.clone().into());
    main_window.set_cesta_vyroba(config.cesta_vyroba.clone().into());
    main_window.set_sync_interval(config.interval_synchronizace.clone().into());
    main_window.set_verze_aplikace(env!("CARGO_PKG_VERSION").into());
    
    aktualizuj_stav_db(&main_window, &config);
    obnov_tabulku_partneru(&main_window);

    // --- 1. LOGIKA PŘÍLOH (PŘIDÁNÍ A MAZÁNÍ) ---
    
    let mw_att_handle = main_window.as_weak();
    let attachments_store = pending_attachments.clone();

    // Přidání souborů (Multiselect z modré zóny)
    main_window.on_vybrat_prilohy(move || {
        let files = rfd::FileDialog::new()
            .add_filter("Data", &["pdf", "step", "stp", "dxf", "dwg", "jpg", "png", "zip"])
            .pick_files(); // Umožní vybrat více souborů naraz

        if let Some(path_bufs) = files {
            let mut store = attachments_store.lock().unwrap();
            store.extend(path_bufs); // Přidáme nové k existujícím

            // Aktualizace UI modelu (převedeme PathBuf na Slint strukturu PrilohaData)
            let ui_data: Vec<PrilohaData> = store.iter().map(|p| {
                PrilohaData {
                    nazev: p.file_name().unwrap_or_default().to_string_lossy().to_string().into(),
                    cesta: p.to_string_lossy().to_string().into(),
                }
            }).collect();

            let ui_model = std::rc::Rc::new(slint::VecModel::from(ui_data));
            if let Some(ui) = mw_att_handle.upgrade() {
                ui.set_model_priloh(ui_model.into());
            }
        }
    });

    let mw_del_handle = main_window.as_weak();
    let attachments_del_store = pending_attachments.clone();

    // Odebrání souboru ze seznamu (Křížek u položky)
    main_window.on_odebrat_prilohu(move |index| {
        let idx = index as usize;
        let mut store = attachments_del_store.lock().unwrap();
        
        if idx < store.len() {
            store.remove(idx);
            
            // Znovu překreslit UI
            let ui_data: Vec<PrilohaData> = store.iter().map(|p| {
                PrilohaData {
                    nazev: p.file_name().unwrap_or_default().to_string_lossy().to_string().into(),
                    cesta: p.to_string_lossy().to_string().into(),
                }
            }).collect();
            
            let ui_model = std::rc::Rc::new(slint::VecModel::from(ui_data));
            if let Some(ui) = mw_del_handle.upgrade() {
                ui.set_model_priloh(ui_model.into());
            }
        }
    });

    // --- FILTRY A HLEDÁNÍ ---
    
    let mw_filter_handle = main_window.as_weak();
    main_window.on_filter_zmenen(move |index| {
        if let Some(ui) = mw_filter_handle.upgrade() {
            ui.set_aktivni_filtr(index);
            if index != 2 {
                ui.set_search_text(SharedString::from(""));
            }
            obnov_tabulku_partneru(&ui);
        }
    });

    let mw_search_handle = main_window.as_weak();
    main_window.on_search_changed(move |text| {
        if let Some(ui) = mw_search_handle.upgrade() {
            ui.set_search_text(text.clone());
            if !text.is_empty() {
                ui.set_aktivni_filtr(2);
            } else {
                ui.set_aktivni_filtr(0);
            }
            obnov_tabulku_partneru(&ui);
        }
    });

    // --- NASTAVENÍ A CESTY ---
    
    let mw_archiv_handle = main_window.as_weak();
    main_window.on_vybrat_archiv(move || {
        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
            let _ = mw_archiv_handle.upgrade().map(|ui| ui.set_cesta_archiv(folder.to_string_lossy().to_string().into()));
        }
    });

    let mw_vyroba_handle = main_window.as_weak();
    main_window.on_vybrat_vyrobu(move || {
        if let Some(folder) = rfd::FileDialog::new().pick_folder() {
            let _ = mw_vyroba_handle.upgrade().map(|ui| ui.set_cesta_vyroba(folder.to_string_lossy().to_string().into()));
        }
    });

    let mw_save_handle = main_window.as_weak();
    main_window.on_ulozit_nastaveni(move || {
        if let Some(ui) = mw_save_handle.upgrade() {
            let nova_config = Config {
                cesta_archiv: ui.get_cesta_archiv().to_string(),
                cesta_vyroba: ui.get_cesta_vyroba().to_string(),
                interval_synchronizace: ui.get_sync_interval().to_string(),
            };
            uloz_konfiguraci(nova_config.clone());
            aktualizuj_stav_db(&ui, &nova_config);
            obnov_tabulku_partneru(&ui);
        }
    });

    // --- EDITACE NÁZVU SLOŽKY ---

    let mw_edit_handle = main_window.as_weak();
    main_window.on_ulozit_nazev_slozky(move |partner_id, novy_nazev| {
        let ui = mw_edit_handle.upgrade().unwrap();
        let p_id = partner_id.to_string();
        let n_val = novy_nazev.to_string();

        if let Ok(data) = fs::read_to_string("partneri.json") {
            if let Ok(mut db) = serde_json::from_str::<Databaze>(&data) {
                if let Some(p) = db.partneri.iter_mut().find(|p| p.id == p_id) {
                    p.slozka = n_val;
                    p.aktualizovano = Local::now().format("%d.%m.%Y %H:%M").to_string();
                }
                if let Ok(json) = serde_json::to_string_pretty(&db) {
                    let _ = fs::write("partneri.json", json);
                }
            }
        }
        obnov_tabulku_partneru(&ui);
    });

    // --- HLAVNÍ LOGIKA SYNCHRONIZACE / IMPORTU ---
    
    let mw_sync_handle = main_window.as_weak();
    let pw_sync_handle = progress_window.as_weak();
    
    // Clone úložiště příloh pro vlákno zpracování
    let attachments_process_store = pending_attachments.clone(); 

    main_window.on_spustit_synchronizaci(move || {
        let ui_main = mw_sync_handle.upgrade().unwrap();
        let rezim = ui_main.get_rezim_prace(); 

        let file_path = match rfd::FileDialog::new()
            .add_filter("Excel soubory", &["xlsx", "xlsm"])
            .pick_file() {
                Some(path) => path,
                None => return,
            };

        if let Some(progress_ui) = pw_sync_handle.upgrade() {
            progress_ui.set_progress(0.0);
            let status_msg = match rezim {
                1 => "Načítám POPTÁVKU...",
                2 => "Načítám OBJEDNÁVKU...",
                _ => "Aktualizuji databázi...",
            };
            progress_ui.set_status(status_msg.into());
            let _ = progress_ui.show();
        }

        let path_to_process = file_path.to_string_lossy().to_string();
        let thread_pw = pw_sync_handle.clone();
        let thread_mw = mw_sync_handle.clone();
        let thread_attachments = attachments_process_store.clone();

        thread::spawn(move || {
            // 1. ZPRACOVÁNÍ EXCELU
            if rezim == 2 {
                println!("INFO: Zpracovávám režim OBJEDNÁVKA ze souboru: {}", path_to_process);
            } else if rezim == 1 {
                println!("INFO: Zpracovávám režim POPTÁVKA ze souboru: {}", path_to_process);
            }

            // Logika aktualizace databáze partnerů
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
                            let id_clone = id.clone();
                            let nazev_clone = nazev.clone();
                            
                            partneri_map.entry(id_clone).and_modify(|p| {
                                if p.nazev != nazev_clone {
                                    p.nazev = nazev_clone;
                                    p.aktualizovano = ted.clone();
                                }
                            }).or_insert(Partner {
                                id,
                                nazev,
                                slozka: String::new(),
                                aktualizovano: ted,
                            });
                        }

                        if idx % 10 == 0 {
                            let val = idx as f32 / total_rows;
                            let p_ui = thread_pw.clone();
                            let _ = slint::invoke_from_event_loop(move || {
                                if let Some(ui) = p_ui.upgrade() {
                                    ui.set_progress(val);
                                    ui.set_status(format!("Zpracování: řádek {}...", idx).into());
                                }
                            });
                        }
                    }
                }
            }

            // Uložení aktualizované DB
            let nyni = Local::now().format("%d.%m.%Y %H:%M").to_string();
            let nova_db = Databaze { 
                posledni_sync: nyni, 
                partneri: partneri_map.values().cloned().collect() 
            };
            if let Ok(json) = serde_json::to_string_pretty(&nova_db) {
                let _ = fs::write("partneri.json", json);
            }

            // 2. KOPÍROVÁNÍ PŘÍLOH (NOVÉ)
            // Kód se spustí pouze pokud jsou nějaké přílohy vybrány
            let attachments = thread_attachments.lock().unwrap();
            if !attachments.is_empty() {
                // Aktualizujeme status okno
                let p_ui_copy = thread_pw.clone();
                let count = attachments.len();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = p_ui_copy.upgrade() {
                        ui.set_status(format!("Kopíruji {} příloh...", count).into());
                    }
                });

                println!("--- Zpracovávám {} příloh ---", count);

                // TODO: V budoucnu zde bude logika pro dynamickou cílovou složku podle ID zakázky
                let cilova_slozka = Path::new("Vystup_Data"); 
                if let Err(e) = fs::create_dir_all(cilova_slozka) {
                     println!("Chyba při vytváření složky: {}", e);
                }

                for src_path in attachments.iter() {
                    if let Some(name) = src_path.file_name() {
                        let dest_path = cilova_slozka.join(name);
                        
                        // Kopírování souboru
                        match fs::copy(src_path, &dest_path) {
                            Ok(_) => println!("OK: Zkopírováno {:?}", name),
                            Err(e) => println!("CHYBA kopírování {:?}: {}", name, e),
                        }
                    }
                }
            }

            // Úklid po dokončení
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
                    ui.set_db_status_code(if diff > threshold { 2 } else { 0 });
                    ui.set_stav_text(if diff > threshold { "DATABÁZE JE NEAKTUÁLNÍ".into() } else { "DATABÁZE JE AKTUÁLNÍ".into() });
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
    let config = nacti_konfiguraci();
    let filtr_index = ui.get_aktivni_filtr();
    let hledany_text = ui.get_search_text().to_lowercase();

    thread::spawn(move || {
        if let Ok(data) = fs::read_to_string("partneri.json") {
            if let Ok(db) = serde_json::from_str::<Databaze>(&data) {
                let celkem_partneru = db.partneri.len() as i32;
                
                let raw_data: Vec<PartnerData> = db.partneri.into_iter().map(|p| {
                    let has_name = !p.slozka.trim().is_empty();
                    let mut exists_in_archive = false;
                    if has_name {
                        let path_arch = Path::new(&config.cesta_archiv).join(&p.slozka);
                        exists_in_archive = path_arch.exists();
                    }

                    PartnerData {
                        id: p.id.into(),
                        nazev: p.nazev.into(),
                        slozka: p.slozka.clone().into(),
                        aktualizovano: p.aktualizovano.into(),
                        ma_slozku: has_name && exists_in_archive, 
                    }
                }).collect();
                
                let ma_slozku_pocet = raw_data.iter().filter(|p| p.ma_slozku).count() as i32;
                let pocet_problemu = celkem_partneru - ma_slozku_pocet;

                // --- FILTRACE ---
                let mut filtrovana_data: Vec<PartnerData> = raw_data.into_iter().filter(|p| {
                    match filtr_index {
                        0 => true, // Celkem
                        1 => !p.ma_slozku, // Problém se složkou
                        2 => { // Hledání (ignoruje stav složky)
                            p.nazev.to_lowercase().contains(&hledany_text) || 
                            p.id.to_lowercase().contains(&hledany_text)
                        },
                        _ => true,
                    }
                }).collect();

                filtrovana_data.sort_by(|a, b| a.id.cmp(&b.id));

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = ui_handle.upgrade() {
                        let model = std::rc::Rc::new(slint::VecModel::from(filtrovana_data));
                        ui.set_model_partneru(model.into());
                        ui.set_pocet_chybi(pocet_problemu);
                        ui.set_pocet_celkem(celkem_partneru);
                    }
                });
            }
        }
    });
}