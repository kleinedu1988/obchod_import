// Magické makro, které vygeneruje Rust kód z tvého .slint souboru
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // Vytvoření instance okna
    let ui = AppWindow::new()?;

    // Spuštění aplikace
    ui.run()
}