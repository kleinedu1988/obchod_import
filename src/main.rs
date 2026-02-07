use serde::{Deserialize, Serialize};
use std::fs;
use slint::ComponentHandle; // Důležité pro práci s oknem

slint::include_modules!();

// Definujeme strukturu dat pro JSON soubor
// "Derive" znamená, že Rust automaticky pochopí, jak to převést na text
#[derive(Serialize, Deserialize)]
struct Nastaveni {
    archiv: String,
    vyroba: String,
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_weak = ui.as_weak();

    // 1. NAČTENÍ VERZE APLIKACE (z Cargo.toml)
    let verze = env!("CARGO_PKG_VERSION");
    ui.set_verze_aplikace(verze.into());

    // 2. NAČTENÍ ULOŽENÉHO NASTAVENÍ PŘI STARTU
    // Zkusíme najít soubor "nastaveni.json"
    if let Ok(obsah_souboru) = fs::read_to_string("nastaveni.json") {
        // Pokud existuje, převedeme ho zpět na data
        if let Ok(nactena_data) = serde_json::from_str::<Nastaveni>(&obsah_souboru) {
            println!("Načítám nastavení: Archiv={}, Výroba={}", nactena_data.archiv, nactena_data.vyroba);
            ui.set_cesta_archiv(nactena_data.archiv.into());
            ui.set_cesta_vyroba(nactena_data.vyroba.into());
        }
    } else {
        println!("Soubor nastaveni.json nenalezen, začínáme s čistým štítem.");
    }

    // 3. LOGIKA PRO TLAČÍTKA "PROCHÁZET"
    let ui_arch = ui_weak.clone();
    ui.on_vybrat_archiv(move || {
        let ui = ui_arch.unwrap();
        if let Some(folder) = rfd::FileDialog::new().set_title("Vyber složku s archivem").pick_folder() {
            ui.set_cesta_archiv(folder.to_string_lossy().to_string().into());
        }
    });

    let ui_vyr = ui_weak.clone();
    ui.on_vybrat_vyrobu(move || {
        let ui = ui_vyr.unwrap();
        if let Some(folder) = rfd::FileDialog::new().set_title("Vyber složku pro výrobu").pick_folder() {
            ui.set_cesta_vyroba(folder.to_string_lossy().to_string().into());
        }
    });

    // 4. LOGIKA PRO TLAČÍTKO "ULOŽIT"
    let ui_save = ui_weak.clone();
    ui.on_ulozit_nastaveni(move || {
        let ui = ui_save.unwrap();
        
        // Vytáhneme aktuální text z kolonek v okně
        let data = Nastaveni {
            archiv: ui.get_cesta_archiv().to_string(),
            vyroba: ui.get_cesta_vyroba().to_string(),
        };

        // Převedeme data na hezký JSON text
        match serde_json::to_string_pretty(&data) {
            Ok(json_text) => {
                // Zapíšeme do souboru
                if let Err(e) = fs::write("nastaveni.json", json_text) {
                    eprintln!("Chyba při zápisu souboru: {}", e);
                } else {
                    println!("Nastavení úspěšně uloženo!");
                }
            },
            Err(e) => eprintln!("Chyba při převodu dat: {}", e),
        }
    });

    ui.run()
}