// Rust Traits (Piirteet) - Esimerkki
// Trait määrittelee yhteisen käyttäytymisen eri tyypeille

// Määritellään trait "Ääntely" - kaikki eläimet voivat tehdä ääntä
trait Aantely {
    fn tee_aani(&self) -> String;
    
    // Default implementaatio - voidaan ylikirjoittaa
    fn esittele_itsesi(&self) -> String {
        format!("Hei, olen eläin ja teen näin: {}", self.tee_aani())
    }
}

// Määritellään eri eläintyypit
struct Koira {
    nimi: String,
    rotu: String,
}

struct Kissa {
    nimi: String,
    vari: String,
}

struct Lehma {
    nimi: String,
    maito_litraa: f32,
}

// Implementoidaan Aantely trait koiralle
impl Aantely for Koira {
    fn tee_aani(&self) -> String {
        "Vuh vuh!".to_string()
    }
    
    // Ylikirjoitetaan default implementaatio
    fn esittele_itsesi(&self) -> String {
        format!("Hei, olen {} ja olen {} -rotuinen koira! {}", 
                self.nimi, self.rotu, self.tee_aani())
    }
}

// Implementoidaan Aantely trait kissalle
impl Aantely for Kissa {
    fn tee_aani(&self) -> String {
        "Miau!".to_string()
    }
    
    fn esittele_itsesi(&self) -> String {
        format!("Hei, olen {} ja olen {} kissa! {}", 
                self.nimi, self.vari, self.tee_aani())
    }
}

// Implementoidaan Aantely trait lehmälle
impl Aantely for Lehma {
    fn tee_aani(&self) -> String {
        "Muu!".to_string()
    }
}

// Yleinen funktio joka ottaa vastaan minkä tahansa Aantely-traitin toteuttavan objektin
fn elaimen_konsertti<T: Aantely>(elain: &T) {
    println!("{}", elain.esittele_itsesi());
    println!("Eläin tekee ääntä: {}", elain.tee_aani());
    println!("---");
}

// Funktio joka ottaa vastaan vektorin eläimiä (trait object)
fn elainkoro_konsertti(elaimet: Vec<&dyn Aantely>) {
    println!("🎵 Eläinkoron konsertti alkaa! 🎵");
    for elain in elaimet {
        println!("{}", elain.esittele_itsesi());
    }
    println!("🎵 Konsertti päättyi! 🎵\n");
}

fn main() {
    println!("=== Rust Traits (Piirteet) Esimerkki ===\n");
    
    // Luodaan eri eläimiä
    let koira = Koira {
        nimi: "Musti".to_string(),
        rotu: "Saksanpaimenkoira".to_string(),
    };
    
    let kissa = Kissa {
        nimi: "Misse".to_string(),
        vari: "musta".to_string(),
    };
    
    let lehma = Lehma {
        nimi: "Mansikki".to_string(),
        maito_litraa: 25.5,
    };
    
    // Demonstroi trait-funktioita yksittäisille eläimille
    println!("--- Yksittäiset eläimet ---");
    elaimen_konsertti(&koira);
    elaimen_konsertti(&kissa);
    elaimen_konsertti(&lehma);
    
    // Demonstroi trait objecteja (dyn Aantely)
    let elainkoro: Vec<&dyn Aantely> = vec![&koira, &kissa, &lehma];
    elainkoro_konsertti(elainkoro);
    
    // Esimerkki trait boundeista
    println!("--- Trait bounds esimerkki ---");
    let elainkoro2: Vec<&dyn Aantely> = vec![&koira, &kissa, &lehma];
    for (i, elain) in elainkoro2.iter().enumerate() {
        println!("Eläin #{}: {}", i + 1, elain.tee_aani());
    }
    
    println!("\n=== Traits mahdollistavat: ===");
    println!("✓ Yhteisen käyttäytymisen määrittelyn");
    println!("✓ Polymorfismin (sama funktio eri tyypeille)");
    println!("✓ Koodin uudelleenkäytön");
    println!("✓ Trait objectit (dyn Trait)");
    println!("✓ Generic funktiot trait boundeilla");
}
