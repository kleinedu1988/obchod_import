# DEVELOPERS.md – MRB Obchodník

Tento dokument je určen pro vývojáře, kteří aplikaci **udržují a rozšiřují**.
Popisuje architekturu, datové toky a konvence, které chrání stabilitu UI a konzistenci lokálních dat.

> `README.md` řeší **co to je** a **jak to spustit**.
> `DEVELOPERS.md` řeší **jak je to postavené** a **jak do toho bezpečně přispívat**.

---

## 1. Struktura projektu

Projekt je rozdělen do modulů pro lepší čitelnost a testovatelnost.

```text
/src
  main.rs          # Inicializace UI, handlery a propojení modulů
  models.rs        # Datové struktury (structs) a modely pro JSON
  config.rs        # I/O operace pro nastaveni.json
  logic.rs         # Výpočetní logika, filtrace, Excel a FS validace
/ui
  main.slint       # Root UI soubor
  detail.slint     # Komponenty importované v main.slint (dle verze)
build.rs           # Slint build pipeline
Cargo.toml         # Závislosti (slint, serde, chrono, rfd...)
partneri.json      # Lokální databáze (v .gitignore)
nastaveni.json     # Konfigurace (v .gitignore)
```

---

## 2. Slint build pipeline

Aplikace používá generování UI modulů pomocí `slint-build`.

### Pravidla

* `build.rs` **kompiluje pouze root UI soubor** (`ui/main.slint`)
* další `.slint` soubory se **importují v `main.slint`**
* **nikdy nekompilovat více root UI souborů**

Porušení těchto pravidel typicky vede k chybám:

* `cannot find type AppWindow`
* `PartnerData not found`

### Správná konfigurace

**build.rs**

```rust
fn main() {
    slint_build::compile("ui/main.slint").unwrap();
}
```

**ui/main.slint**

```slint
import { DetailWindow } from "detail.slint";
```

---

## 3. Architektura: UI ↔ Backend

### UI vrstva (Slint)

Soubor: `ui/main.slint`

UI je **deklarativní** a vystavuje pouze rozhraní (callbacky a vlastnosti). Neslouží k provádění logiky.

* `export component AppWindow`
* `export component ProgressWindow`
* `export struct PartnerData`

UI **neobsahuje business logiku**, pouze:

* layout
* styling
* lokální interakce (klik, focus)
* volání callbacků do Rustu

### Backend vrstva (Rust moduly)

* **main.rs** – „lepidlo“ aplikace

  * obsahuje `main()`
  * nastavuje callbacky z UI
  * spouští vlákna pro náročné operace

* **logic.rs** – veškeré „přemýšlení"

  * filtrace, třídění, statistiky
  * validace složek v archivu
  * práce se souborovým systémem
  * neví nic o existenci UI

* **config.rs** – izolace I/O pro konfiguraci

* **models.rs** – jediné místo pro definici datových struktur

---

## 4. Doménový model a persistence

### Datové soubory

* `partneri.json` – lokální databáze partnerů
* `nastaveni.json` – konfigurace cest a intervalů

> Tyto soubory jsou lokální a **musí být v `.gitignore`**.

### Datové struktury (models.rs)

* **Partner**

  * `id`
  * `nazev`
  * `slozka`
  * `aktualizovano`

* **Databaze**

  * seznam partnerů
  * `posledni_sync`

* **Config**

  * `cesta_archiv`
  * `cesta_vyroba`
  * `interval_synchronizace`

### UI projekce

* **PartnerData**

  * `ma_slozku` je **odvozená hodnota**

### Zásady importu

* import **nesmí přepsat `slozka`**, pokud byla ručně zadána
* `nazev` a `aktualizovano` se mění pouze při skutečné změně
* `posledni_sync` se nastaví po dokončení importu

---

## 5. Režimy aplikace a workflow

Aplikace rozlišuje procesní režimy přes `rezim_prace`:

* `0` – Rozcestník
* `1` – Poptávka
* `2` – Objednávka

UI z režimu odvozuje:

* akcent barvy
* název režimu

### Backend chování

Logika výběru dat pro jednotlivé režimy patří výhradně do `logic.rs`.

Aktuálně všechny režimy používají standardní synchronizaci partnerů, aby zůstala zachována stabilita aplikace.

---

## 6. Multithreading a UI bezpečnost

### Základní pravidlo

**Slint UI se nikdy nesmí měnit z background threadu.**

### Doporučený tok

1. uživatel klikne na tlačítko (handler v `main.rs`)
2. `main.rs` spustí `thread::spawn`
3. vlákno zavolá funkci z `logic.rs`
4. výsledek se vrátí do UI přes `slint::invoke_from_event_loop`

### Bezpečný vzor

```rust
let Some(ui) = weak.upgrade() else { return; };
```

---

## 7. Validace složek v archivu

Validace je součástí přípravy dat v `logic.rs`.

Cesta se skládá dynamicky:

```rust
Path::new(&config.cesta_archiv).join(&partner.slozka)
```

UI model dostává pouze výsledek:

```text
PartnerData.ma_slozku = bool
```

---

## 8. Filtrace a vyhledávání

Logika filtrace je implementována v `logic::priprav_data_partneru`.

### Vstup

* reference na konfiguraci
* index filtru
* hledaný text

### Výstup

* vyfiltrovaný a seřazený seznam partnerů
* vypočtené statistiky

---

## 9. Inline editace složek

Změna názvu složky:

* probíhá v UI
* ukládá se callbackem do backendu
* aktualizuje `partneri.json`
* znovu se načítá model

Doporučení:

* vždy `trim()` vstup
* validovat zakázané znaky

---

## 10. Status databáze

Funkce `logic::zkontroluj_stav_db` vrací stavový kód:

* `0` – OK (zelená)
* `1` – chyba / nenačteno (červená)
* `2` – neaktuální (oranžová)

---

## 11. Konvence a doporučení

### Modularita

* nové výpočetní funkce → `logic.rs`
* nové datové parametry → `models.rs`
* `main.rs` pouze UI glue kód

### Bezpečnost

* UI měnit pouze přes `invoke_from_event_loop`
* při práci s UI z vláken vždy používat `weak.upgrade()`
* I/O operace izolovat do `config.rs`

---

## 12. Troubleshooting

### „Změnil jsem strukturu Partnera a aplikace padá"

* zkontroluj shodu `models.rs` ↔ `PartnerData` v `main.slint`

### „Data v tabulce se neaktualizují"

* ověř, že `ui.set_model_partneru(...)` je voláno uvnitř `invoke_from_event_loop`

---

Tento dokument reflektuje stav po refaktoringu na **verzi 0.2.0 (modulární architektura)**.
