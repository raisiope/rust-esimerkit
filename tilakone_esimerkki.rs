// Tilakone-esimerkit Rust enumeilla

use std::collections::VecDeque;

// Yksinkertainen liikennevalo-tilakone
#[derive(Debug, Clone, PartialEq)]
enum LiikennevalonTila {
    Punainen,
    Keltainen,
    Vihreä,
    VilkkuvaKeltainen,
}

struct Liikennevalo {
    nykyinen_tila: LiikennevalonTila,
    ajastin: u32,
}

impl Liikennevalo {
    fn uusi() -> Self {
        Liikennevalo {
            nykyinen_tila: LiikennevalonTila::Punainen,
            ajastin: 0,
        }
    }
    
    fn päivitä(&mut self) {
        self.ajastin += 1;
        
        match (&self.nykyinen_tila, self.ajastin) {
            (LiikennevalonTila::Punainen, 5) => {
                self.nykyinen_tila = LiikennevalonTila::Vihreä;
                self.ajastin = 0;
            }
            (LiikennevalonTila::Vihreä, 8) => {
                self.nykyinen_tila = LiikennevalonTila::Keltainen;
                self.ajastin = 0;
            }
            (LiikennevalonTila::Keltainen, 2) => {
                self.nykyinen_tila = LiikennevalonTila::Punainen;
                self.ajastin = 0;
            }
            (LiikennevalonTila::VilkkuvaKeltainen, 1) => {
                self.nykyinen_tila = LiikennevalonTila::Punainen;
                self.ajastin = 0;
            }
            _ => {} // Odotetaan
        }
    }
    
    fn huolto_tila(&mut self) {
        self.nykyinen_tila = LiikennevalonTila::VilkkuvaKeltainen;
        self.ajastin = 0;
    }
    
    fn tilan_kuvaus(&self) -> &str {
        match self.nykyinen_tila {
            LiikennevalonTila::Punainen => "🔴 SEIS - Älä kulje!",
            LiikennevalonTila::Keltainen => "🟡 VALMISTAUDU - Pysähdy jos mahdollista",
            LiikennevalonTila::Vihreä => "🟢 AJA - Tie on vapaa",
            LiikennevalonTila::VilkkuvaKeltainen => "🟡💫 HUOLTO - Varovainen liikenne",
        }
    }
}

// Monimutkaisempi tilakone: Automaatti (vending machine)
#[derive(Debug, Clone)]
enum AutomaatinTila {
    OdottaaRahaa { kolikot: u32 },
    TuoteValittu { kolikot: u32, tuote: String },
    AnnetaanVaihtorahaa { vaihtoraha: u32 },
    TuoteAnnetaan { tuote: String },
    Virhe { viesti: String },
}

#[derive(Debug)]
enum AutomaatinTapahtuma {
    LisääKolikko(u32),
    ValitseTuote(String, u32), // tuote, hinta
    OtaVaihtoraha,
    OtaTuote,
    Nollaa,
}

struct Automaatti {
    tila: AutomaatinTila,
    tuotteet: std::collections::HashMap<String, u32>,
}

impl Automaatti {
    fn uusi() -> Self {
        let mut tuotteet = std::collections::HashMap::new();
        tuotteet.insert("Sokeri".to_string(), 150);
        tuotteet.insert("Kahvi".to_string(), 200);
        tuotteet.insert("Tee".to_string(), 180);
        tuotteet.insert("Kaakao".to_string(), 170);
        
        Automaatti {
            tila: AutomaatinTila::OdottaaRahaa { kolikot: 0 },
            tuotteet,
        }
    }
    
    fn käsittele_tapahtuma(&mut self, tapahtuma: AutomaatinTapahtuma) {
        let uusi_tila = match (&self.tila, tapahtuma) {
            // Rahaa odotetaan
            (AutomaatinTila::OdottaaRahaa { kolikot }, AutomaatinTapahtuma::LisääKolikko(uusi_kolikko)) => {
                AutomaatinTila::OdottaaRahaa { kolikot: kolikot + uusi_kolikko }
            }
            
            (AutomaatinTila::OdottaaRahaa { kolikot }, AutomaatinTapahtuma::ValitseTuote(tuote, hinta)) => {
                if *kolikot >= hinta {
                    if self.tuotteet.contains_key(&tuote) {
                        AutomaatinTila::TuoteValittu { kolikot: *kolikot, tuote }
                    } else {
                        AutomaatinTila::Virhe { viesti: format!("Tuote '{}' ei ole saatavilla", tuote) }
                    }
                } else {
                    AutomaatinTila::Virhe { viesti: format!("Ei tarpeeksi rahaa! Tarvitset {} senttiä lisää", hinta - kolikot) }
                }
            }
            
            // Tuote valittu
            (AutomaatinTila::TuoteValittu { kolikot, tuote }, AutomaatinTapahtuma::OtaTuote) => {
                let hinta = self.tuotteet.get(tuote).unwrap_or(&0);
                let vaihtoraha = kolikot - hinta;
                
                if vaihtoraha > 0 {
                    AutomaatinTila::AnnetaanVaihtorahaa { vaihtoraha }
                } else {
                    AutomaatinTila::TuoteAnnetaan { tuote: tuote.clone() }
                }
            }
            
            // Vaihtorahan antaminen
            (AutomaatinTila::AnnetaanVaihtorahaa { vaihtoraha: _ }, AutomaatinTapahtuma::OtaVaihtoraha) => {
                AutomaatinTila::OdottaaRahaa { kolikot: 0 }
            }
            
            // Tuotteen antaminen
            (AutomaatinTila::TuoteAnnetaan { tuote: _ }, AutomaatinTapahtuma::OtaTuote) => {
                AutomaatinTila::OdottaaRahaa { kolikot: 0 }
            }
            
            // Virheen käsittely
            (AutomaatinTila::Virhe { viesti: _ }, AutomaatinTapahtuma::Nollaa) => {
                AutomaatinTila::OdottaaRahaa { kolikot: 0 }
            }
            
            // Nollaus mistä tahansa tilasta
            (_, AutomaatinTapahtuma::Nollaa) => {
                AutomaatinTila::OdottaaRahaa { kolikot: 0 }
            }
            
            // Virheelliset siirtymät
            _ => {
                AutomaatinTila::Virhe { viesti: "Virheellinen toiminto nykyisessä tilassa".to_string() }
            }
        };
        
        self.tila = uusi_tila;
    }
    
    fn näytä_tila(&self) -> String {
        match &self.tila {
            AutomaatinTila::OdottaaRahaa { kolikot } => {
                format!("💰 Odottaa rahaa (nyt: {} senttiä)\nSaatavilla: {:?}", kolikot, self.tuotteet.keys().collect::<Vec<_>>())
            }
            AutomaatinTila::TuoteValittu { kolikot, tuote } => {
                format!("🎯 Tuote '{}' valittu (rahaa: {} senttiä)", tuote, kolikot)
            }
            AutomaatinTila::AnnetaanVaihtorahaa { vaihtoraha } => {
                format!("💸 Vaihtoraha: {} senttiä - ota rahat!", vaihtoraha)
            }
            AutomaatinTila::TuoteAnnetaan { tuote } => {
                format!("🥤 Tuote '{}' valmis - ota tuote!", tuote)
            }
            AutomaatinTila::Virhe { viesti } => {
                format!("❌ Virhe: {}", viesti)
            }
        }
    }
}

fn main() {
    println!("=== Rust Tilakone -esimerkit ===\n");
    
    // Liikennevalo-esimerkki
    println!("--- Liikennevalo-simulaatio ---");
    let mut valo = Liikennevalo::uusi();
    
    println!("Liikennevalo käynnistetty!");
    for sykli in 0..20 {
        println!("Sykli {}: {:?} - {}", sykli, valo.nykyinen_tila, valo.tilan_kuvaus());
        valo.päivitä();
        
        // Huoltotila syklissä 15
        if sykli == 15 {
            println!("  🔧 Siirretään huoltotilaan!");
            valo.huolto_tila();
        }
    }
    
    // Automaatti-esimerkki
    println!("\n--- Juoma-automaatti simulaatio ---");
    let mut automaatti = Automaatti::uusi();
    
    let tapahtumat = vec![
        AutomaatinTapahtuma::LisääKolikko(50),
        AutomaatinTapahtuma::LisääKolikko(100),
        AutomaatinTapahtuma::ValitseTuote("Kahvi".to_string(), 200),
        AutomaatinTapahtuma::LisääKolikko(50),
        AutomaatinTapahtuma::ValitseTuote("Kahvi".to_string(), 200),
        AutomaatinTapahtuma::OtaTuote,
        AutomaatinTapahtuma::Nollaa,
        AutomaatinTapahtuma::LisääKolikko(200),
        AutomaatinTapahtuma::ValitseTuote("Tee".to_string(), 180),
        AutomaatinTapahtuma::OtaTuote,
        AutomaatinTapahtuma::OtaVaihtoraha,
    ];
    
    println!("{}", automaatti.näytä_tila());
    
    for (i, tapahtuma) in tapahtumat.into_iter().enumerate() {
        println!("\n{}. Tapahtuma: {:?}", i + 1, tapahtuma);
        automaatti.käsittele_tapahtuma(tapahtuma);
        println!("   -> {}", automaatti.näytä_tila());
    }
    
    println!("\n=== Tilakoneet valmis! ===");
}
