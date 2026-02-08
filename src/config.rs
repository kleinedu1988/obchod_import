// =========================================================================
// MODUL: SPRÁVA KONFIGURACE (config.rs)
// =========================================================================
// Tento modul se stará o čtení a zápis nastavení aplikace do JSON souboru.
// Zajišťuje, aby si aplikace pamatovala cesty ke složkám i po restartu.

use std::fs;
use crate::models::Config;

// -------------------------------------------------------------------------
// LOGIKA NAČÍTÁNÍ
// -------------------------------------------------------------------------

/// Pokusí se načíst konfigurační data ze souboru "nastaveni.json".
/// 
/// # Návratová hodnota
/// * Vrátí naplněnou strukturu `Config`.
/// * Pokud soubor neexistuje nebo je poškozený, vrátí výchozí hodnoty (Default).
pub fn nacti() -> Config {
    // 1. Pokus o přečtení souboru jako textu
    fs::read_to_string("nastaveni.json")
        // 2. Pokud se čtení povedlo, zkusíme převést text (JSON) na strukturu Config
        .and_then(|data| serde_json::from_str(&data).map_err(|e| e.into()))
        // 3. Pokud soubor chybí nebo parsování selže, použijeme definované Default hodnoty
        .unwrap_or_default()
}

// -------------------------------------------------------------------------
// LOGIKA UKLÁDÁNÍ
// -------------------------------------------------------------------------

/// Uloží aktuální stav konfigurace do souboru "nastaveni.json".
/// 
/// # Argumenty
/// * `cfg` - Struktura Config obsahující aktuální cesty a intervaly z UI.
pub fn uloz(cfg: Config) {
    // 1. Převedeme strukturu do formátovaného "pretty" JSON řetězce (hezky čitelný pro lidi)
    if let Ok(json) = serde_json::to_string_pretty(&cfg) {
        // 2. Zapíšeme výsledný JSON na disk
        // Poznámka: Používáme let _ pro potlačení varování o nevyužitém výsledku zápisu
        let _ = fs::write("nastaveni.json", json);
    }
}