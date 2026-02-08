# MRB Obchodník 🚀

![Version](https://img.shields.io/badge/version-0.4.0-blue.svg)
![Rust](https://img.shields.io/badge/built_with-Rust-orange.svg)
![UI](https://img.shields.io/badge/UI-Slint-green.svg)

**MRB Obchodník** je moderní desktopová aplikace pro správu a zpracování obchodních dat.  
Slouží k efektivnímu importu podkladů z Excelu, správě databáze partnerů a řízenému převodu dat do výrobních a archivačních struktur.

Od verze **0.4.x** je aplikace postavena na **procesním workflow** s jasně oddělenými pracovními režimy a hybridním způsobem zadávání dat.

---

## 📸 Náhled aplikace
![Hlavní obrazovka aplikace](assets/screen.png)

---

## 🌟 Klíčové vlastnosti

- **Procesní workflow (v0.4+)**
  - Domovský **Hub (Rozcestník)** pro okamžitou volbu režimu
  - Oddělené pracovní kontexty:
    - **Poptávka** – příprava, kalkulace, podklady
    - **Objednávka** – validace a převod do výroby
  - Barevné rozlišení režimů pro minimalizaci chyb

- **Hybridní zadávání dat**
  - Kombinace ručního formuláře a DropZóny v jednom workspace
  - Možnost založení dokladu i bez importu externího souboru

- **Rychlý import dat**
  - Využití Rustu a knihovny `calamine` pro bleskové zpracování Excel souborů  
    (`.xlsx`, `.xlsm`)
  - Import tisíců řádků během zlomku sekundy

- **Chytrá synchronizace partnerů**
  - Automatická identifikace podle ID / IČO
  - Aktualizace názvů a časových značek
  - **Zachování uživatelských cest ke složkám** (nepřepisují se)

- **Validace archivní struktury**
  - Ověření fyzické existence složek v Archivu zakázek
  - Okamžitá vizuální indikace chybějících nebo neplatných cest

- **Moderní UI**
  - Postaveno na frameworku **Slint**
  - Tmavý režim (Dark Mode)
  - Virtualizovaný seznam (`ListView`) umožňující práci s 14 000+ záznamy
  - Samostatné okno průběhu importu (Progress Window)

- **Multithreading**
  - Import a zpracování dat běží na pozadí
  - GUI zůstává plně responzivní bez zamrzání

- **Lokální persistence**
  - Nastavení i databáze ukládány do JSON souborů
  - Bez závislosti na externím backendu nebo připojení k internetu

---

## 🛠️ Použité technologie

- **Jazyk:** [Rust](https://www.rust-lang.org/) 🦀
- **GUI:** [Slint](https://slint.dev/)

### Knihovny
- `serde`, `serde_json` – práce s JSON daty
- `calamine` – čtení Excel souborů
- `chrono` – časová razítka a synchronizace
- `rfd` – nativní systémové dialogy

---

## 🚀 Jak spustit projekt

### Prerekvizity
- Nainstalovaný **Rust** a **Cargo** (≥ 1.70)
- Nainstalovaný **Git**
- VS Code (doporučeno) s rozšířením **Slint**

### Instalace a spuštění

```bash
git clone https://github.com/TVUJ-UZIVATEL/mrb-obchodnik.git
cd mrb-obchodnik
cargo run --release