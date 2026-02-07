## [0.3.0] - 2026-02-07
### Přidáno
- **Jádro importu:** Implementována kompletní logika pro čtení Excel souborů (`.xlsx`) pomocí knihovny `calamine`.
- **Databázový systém:** Aplikace nyní vytváří a spravuje soubor `partneri.json`.
  - Inteligentní slučování: Noví partneři se přidají, existujícím se aktualizuje název, ale *složka zůstává zachována*.
  - Globální časová značka poslední synchronizace.
- **Víceokenní rozhraní:** Přidáno samostatné vyskakovací okno (`ProgressWindow`), které zobrazuje průběh importu.
- **Multithreading:** Import běží na pozadí v samostatném vlákně, takže hlavní okno nezamrzá.

### Změněno
- **Architektura UI:** Rozdělení `main.slint` na dvě samostatná okna (`AppWindow` a `ProgressWindow`).
- **Čištění kódu:** Odstraněna nepotřebná varování v Rustu a optimalizace importů.

### Opraveno
- **Overlay problém:** Progress bar se nyní zobrazuje korektně jako samostatné okno, nikoliv jako vrstva uvnitř hlavního layoutu.

## [0.2.1] - 2026-02-07
### Přidáno
- Implementován callback `spustit_synchronizaci` připravený pro logiku importu Excelu.

### Změněno
- **Vyladění UI:** Odstranění nadbytečných nadpisů a fixních mezer v záložce Synchronizace pro čistší vzhled.
- **Hierarchie:** Horní přepínače záložek jsou nyní zarovnány k hornímu okraji, což zvětšuje pracovní prostor.

### Opraveno
- **Fixace tlačítek:** Vyřešen kritický problém s layoutem. Tlačítka "ULOŽIT KONFIGURACI" a "NAHRÁT EXCEL A SPUSTIT" jsou nyní nekompromisně ukotvena u spodního okraje okna ve všech rozlišeních.
- **Syntaktické opravy:** Odstraněny nepodporované vlastnosti u komponenty `LineEdit`, které bránily kompilaci.

## [0.2.0] - 2026-02-07
### Přidáno
- **Trvalé ukládání nastavení:** Aplikace si nyní pamatuje cesty k archivu a výrobě i po restartu (využívá `serde` a `nastaveni.json`).
- **Nativní dialogy:** Implementován výběr složek pomocí systémového okna (knihovna `rfd`).
- **Automatická verze:** Číslo verze v UI se nyní načítá automaticky z `Cargo.toml`.

### Změněno
- **Struktura UI:** Zjednodušení projektu – všechny komponenty sloučeny zpět do `ui/main.slint` pro vyšší stabilitu.
- **Levý panel:** Vylepšeno zarovnání prvků, číslo verze je nyní fixováno na dně panelu pomocí pružiny (`VerticalLayout`).

### Odstraněno
- Smazány nadbytečné soubory `home.slint`, `settings.slint` a `components.slint`.

## [0.1.0] - 2026-02-05
- První funkční verze v Rustu.
- Základní okno aplikace.