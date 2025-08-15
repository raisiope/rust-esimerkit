// Rust enum -esimerkit
#[derive(Debug)]
enum Viikonpäivä {
    Maanantai,
    Tiistai,
    Keskiviikko,
    Torstai,
    Perjantai,
    Lauantai,
    Sunnuntai,
}

// Enum, joka voi sisältää dataa
#[derive(Debug)]
enum Sää {
    Aurinkoinen(i32),           // lämpötila
    Sateinen { sadekorkeus: f64 }, // nimetty kenttä
    Luminen,                    // ei dataa
    Tuulinen(i32, String),      // lämpötila ja tuulen suunta
}

// Enum message-systeemille
#[derive(Debug)]
enum Viesti {
    Lopeta,
    Siirrä { x: i32, y: i32 },
    Kirjoita(String),
    MuutaVäri(i32, i32, i32), // RGB
}

impl Viikonpäivä {
    fn on_viikonloppu(&self) -> bool {
        match self {
            Viikonpäivä::Lauantai | Viikonpäivä::Sunnuntai => true,
            _ => false,
        }
    }
    
    fn seuraava_päivä(&self) -> Viikonpäivä {
        match self {
            Viikonpäivä::Maanantai => Viikonpäivä::Tiistai,
            Viikonpäivä::Tiistai => Viikonpäivä::Keskiviikko,
            Viikonpäivä::Keskiviikko => Viikonpäivä::Torstai,
            Viikonpäivä::Torstai => Viikonpäivä::Perjantai,
            Viikonpäivä::Perjantai => Viikonpäivä::Lauantai,
            Viikonpäivä::Lauantai => Viikonpäivä::Sunnuntai,
            Viikonpäivä::Sunnuntai => Viikonpäivä::Maanantai,
        }
    }
}

fn käsittele_sää(sää: &Sää) {
    match sää {
        Sää::Aurinkoinen(lämpötila) => {
            if *lämpötila > 20 {
                println!("Kaunis aurinkoinen päivä! {}°C - Hyvä päivä ulkoiluun! ☀️", lämpötila);
            } else {
                println!("Aurinkoista mutta hieman viileää: {}°C", lämpötila);
            }
        }
        Sää::Sateinen { sadekorkeus } => {
            println!("Sataa {}mm. Muista sateenvarjo! 🌧️", sadekorkeus);
        }
        Sää::Luminen => {
            println!("Lumisadetta! Talvivaatteet päälle! ❄️");
        }
        Sää::Tuulinen(lämpötila, suunta) => {
            println!("Tuulista {}°C, tuuli puhaltaa {}:sta 💨", lämpötila, suunta);
        }
    }
}

fn prosessoi_viesti(viesti: Viesti) {
    match viesti {
        Viesti::Lopeta => {
            println!("Lopetetaan ohjelma...");
        }
        Viesti::Siirrä { x, y } => {
            println!("Siirretään positioon ({}, {})", x, y);
        }
        Viesti::Kirjoita(teksti) => {
            println!("Kirjoitetaan: '{}'", teksti);
        }
        Viesti::MuutaVäri(r, g, b) => {
            println!("Vaihdetaan väriksi RGB({}, {}, {})", r, g, b);
        }
    }
}

fn main() {
    println!("=== Rust Enum -esimerkit ===\n");
    
    // Yksinkertaiset enumit
    println!("--- Viikonpäivät ---");
    let tänään = Viikonpäivä::Keskiviikko;
    println!("Tänään on {:?}", tänään);
    println!("Onko viikonloppu? {}", tänään.on_viikonloppu());
    println!("Huomenna on {:?}", tänään.seuraava_päivä());
    
    let viikonloppu = Viikonpäivä::Lauantai;
    println!("Lauantai - viikonloppu? {}", viikonloppu.on_viikonloppu());
    
    // Enumit datalla
    println!("\n--- Säätiedot ---");
    let säätilat = vec![
        Sää::Aurinkoinen(25),
        Sää::Sateinen { sadekorkeus: 3.2 },
        Sää::Luminen,
        Sää::Tuulinen(15, "pohjoisesta".to_string()),
        Sää::Aurinkoinen(18),
    ];
    
    for sää in &säätilat {
        käsittele_sää(sää);
    }
    
    // Viesti-enum
    println!("\n--- Viestien käsittely ---");
    let viestit = vec![
        Viesti::Kirjoita("Hei maailma!".to_string()),
        Viesti::Siirrä { x: 100, y: 200 },
        Viesti::MuutaVäri(255, 0, 128),
        Viesti::Lopeta,
    ];
    
    for viesti in viestit {
        prosessoi_viesti(viesti);
    }
    
    // Option ja Result (Rustin sisäänrakennetut enumit)
    println!("\n--- Option ja Result enumit ---");
    
    // Option<T> esimerkki
    let numerot = vec![1, 2, 3, 4, 5];
    match numerot.get(2) {
        Some(arvo) => println!("Indeksi 2: {}", arvo),
        None => println!("Indeksi 2 ei löydy"),
    }
    
    match numerot.get(10) {
        Some(arvo) => println!("Indeksi 10: {}", arvo),
        None => println!("Indeksi 10 ei löydy"),
    }
    
    // Result<T, E> esimerkki
    let luku_str = "123";
    match luku_str.parse::<i32>() {
        Ok(luku) => println!("Parsittiin onnistuneesti: {}", luku),
        Err(e) => println!("Parsinta epäonnistui: {}", e),
    }
    
    let huono_luku = "abc";
    match huono_luku.parse::<i32>() {
        Ok(luku) => println!("Parsittiin: {}", luku),
        Err(e) => println!("'{}' ei ole validi luku: {}", huono_luku, e),
    }
}
