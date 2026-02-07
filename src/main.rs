// Magické makro, které propojí .slint soubor s Rustem
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // --- AUTOMATICKÉ NAČTENÍ VERZE Z CARGO.TOML ---
    // env!("CARGO_PKG_VERSION") vytáhne to číslo (např. "0.1.1") přímo při kompilaci
    let verze = env!("CARGO_PKG_VERSION");
    ui.set_verze_aplikace(verze.into()); // Pošleme to do Slintu

    let ui_weak = ui.as_weak();

    // --- LOGIKA PRO VÝBĚR ARCHIVU ---
    let ui_copy = ui_weak.clone();
    ui.on_vybrat_archiv(move || {
        // Otevřeme nativní dialog pro výběr složky
        if let Some(folder) = rfd::FileDialog::new()
            .set_title("Vyberte složku s archivem dat")
            .pick_folder() 
        {
            // Pokud uživatel složku vybral, získáme cestu jako text
            let cesta = folder.to_string_lossy().to_string();
            
            // "Rozbalíme" okno ze slabého odkazu a pošleme do něj cestu
            if let Some(ui) = ui_copy.upgrade() {
                ui.set_cesta_archiv(cesta.into());
            }
        }
    });

    // --- LOGIKA PRO VÝBĚR VÝROBY ---
    let ui_copy = ui_weak.clone();
    ui.on_vybrat_vyrobu(move || {
        if let Some(folder) = rfd::FileDialog::new()
            .set_title("Vyberte cílovou složku pro výrobu")
            .pick_folder() 
        {
            let cesta = folder.to_string_lossy().to_string();
            if let Some(ui) = ui_copy.upgrade() {
                ui.set_cesta_vyroba(cesta.into());
            }
        }
    });

    // Spuštění aplikace
    ui.run()
}