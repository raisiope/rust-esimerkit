// Yksinkertainen Hei maailma -ohjelma Rustilla
fn main() {
    println!("Hei maailma!");
    println!("Tervetuloa Rust-ohjelmointiin! 🦀");
    
    // Lisätään hieman interaktiivisuutta
    let nimi = "Pekka";
    let ika = 25;
    
    println!("Moikka, {}! Olet {} vuotta vanha.", nimi, ika);
    
    // Yksinkertainen laskutoimitus
    let tulos = laskuharjoitus(10, 5);
    println!("10 + 5 = {}", tulos);
}

// Yksinkertainen funktio
fn laskuharjoitus(a: i32, b: i32) -> i32 {
    a + b
}
