## [0.4.2] - 2026-02-08
### ✨ Přidáno
- **Zásobník příloh (Attachment Backpack):** Implementována sekundární zóna v pravém panelu pro shromažďování technické dokumentace (PDF, STEP, DXF) před zpracováním.
- **Multiselect:** Přidána podpora pro výběr více souborů najednou v systémovém dialogu.
- **Správa souborů:** Vizualizace seznamu příloh s možností odebrání (červený křížek) před finálním odesláním.
- **Kopírovací logika:** Backend (Rust) nyní při synchronizaci zkopíruje nejen data z Excelu, ale i všechny soubory ze zásobníku do cílové složky.

### ⚙️ Změněno
- **Layout Importu:** Pravý panel rozdělen na dvě nezávislé sekce (Řídící Excel nahoře, Přílohy dole).
- **Univerzální DropZone:** Komponenta DropZone upravena pro obecné použití (Excel i Přílohy).
- **Status Bar:** Rozšířena zpětná vazba o informace týkající se kopírování souborů.

### 🛠️ Opraveno
- **Layout Stretch:** Opraveno chování pružiny (vertical-stretch) v pravém panelu, zásobník nyní správně vyplňuje volné místo.
- **UI detaily:** Vylepšena typografie a hover efekty u mazacích tlačítek v seznamu souborů.

## [0.4.1] – 2026-02-08
### Přidáno
- **Zásobník příloh (Attachment Backpack)**  
  Nová sekce v pravém panelu umožňující shromažďovat technickou dokumentaci (PDF, STEP, DXF, DWG) před samotným zpracováním zakázky.

- **Multiselect souborů**  
  Podpora hromadného výběru datových souborů v systémovém dialogu.

- **Správa souborů v UI**  
  Implementována komponenta `FileItem`, umožňující vizuální kontrolu seznamu příloh a jejich selektivní odebírání (červený křížek) před odesláním.

- **Automatické kopírování příloh**  
  Přidána Rust logika pro fyzické kopírování všech příloh ze zásobníku do cílové složky `Vystup_Data` (nebo do složky partnera) během procesu synchronizace.

### Změněno
- **Univerzální DropZone**  
  Komponenta `DropZone` byla upravena na univerzální stavební prvek s dynamickou ikonou a texty, což umožňuje její opakované použití pro Excel i datové přílohy.

- **Redesign pravého panelu**  
  Pravý sloupec je nově vertikálně rozdělen na:
  - sekci 1: Řídící Excel
  - sekci 2: Datové přílohy  
  Každá sekce má vlastní rolovací oblast.

- **UI feedback v průběhu synchronizace**  
  Status bar v okně průběhu nyní zobrazuje také počet kopírovaných příloh.

### Opraveno
- **Layout stretch v pravém panelu**  
  Opraveno chování vertikálního roztahování (odstranění omezujícího zarovnání), díky čemuž se zásobník příloh dynamicky přizpůsobuje výšce okna.

- **Typografie mazacího tlačítka**  
  Zjemněn vizuální styl červeného křížku pro odebrání souboru, včetně interaktivního hover efektu.

## [0.4.0] – 2026-02-08
### Přidáno
- **Hub (Rozcestník):**  
  Nová domovská obrazovka umožňující okamžitou volbu mezi moduly **Poptávka** a **Objednávka** pomocí vizuálních karet.  
  Zrychluje zahájení práce a usnadňuje orientaci bez nutnosti procházet menu.

- **Pracovní režimy:**  
  Zavedení barevného rozlišení procesů – **oranžová pro Poptávky**, **zelená pro Objednávky**.  
  Tato vizuální kotva minimalizuje riziko záměny kontextu a poskytuje okamžitý přehled o aktuální fázi zpracování.

- **Hybridní formulář:**  
  Redesign importní stránky na dvousloupcový layout.  
  Levá část obsahuje formulář pro ruční zadání údajů, pravá slouží jako **DropZóna**.  
  Umožňuje plynulé kombinování obou způsobů zadávání dat na jedné pracovní ploše.

- **Manuální zakládání dokladů:**  
  Přidána logika pro založení dokladu přímo z ručně vyplněných polí.  
  Zrychluje práci v případech, kdy není potřeba import externího souboru.

### Změněno
- **Navigační schéma:**  
  Systémová tlačítka byla přebarvena na neutrální modrou.  
  Tím se eliminuje kolize se zeleným indikátorem objednávek a vzniká jasnější hierarchie mezi navigací a pracovním procesem.

- **Dynamický sidebar:**  
  Tlačítko aktivního procesu v postranním panelu nyní v reálném čase mění text i barvu podle zvoleného režimu, čímž sjednocuje uživatelský zážitek.

- **Stabilizace okna aplikace:**  
  Nastavena preferovaná velikost okna na **1200 × 800 px**.  
  Změna zabraňuje „přeblikávání“ a nechtěnému zmenšování okna při přechodu na stránky s menším množstvím prvků.

### Opraveno
- **Stabilita kompilace:**  
  Odstraněna nekompatibilní animace `scale` u komponenty `Rectangle`, která způsobovala chyby při sestavování.  
  Kód je nyní plně validní vůči aktuálním standardům frameworku **Slint**.

- **Layout DropZóny:**  
  Definována pevná minimální výška pro **DropZónu**.  
  Oprava zabraňuje rušivým změnám výšky okolních prvků při přepínání mezi formulářem a importem.

## [0.3.6] – 2026-02-08
### Přidáno
- **Nezávislé statistiky:**  
  Přidána vlastnost `pocet_celkem`, díky které zůstává celkový počet partnerů na dashboardu konstantní i při aktivním vyhledávání nebo filtraci dat.

### Změněno
- **Redesign vstupních polí:**  
  Komponenty `LineEdit` pro vyhledávání a zápis složky byly nahrazeny nízkoúrovňovým `TextInput`.  
  Tím byl odstraněn rušivý systémový focus border (modrý pruh) a vstupní pole nyní plně splývají s tmavým vizuálním stylem aplikace.

- **Logika navigace:**  
  Při kliknutí na **Nastavení systému** v levém panelu se záložky vždy resetují na výchozí **Aktualizace dat**.

- **Sjednocení vizuálu tabulky:**  
  Zarovnání textů v tabulce bylo sjednoceno. Záhlaví i obsah řádků jsou nyní striktně zarovnány doleva pro lepší čitelnost.

### Opraveno
- **Projektová hygiena (Rust):**  
  Odstraněna varování kompilátoru *(unused imports: `ModelRc`, `VecModel`, `Weak` a nadbytečná klíčová slova `mut`)*.  
  Výsledkem je zcela čistý průběh kompilace bez varování.

- **Lícování tabulky:**  
  Odstraněny drobné odchylky v odsazení sloupců (`spacing`), které způsobovaly nelícování obsahu řádků se záhlavím tabulky.

## [0.3.5] – 2026-02-08
### Přidáno
- **Inline editace složek:**  
  Název složky partnera lze nyní upravit přímo v tabulce pomocí textového pole.  
  Změna se uloží potvrzením klávesou **Enter**.

- **Validace existence složek:**  
  Implementována automatická kontrola, zda zadaný název složky fyzicky existuje v definované **cestě k Archivu zakázek**.

### Změněno
- **Logika stavových indikátorů:**  
  Stavový indikátor partnera (puntík) je nyní **zelený pouze tehdy**, pokud:
  - je vyplněn název složky **a zároveň**
  - tato složka skutečně existuje v Archivu zakázek.

- **Zpřesnění UI textace:**  
  Texty v tabulce i ve statistikách byly upraveny tak, aby jednoznačně deklarovaly, že se ověřuje **přítomnost složky v Archivu zakázek**.

### Opraveno
- **Projektová hygiena:**  
  Soubory `partneri.json` a `nastaveni.json` byly přidány do `.gitignore`, čímž se zabránilo nechtěnému verzování citlivých lokálních dat.

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