# 🦀 Rust Esimerkkikokoelma

Tämä repositorio sisältää erilaisia Rust-ohjelmointikielen esimerkkejä suomeksi kommentoituna. Esimerkit on suunniteltu opettamaan Rustin peruskäsitteitä ja edistyneempiä tekniikoita.

## 📁 Sisältö

### Perusteet
- **`hei_maailma.rs`** - Yksinkertainen "Hei maailma" ohjelma
  - Perus println! makro
  - Muuttujat ja funktiot
  - Parametrit ja paluuarvot

### Tietorakenteet
- **`rust_esimerkki.rs`** - Henkilöt ja tilastot
  - Strukturit (struct) ja metodit
  - Vektorit ja HashMap
  - Iterator-metodit (map, sum, max_by_key)
  - Borrow checker ja viittaukset

### Enumit
- **`enum_esimerkki.rs`** - Enum-käyttötapaukset
  - Yksinkertaiset enumit
  - Enumit datalla
  - Pattern matching
  - Option ja Result enumit

### Tilakoneet (State Machines)
- **`tilakone_esimerkki.rs`** - Käytännölliset tilakone-esimerkit
  - Liikennevalo-simulaatio
  - Juoma-automaatti
  - Tapahtumapohjainen tilanhallinta
  - Virheenkäsittely tilakoneiden kanssa

## 🚀 Käyttö

### Vaatimukset
- Rust 1.70+ (testattu versiolla 1.89.0)
- Windows/Linux/macOS

### Ohjelmien kääntäminen ja suoritus

```bash
# Käännä ohjelma
rustc esimerkin_nimi.rs

# Suorita (Windows)
.\esimerkin_nimi.exe

# Suorita (Linux/macOS)
./esimerkin_nimi
```

### Esimerkki käytöstä

```bash
# Käännä ja suorita enum-esimerkki
rustc enum_esimerkki.rs
.\enum_esimerkki.exe
```

## 📚 Opitut käsitteet

### Rust-perusteet
- ✅ Muuttujat ja tietotyypit
- ✅ Funktiot ja parametrit
- ✅ Pattern matching
- ✅ Error handling

### Tietorakenteet
- ✅ Structs ja impl-lohkot
- ✅ Enums ja niiden sovellukset
- ✅ Vektorit ja HashMaps
- ✅ Iterator-metodit

### Edistyneet aiheet
- ✅ Ownership ja borrowing
- ✅ Tilakoneet (State machines)
- ✅ Generic types
- ✅ Debug ja Display traits

## 🎯 Seuraavat askelet

Jos haluat oppia lisää Rustia, suosittelen tutustumaan:

1. **[The Rust Book](https://doc.rust-lang.org/book/)** - Virallinen Rust-opas
2. **[Rust by Example](https://doc.rust-lang.org/rust-by-example/)** - Käytännön esimerkkejä
3. **[Rustlings](https://github.com/rust-lang/rustlings/)** - Interaktiiviset harjoitukset
4. **Cargo projektit** - Isompien projektien hallinta

## 🐛 Virheenkorjaus

Jos kohtaat ongelmia:

1. Varmista, että Rust on asennettu: `rustc --version`
2. Tarkista, että tiedostopolut ovat oikein
3. Katso virheviestit huolellisesti - Rustin virheviestit ovat yleensä hyvin informatiivisia

## 📝 Lisenssit

Nämä esimerkit on tarkoitettu opetuskäyttöön ja ne ovat vapaasti käytettävissä.

---

Tehty ❤️:lla Rust-yhteisölle! 🦀

*Huom: Esimerkit sisältävät suomenkielisiä kommentteja ja muuttujanimiä opetuksen helpottamiseksi.*
