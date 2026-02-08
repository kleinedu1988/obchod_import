# DEVELOPERS.md – MRB Obchodník

Tento dokument je určen pro vývojáře, kteří aplikaci **udržují a rozšiřují**.
Popisuje architekturu, datové toky a konvence, které chrání stabilitu UI a konzistenci lokálních dat.

> `README.md` řeší **co to je** a **jak to spustit**.
> `DEVELOPERS.md` řeší **jak je to postavené** a **jak do toho bezpečně přispívat**.

---

## 1. Struktura projektu

Doporučená struktura repozitáře:

```text
/src
  main.rs
/ui
  main.slint
  detail.slint        (volitelné / dle verze)
build.rs
Cargo.toml
partneri.json         (lokální data – v .gitignore)
nastaveni.json        (lokální data – v .gitignore)
```

---

## 2. Slint build pipeline

Aplikace používá generování UI modulů pomocí `slint-build`.

### Pravidla

* `build.rs` **kompiluje pouze root UI soubor** (`ui/main.slint`)
* další `.slint` soubory se **importují v `main.slint`**
* **nikdy nekompilovat více root UI souborů**

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

Porušení těchto pravidel typicky vede k chybám:

* `cannot find type AppWindow`
* `PartnerData not found`

---

## 3. Architektura: UI ↔ Backend

### UI vrstva (Slint)

Soubor: `ui/main.slint`

UI je **deklarativní** a vystavuje pouze rozhraní:

* `export component AppWindow`
* `export component ProgressWindow`
* `export struct PartnerData`

UI **neobsahuje business logiku**, pouze:

* layout
* styling
* lokální interakce (klik, focus)
* volání callbacků do Rustu

### Backend vrstva (Rust)

Soubor: `src/main.rs`

Backend:

* načítá a ukládá lokální JSON soubory
* parsuje Excel pomocí `calamine`
* počítá statistiky
* provádí validace složek v archivu
* aktualizuje UI přes `slint::invoke_from_event_loop`

---

## 4. Doménový model a persistence

### Datové soubory

* `partneri.json` – lokální databáze partnerů
* `nastaveni.json` – konfigurace prostředí (cesty, intervaly)

> Tyto soubory jsou lokální a **musí být v `.gitignore`**.

### Datové struktury

**Rust**

* `Partner` – persistovaný záznam

  * `id`, `nazev`, `slozka`, `aktualizovano`

* `Databaze` – obálka nad kolekcí + `posledni_sync`

* `Config`

  * `cesta_archiv`
  * `cesta_vyroba`
  * `interval_synchronizace`

**Slint**

* `PartnerData` – UI projekce

  * `ma_slozku` je **odvozená hodnota** (viz validace archivu)

### Zásady importu / merge

* import **nesmí přepsat `slozka`**, pokud byla ručně zadána
* `nazev` a `aktualizovano` se aktualizují pouze při změně
* `posledni_sync` se nastaví po dokončení importu

---

## 5. Režimy aplikace a workflow

Aplikace používá procesní režimy přes `rezim_prace`:

* `0` – Rozcestník (Hub)
* `1` – Poptávka
* `2` – Objednávka

UI z režimu odvozuje:

* `akcent_barva` (oranžová / zelená / modrá)
* `nazev_rezimu`

### Důsledky pro backend

Callback `spustit_synchronizaci()` běží ve všech režimech:

* `rezim == 0` – aktualizace databáze
* `rezim == 1` – import podkladů pro poptávku (budoucí parser)
* `rezim == 2` – import podkladů pro objednávku (budoucí parser)

Aktuálně všechny režimy používají **standardní update partnerů**, aby zůstala zachována stabilita aplikace.

---

## 6. Multithreading a UI bezpečnost

### Základní pravidlo

**Slint UI se nikdy nesmí měnit z background threadu.**

* background thread:

  * parsování Excelu
  * práce se soubory
  * validace složek

* UI thread:

  * `set_*`
  * změny modelů

### Doporučený vzor

* dlouhá operace → `thread::spawn`
* update UI → `slint::invoke_from_event_loop`

### Progress okno

* `ProgressWindow` se zobrazí před spuštěním threadu
* průběh se aktualizuje periodicky (např. `idx % 10`)
* okno se skryje po dokončení operace

---

## 7. Validace složek v archivu

Pole `Partner.slozka` ukládá **pouze název podsložky**, nikoli absolutní cestu.

Validace probíhá při přípravě modelu pro UI:

Složka je platná, pokud:

* `slozka` není prázdná
* existuje fyzicky v `Path::new(config.cesta_archiv).join(slozka)`

Výsledek:

```text
PartnerData.ma_slozku = has_name && exists_in_archive
```

Indikátor v UI je zelený pouze při splnění obou podmínek.

---

## 8. Filtrace a vyhledávání

### Stavové proměnné

* `aktivni_filtr`

  * `0` – Celkem
  * `1` – Problém se složkou
  * `2` – Vyhledávání

* `search_text`

### Logika

* změna filtru resetuje `search_text` (pokud `filtr != 2`)
* vyhledávání automaticky přepíná filtr na `2`
* smazání vyhledávání vrací filtr na `0`

### Statistiky

* `pocet_celkem` – konstantní (nezávislé na filtrech)
* `pocet_chybi` – počítáno z **plného datasetu**, ne z filtrovaného

---

## 9. Inline editace složek

V tabulce je `TextInput` pro `partner.slozka`.

Uložení probíhá:

* při `accepted` (Enter)
* callback: `ulozit_nazev_slozky(partner.id, self.text)`

Backend:

* aktualizuje `partneri.json`
* nastaví `aktualizovano`
* znovu načte model (obnova tabulky)

### Doporučení

* nepoužívat `unwrap()` na `upgrade()` UI handle
* validovat `novy_nazev`:

  * `trim()`
  * zakázané znaky dle filesystemu

---

## 10. Status databáze

Funkce `aktualizuj_stav_db()`:

* načte `partneri.json`
* parsuje `posledni_sync` (`%d.%m.%Y %H:%M`)
* porovná s `interval_synchronizace`

Nastavuje:

* `db_status_code`

  * `0` – aktuální (zelená)
  * `2` – neaktuální (oranžová)
  * `1` – chyba / nenačteno (červená)

* `stav_text`

* `posledni_sync_cas`

---

## 11. Konvence a doporučení

### UI

* vlastní property: `snake_case` (`ma_slozku`, `model_partneru`)
* `-` je vyhrazen pro built-in property (`padding-left`, `horizontal-alignment`)

### Rust

* žádné UI mutace mimo `invoke_from_event_loop`
* preferovat bezpečný vzor:

```rust
let Some(ui) = weak.upgrade() else { return; };
```

* minimalizovat opakované čtení JSON (budoucí optimalizace)

### Výkon (budoucí zlepšení)

* debounce / batch inline editací
* cache `partneri.json` v paměti a zapisovat pouze diff

---

## 12. Troubleshooting

### `cannot find type AppWindow / PartnerData`

* špatně zapojená build pipeline
* `build.rs` musí kompilovat pouze root UI soubor
* `PartnerData` musí být `export struct`
* `detail.slint` se **importuje**, nekompiluje

### UI zamrzá při importu

* import musí běžet v `thread::spawn`
* UI update pouze přes `invoke_from_event_loop`

### Zelený stav se neukazuje

* ověř `cesta_archiv` v `nastaveni.json`
* `Partner.slozka` je pouze název podsložky
* validace používá `Path::join`

---

## 13. Roadmap (vývojářská)

* parser režimu **Objednávka** (`Transformatorek_MRB_rozsireny.xlsx`)
* parser režimu **Poptávka** (standardizace vstupu)
* volitelná tvorba složek v archivu (checkbox)
* lepší error reporting (UI hlášky místo `println!`)

---

## 14. Poznámky k aktuálnímu kódu

### 1) Konvence názvů property v UI

Chybné:

```slint
partner.ma-slozku
```

Správně:

```slint
partner.ma_slozku
```

### 2) Bezpečné čtení Excel buněk

Chybné:

```rust
let id = row[0].to_string();
let nazev = row[1].to_string();
```

Správné:

```rust
let id = row.get(0).map(|c| c.to_string()).unwrap_or_default();
let nazev = row.get(1).map(|c| c.to_string()).unwrap_or_default();
```
