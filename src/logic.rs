// =========================================================================
// MODUL: HLAVNÍ LOGIKA A ZPRACOVÁNÍ DAT (logic.rs)
// =========================================================================
// Tento modul obsahuje výpočetně náročné operace:
// 1. Kontrolu aktuálnosti databáze podle času.
// 2. Filtrování a řazení seznamu partnerů.
// 3. Ověřování existence složek na disku.

use std::fs;
use std::path::Path;
use chrono::{Local, NaiveDateTime, Duration};
use crate::models::{Config, Databaze, Partner};

// -------------------------------------------------------------------------
// KONTROLA STAVU DATABÁZE
// -------------------------------------------------------------------------

/// Zjistí stav databáze (aktuální/neaktuální) na základě času poslední synchronizace.
/// 
/// # Návratová hodnota
/// * `(i32, String, String)` -> (Stavový kód, Text stavu, Čas poslední synchronizace)
///   * Kód 0: OK (Zelená v UI)
///   * Kód 1: Chyba/Chybí data (Šedá/Červená)
///   * Kód 2: Neaktuální (Oranžová)
pub fn zkontroluj_stav_db(config: &Config) -> (i32, String, String) {
    // 1. Pokusíme se otevřít a přečíst soubor s partnery
    if let Ok(data) = fs::read_to_string("partneri.json") {
        if let Ok(db) = serde_json::from_str::<Databaze>(&data) {
            
            // 2. Převedeme uložený textový čas na objekt data a času (NaiveDateTime)
            let last_sync_res = NaiveDateTime::parse_from_str(&db.posledni_sync, "%d.%m.%Y %H:%M");
            
            if let Ok(last_sync) = last_sync_res {
                let now = Local::now().naive_local();
                let diff = now.signed_duration_since(last_sync);
                
                // 3. Určíme limit (threshold) podle nastavení uživatele
                let threshold = match config.interval_synchronizace.as_str() {
                    "1 týden" => Duration::days(7),
                    "14 dní"  => Duration::days(14),
                    "1 měsíc" => Duration::days(30),
                    _         => Duration::days(7), // Výchozí pokud není vybráno
                };
                
                // 4. Vyhodnotíme, zda už interval vypršel
                if diff > threshold {
                    return (2, "DATABÁZE JE NEAKTUÁLNÍ".to_string(), db.posledni_sync);
                } else {
                    return (0, "DATABÁZE JE AKTUÁLNÍ".to_string(), db.posledni_sync);
                }
            }
        }
    }
    
    // Pokud soubor neexistuje nebo je poškozený
    (1, "DATABÁZE NENÍ NAČTENA".to_string(), "--:--".to_string())
}

// -------------------------------------------------------------------------
// PŘÍPRAVA A FILTROVÁNÍ DAT PRO TABULKU
// -------------------------------------------------------------------------

/// Načte partnery ze souboru, zkontroluje jejich složky v archivu a zfiltruje je podle UI.
/// 
/// # Argumenty
/// * `config` - Aktuální cesty k archivu
/// * `filtr_index` - 0: Vše, 1: Jen chybějící složky, 2: Vyhledávání textu
/// * `hledany_text` - Text pro vyhledávání (jméno nebo ID)
pub fn priprav_data_partneru(config: &Config, filtr_index: i32, hledany_text: String) -> (Vec<Partner>, i32, i32) {
    let mut vysledek = Vec::new();
    let mut pocet_chybi = 0;
    
    // 1. Načtení dat z JSONu
    if let Ok(data) = fs::read_to_string("partneri.json") {
        if let Ok(db) = serde_json::from_str::<Databaze>(&data) {
            let celkem = db.partneri.len() as i32;
            
            // 2. Procházení jednotlivých partnerů
            for p in db.partneri {
                // Ověření, zda existuje složka partnera v zadané cestě archivu
                let has_name = !p.slozka.trim().is_empty();
                let exists = if has_name { 
                    Path::new(&config.cesta_archiv).join(&p.slozka).exists() 
                } else { 
                    false 
                };
                
                if !exists { pocet_chybi += 1; }

                // 3. Logika filtrování (rozhodujeme, zda partnera ukážeme v tabulce)
                let vyhovuje_filtru = match filtr_index {
                    0 => true, // Zobrazit vše
                    1 => !exists, // Jen ti, co nemají složku na disku
                    2 => { // Vyhledávání podle textu (ignoruje velikost písmen)
                        let search = hledany_text.to_lowercase();
                        p.nazev.to_lowercase().contains(&search) || p.id.to_lowercase().contains(&search)
                    },
                    _ => true,
                };

                if vyhovuje_filtru { 
                    vysledek.push(p); 
                }
            }
            
            // 4. Seřazení výsledků podle ID (A-Z)
            vysledek.sort_by(|a, b| a.id.cmp(&b.id));
            
            return (vysledek, celkem, pocet_chybi);
        }
    }
    
    // Pokud se nepodaří načíst žádná data
    (Vec::new(), 0, 0)
}