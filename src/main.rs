// =========================================================================
// HLAVNÍ SOUBOR APLIKACE (main.rs)
// =========================================================================
// Tento soubor inicializuje okno, propojuje UI s logikou a spravuje vlákna.

mod models;
mod config;
mod logic;

use slint::{ComponentHandle, VecModel}; 
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::thread;

// Načtení vygenerovaného kódu ze Slint souborů
slint::include_modules!();

// -------------------------------------------------------------------------
// VSTUPNÍ BOD PROGRAMU
// -------------------------------------------------------------------------
fn main() -> Result<(), slint::PlatformError> {
    // 1. Vytvoření instance hlavního okna
    let main_window = AppWindow::new()?;
    
    // 2. Sdílené úložiště pro vybrané přílohy (bezpečné pro přístup z více míst)
    let pending_attachments = Arc::new(Mutex::new(Vec::<PathBuf>::new()));
    
    // 3. Načtení uloženého nastavení a prvotní naplnění dat
    let cfg = config::nacti();
    nastav_ui_dle_konfigurace(&main_window, &cfg);
    obnov_ui_data(&main_window);

    // --- HANDLERY PŘÍLOH (Výběr a odebírání souborů) ---
    
    let mw_weak = main_window.as_weak();
    let attachments = pending_attachments.clone();
    main_window.on_vybrat_prilohy(move || {
        // Otevře systémový dialog pro výběr souborů
        if let Some(files) = rfd::FileDialog::new().pick_files() {
            let mut store = attachments.lock().unwrap();
            store.extend(files);
            aktualizuj_seznam_priloh(&mw_weak, &store);
        }
    });

    let mw_weak = main_window.as_weak();
    let attachments = pending_attachments.clone();
    main_window.on_odebrat_prilohu(move |idx| {
        let mut store = attachments.lock().unwrap();
        if (idx as usize) < store.len() {
            store.remove(idx as usize);
            aktualizuj_seznam_priloh(&mw_weak, &store);
        }
    });

    // --- NASTAVENÍ (Ukládání parametrů z UI) ---
    
    let mw_weak = main_window.as_weak();
    main_window.on_ulozit_nastaveni(move || {
        if let Some(ui) = mw_weak.upgrade() {
            let nova_cfg = models::Config {
                cesta_archiv: ui.get_cesta_archiv().to_string(),
                cesta_vyroba: ui.get_cesta_vyroba().to_string(),
                interval_synchronizace: ui.get_sync_interval().to_string(),
            };
            config::uloz(nova_cfg);
            obnov_ui_data(&ui); // Přepočítat stav po změně cest
        }
    });

    // --- FILTROVÁNÍ A HLEDÁNÍ (Reakce na změny v tabulce) ---
    
    let mw_weak = main_window.as_weak();
    main_window.on_filter_zmenen(move |idx| {
        if let Some(ui) = mw_weak.upgrade() {
            ui.set_aktivni_filtr(idx);
            obnov_ui_data(&ui);
        }
    });

    let mw_weak = main_window.as_weak();
    main_window.on_search_changed(move |text| {
        if let Some(ui) = mw_weak.upgrade() {
            ui.set_search_text(text);
            ui.set_aktivni_filtr(2); // Automaticky přepnout na filtr hledání
            obnov_ui_data(&ui);
        }
    });

    // 4. Spuštění smyčky událostí (aplikace běží do zavření okna)
    main_window.run()
}

// -------------------------------------------------------------------------
// POMOCNÉ FUNKCE PRO AKTUALIZACI UI
// -------------------------------------------------------------------------

/// Nastaví textová pole v sekci Nastavení podle objektu Config
fn nastav_ui_dle_konfigurace(ui: &AppWindow, cfg: &models::Config) {
    ui.set_cesta_archiv(cfg.cesta_archiv.clone().into());
    ui.set_cesta_vyroba(cfg.cesta_vyroba.clone().into());
    ui.set_sync_interval(cfg.interval_synchronizace.clone().into());
    ui.set_verze_aplikace(env!("CARGO_PKG_VERSION").into());
}

/// Hlavní funkce pro obnovu dat v tabulce a stavových informací.
/// Běží na pozadí, aby nezasekla uživatelské rozhraní při kontrole souborů.
fn obnov_ui_data(ui: &AppWindow) {
    let cfg = config::nacti();
    
    // Okamžitá kontrola stavu DB (rychlá operace)
    let (kod, text, cas) = logic::zkontroluj_stav_db(&cfg);
    ui.set_db_status_code(kod);
    ui.set_stav_text(text.into());
    ui.set_posledni_sync_cas(cas.into());

    let ui_handle = ui.as_weak();
    let filtr = ui.get_aktivni_filtr();
    let search = ui.get_search_text().to_lowercase();

    // Spuštění náročnější operace (kontrola stovek složek) v jiném vlákně
    thread::spawn(move || {
        let (partneri, celkem, chybi) = logic::priprav_data_partneru(&cfg, filtr, search);
        
        // Odeslání výsledků zpět do hlavního UI vlákna
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_handle.upgrade() {
                let slint_data: Vec<PartnerData> = partneri.into_iter().map(|p| PartnerData {
                    id: p.id.into(), 
                    nazev: p.nazev.into(), 
                    slozka: p.slozka.into(),
                    aktualizovano: p.aktualizovano.into(), 
                    ma_slozku: true // Zde by se mohl přidat i stav existence
                }).collect();
                
                ui.set_model_partneru(Rc::new(VecModel::from(slint_data)).into());
                ui.set_pocet_celkem(celkem);
                ui.set_pocet_chybi(chybi);
            }
        });
    });
}

/// Překreslí seznam souborů vybraných jako přílohy
fn aktualizuj_seznam_priloh(ui_weak: &slint::Weak<AppWindow>, store: &[PathBuf]) {
    let ui_data: Vec<PrilohaData> = store.iter().map(|p| PrilohaData {
        nazev: p.file_name().unwrap_or_default().to_string_lossy().to_string().into(),
        cesta: p.to_string_lossy().to_string().into(),
    }).collect();
    
    if let Some(ui) = ui_weak.upgrade() {
        ui.set_model_priloh(Rc::new(VecModel::from(ui_data)).into());
    }
}