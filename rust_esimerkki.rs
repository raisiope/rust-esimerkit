// Monimutkaisempi Rust-esimerkki
use std::collections::HashMap;

// Määritellään struct (rakenne)
#[derive(Debug)]
struct Henkilö {
    nimi: String,
    ikä: u32,
    kaupunki: String,
}

impl Henkilö {
    // Konstruktori
    fn uusi(nimi: String, ikä: u32, kaupunki: String) -> Henkilö {
        Henkilö { nimi, ikä, kaupunki }
    }
    
    // Metodi
    fn esittele(&self) {
        println!("Hei! Olen {} ja asun {}:ssa. Olen {} vuotta vanha.", 
                self.nimi, self.kaupunki, self.ikä);
    }
}

fn main() {
    println!("=== Rust-esimerkki: Henkilöt ja tilastot ===\n");
    
    // Luodaan vektori henkilöistä
    let mut henkilöt = Vec::new();
    
    henkilöt.push(Henkilö::uusi("Matti".to_string(), 30, "Helsinki".to_string()));
    henkilöt.push(Henkilö::uusi("Liisa".to_string(), 25, "Tampere".to_string()));
    henkilöt.push(Henkilö::uusi("Jukka".to_string(), 35, "Turku".to_string()));
    henkilöt.push(Henkilö::uusi("Anna".to_string(), 28, "Helsinki".to_string()));
    
    // Tulostetaan kaikki henkilöt
    println!("Henkilöt:");
    for (i, henkilö) in henkilöt.iter().enumerate() {
        println!("{}. {:?}", i + 1, henkilö);
    }
    
    println!("\n--- Esittelyt ---");
    for henkilö in &henkilöt {
        henkilö.esittele();
    }
    
    // Lasketaan tilastoja
    let keskiarvo_ikä = henkilöt.iter()
        .map(|h| h.ikä as f64)
        .sum::<f64>() / henkilöt.len() as f64;
    
    println!("\n--- Tilastot ---");
    println!("Henkilöitä yhteensä: {}", henkilöt.len());
    println!("Keski-ikä: {:.1} vuotta", keskiarvo_ikä);
    
    // Lasketaan kaupunkien jakauma
    let mut kaupunki_jakauma = HashMap::new();
    for henkilö in &henkilöt {
        *kaupunki_jakauma.entry(&henkilö.kaupunki).or_insert(0) += 1;
    }
    
    println!("\nKaupunkien jakauma:");
    for (kaupunki, määrä) in kaupunki_jakauma {
        println!("  {}: {} henkilö(ä)", kaupunki, määrä);
    }
    
    // Etsitään vanhin henkilö
    if let Some(vanhin) = henkilöt.iter().max_by_key(|h| h.ikä) {
        println!("\nVanhin henkilö: {} ({} vuotta)", vanhin.nimi, vanhin.ikä);
    }
}
