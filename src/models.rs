// =========================================================================
// MODUL: DATOVÉ MODELY (models.rs)
// =========================================================================
// Tento modul definuje strukturu dat, se kterými aplikace pracuje.
// Používá knihovnu Serde pro automatický převod mezi strukturami v Rustu
// a formátem JSON (pro ukládání do souborů).

use serde::{Deserialize, Serialize};

// -------------------------------------------------------------------------
// MODEL: PARTNER
// -------------------------------------------------------------------------

/// Reprezentuje jednoho obchodního partnera načteného z databáze.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Partner {
    /// Unikátní identifikátor (např. zkratka nebo kód)
    pub id: String,
    /// Celý název společnosti
    pub nazev: String,
    /// Název složky v archivu, která patří tomuto partnerovi
    pub slozka: String,
    /// Datum a čas, kdy byl záznam naposledy ověřen/aktualizován
    pub aktualizovano: String,
}

// -------------------------------------------------------------------------
// MODEL: KONFIGURACE (NASTAVENÍ)
// -------------------------------------------------------------------------

/// Reprezentuje globální nastavení aplikace ukládané do 'nastaveni.json'.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    /// Cesta k hlavnímu archivu dokumentů
    pub cesta_archiv: String,
    /// Cesta, kam se mají exportovat data pro výrobu
    pub cesta_vyroba: String,
    /// Textový popis intervalu (např. "1 týden", "14 dní")
    pub interval_synchronizace: String,
}

// Výchozí hodnoty pro konfiguraci (pokud soubor s nastavením neexistuje)
impl Default for Config {
    fn default() -> Self {
        Self {
            cesta_archiv: String::new(),
            cesta_vyroba: String::new(),
            // Přednastavený interval pro novou instalaci
            interval_synchronizace: "1 týden".to_string(),
        }
    }
}

// -------------------------------------------------------------------------
// MODEL: DATABÁZE (STRUKTURA JSON SOUBORU)
// -------------------------------------------------------------------------

/// Hlavní obal pro data uložená v souboru 'partneri.json'.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Databaze {
    /// Čas poslední úspěšné synchronizace s Excelem
    pub posledni_sync: String,
    /// Seznam všech partnerů v systému
    pub partneri: Vec<Partner>,
}