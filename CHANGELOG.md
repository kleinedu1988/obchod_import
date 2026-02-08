## [0.3.4] – 2026-02-08
### Přidáno
- **Kontrola aktuálnosti dat:**  
  Implementována logika hlídání stáří databáze `partneri.json`. Stav databáze je nyní vyhodnocován vůči uživatelsky nastavenému intervalu.

- **Konfigurovatelný interval kontroly:**  
  V záložce **Obecné** přibyla možnost nastavit interval kontroly aktuálnosti databáze  
  *(1 týden až 6 měsíců + testovací režim).*

- **Nový stav „Neaktuální databáze“:**  
  Přidán oranžový vizuální indikátor pro stav **Databáze je neaktuální** v sekci **Aktualizace dat**.

### Změněno
- **Reorganizace nastavení (UX):**  
  Přehlednější struktura a logičtější pojmenování záložek:
  - **Aktualizace dat** *(původně Synchronizace)*
  - **Databáze partnerů** *(původně Správce dat)*
  - **Obecné** *(původně Cesty)*

- **Chování navigace:**  
  Při vstupu do nastavení se aplikace nyní automaticky přepne na výchozí záložku  
  **Aktualizace dat**.

### Opraveno
- **Čištění Rust kódu:**  
  Odstraněn nepoužívaný import `SharedString`, čímž byl vyčištěn výstup kompilátoru a odstraněna zbytečná varování.

## [0.3.3] - 2026-02-07
### Opraveno
- **Struktura layoutu:** Kompletní rekonstrukce pravého panelu. Odstraněno nesprávné vnořování do statických kontejnerů, které způsobovalo kolaps `ListView`.
- **UI Scaling:** Implementována správná kaskáda `vertical-stretch`. Seznam partnerů se nyní dynamicky roztahuje přes celou dostupnou plochu okna.
- **Syntaktické opravy:** Paddingy u vyhledávání a tabulky byly přepsány z neplatného CSS formátu na nativní Slint syntaxi (`padding-left`, `padding-right`), což vyřešilo chyby při kompilaci.

## [0.3.2] - 2026-02-07
### Opraveno
- **Logika statistik:** Opraven výpočet "Chybějící složka" v Rustu. Nyní se dynamicky vypočítává jako `Celkový počet - Počet přiřazených`, což zajišťuje přesnost i při 14 000+ záznamech.
- **UI Dashboard:** Navrácen `HorizontalLayout` pro karty statistik ve Správci dat (ikona vlevo, čísla vpravo) pro lepší čitelnost.

### Změněno
- **Data Binding:** Plné propojení statistik v UI (`AppWindow`) s backendovou logikou. Čísla se aktualizují okamžitě po načtení dat nebo importu.

## [0.3.1] - 2026-02-07
### Změněno
- **Optimalizace renderování:** Přechod z `ScrollView` na `ListView` (UI Virtualizace). Aplikace nyní vykresluje pouze viditelné řádky, což umožňuje plynulý posun i při 14 000+ záznamech.
- **Asynchronní načítání:** Načítání a parsování `partneri.json` přesunuto do samostatného vlákna. Start aplikace je okamžitý a GUI nezamrzá.

### Opraveno
- **Critical Fix:** Opraveno zamrzání aplikace (Application Not Responding) při práci s velkým množstvím dat.
- **UI Layout:** Opraveno zarovnání stavových indikátorů (puntíků) v tabulce pomocí vnořených layoutů.

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