fn main() {
    // Toto řekne Rustu: "Vezmi main.slint a udělej z něj Rust kód"
    slint_build::compile("ui/main.slint").expect("Chyba při kompilaci UI");
}