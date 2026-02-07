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