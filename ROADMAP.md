# ROADMAP.md – MRB Obchodník

Tento dokument popisuje **plánovaný vývoj aplikace MRB Obchodník**.
Slouží jako orientační mapa pro vývojáře, maintainery a zadavatele.

> Roadmapa není závazný kontrakt.  
> Je to **živý dokument**, který se může měnit podle priorit a zpětné vazby.

---

## 🟢 Aktuální stav (baseline)

- stabilní UI postavené na **Slint + Rust**
- lokální persistence přes JSON
- import partnerů z Excelu
- validace existence složek v archivu
- inline editace názvů složek
- základní filtrace a statistiky

Tato verze je považována za **produkčně použitelnou**.

---

## 🟡 Krátkodobé cíle (v1.x)

Zaměření: **stabilita, použitelnost, čitelnost chyb**

### UI & UX
- [ ] lepší error hlášky v UI (dialogy místo `println!`)
- [ ] jasnější stavové zprávy během importu
- [ ] sjednocení textů a popisků (terminologie)

### Import & data
- [ ] parser režimu **Objednávka**
  - vstup: `Transformatorek_MRB_rozsireny.xlsx`
- [ ] bezpečnější práce s prázdnými / poškozenými řádky v Excelu
- [ ] detailnější logování chyb importu

### Kód & údržba
- [ ] refaktor importního kódu (oddělení parsování a mapování)
- [ ] omezení opakovaného čtení `partneri.json`
- [ ] sjednocení naming conventions (UI ↔ Rust)

---

## 🟠 Střednědobé cíle (v2.x)

Zaměření: **rozšiřitelnost a výkon**

### Nové workflow
- [ ] parser režimu **Poptávka**
  - standardizovaný formát vstupu
  - validace povinných sloupců
- [ ] rozšíření režimů bez rozbití stávající logiky

### Data & výkon
- [ ] cache `partneri.json` v paměti
- [ ] zapisovat pouze diff (ne celý soubor)
- [ ] batch / debounce ukládání při inline editaci

### UI funkce
- [ ] volitelná automatická tvorba složek v archivu
  - checkbox ve formuláři
- [ ] lepší vizuální indikace chyb (ikonky, barvy, tooltipy)
- [ ] rozšířená filtrace (kombinace filtr + search)

---

## 🔵 Dlouhodobé cíle (v3.x+)

Zaměření: **komfort, profesionalizace, škálování**

### Architektura
- [ ] jasné oddělení doménové logiky od UI vrstvy
- [ ] příprava na alternativní persistence (SQLite?)
- [ ] lepší testovatelnost importních modulů

### Uživatelé & provoz
- [ ] historie změn partnera (audit log)
- [ ] možnost revertu posledního importu
- [ ] export statistik (CSV / Excel)

### Vývojářský komfort
- [ ] lepší struktura dokumentace (`ARCHITECTURE.md`, `DOMAIN.md`)
- [ ] ADR záznamy pro klíčová rozhodnutí
- [ ] základní test coverage kritických částí

---

## 🔴 Mimo rozsah (zatím neplánováno)

Tyto body **nejsou cílem projektu**, pokud se zásadně nezmění požadavky:

- síťová synchronizace / server backend
- multi-user prostředí
- komplexní role a oprávnění
- realtime kolaborace

---

## 📌 Poznámky k prioritám

- **Stabilita má vždy přednost před novými funkcemi**
- Import dat je považován za **kritickou část systému**
- UI nesmí zamrzat – multithreading je nedotknutelné pravidlo
- Každý nový režim musí:
  - zachovat kompatibilitu se stávajícími daty
  - respektovat existující workflow

---

## 🧭 Jak roadmapu aktualizovat

- hotové položky přesunout do `CHANGELOG.md`
- větší rozhodnutí zaznamenat do `ADR/`
- roadmapu aktualizovat **při každém minor release**