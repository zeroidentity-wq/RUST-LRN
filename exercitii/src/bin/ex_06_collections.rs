use rand::Rng;
// Exercitii 06 — Vec si HashMap
// Rulare: cargo run --bin ex_06_collections

use std::collections::HashMap;
use rand::random;
// ============================================================
// EXERCITIUL 1 — Vec: operatii de baza
// ============================================================
// Creeaza un Vec<String> cu numele a 3 jucatori.
// Adauga un al 4-lea jucator cu push.
// Printeaza toti jucatorii cu un for si index (foloseste enumerate).
// Scoate ultimul cu pop si printeaza ce ai scos (e Option — foloseste match).
// ============================================================

fn ex1(){
    let mut v: Vec<String> = vec!["Albastros".to_string(), "Murish".to_string(), "Gablov".to_string()];
    v.push(String::from("Cameros"));
    // iterare cu index — folosim enumerate()
    for (i, player) in v.iter().enumerate() {
        println!("Player {}: {}", i,player);
    }
    match v.pop() {
        None => { println!("No player"); },
        Some(scos) => {println!("Scos: {}", scos);},
    }

}

// ============================================================
// EXERCITIUL 2 — Vec: acces sigur
// ============================================================
// Pornind de la Vec-ul de mai sus (sau unul nou),
// incearca sa accesezi un index care nu exista folosind .get().
// Printeaza "Gasit: X" sau "Index inexistent" cu match.
// ============================================================

fn ex2 () {
    let mut v: Vec<String> = vec!["Albastros".to_string(), "Murish".to_string(), "Gablov".to_string()];
    for (i, player) in v.iter().enumerate() {
        println!("Player {}: {}", i,player);
    }
    match v.get(3) {
        None => { println!("No player gasit la index 3."); },
        Some(gasit) => {println!("Gasit: {}", gasit);},
    }

}

// ============================================================
// EXERCITIUL 3 — HashMap: construieste un registru de HP
// ============================================================
// Creeaza un HashMap<String, i32> care mapeaza
// numele jucatorilor la HP-ul lor.
// Adauga minim 3 jucatori.
// Afiseaza HP-ul unui jucator care EXISTA si al unuia care NU exista,
// folosind match pe rezultatul lui .get().
// ============================================================

fn ex03() {
    let mut hp: HashMap<String, i32> = HashMap::new();
    hp.insert("Damos".to_string(), 100);
    hp.insert("Galos".to_string(), 22);
    hp.insert("Helios".to_string(), 55);

    match hp.get("Dragon") {
        None => { println!("No dragon"); },
        Some(viata) => { println!("Dragon: {}", viata); },
    }

    match hp.get("Damos") {
        None => { println!("No player found!"); },
        Some(viata) => { println!("Player: Damos : {}", viata); },
    }


}

// ============================================================
// EXERCITIUL 4 — HashMap: entry si modificare
// ============================================================
// Pornind de la HashMap-ul din ex 3:
// - Incearca sa adaugi un jucator deja existent cu entry().or_insert()
//   si verifica ca valoarea nu s-a schimbat.
// - Sterge un jucator cu remove() si printeaza ce ai scos.
// - Itereaza peste toti jucatorii ramasi si afiseaza "Nume: X, HP: Y".
// ============================================================
fn ex04(){
    let mut hp: HashMap<String, i32> = HashMap::new();
    hp.insert("Damos".to_string(), 100);
    hp.insert("Galos".to_string(), 22);
    hp.insert("Helios".to_string(), 55);
    println!("Inainte de entry: {:?}", hp);
    hp.entry("Davos".to_string()).or_insert(50);
    hp.entry("Davos".to_string()).or_insert(50); // deja existent nu va rula
    println!("Dupa entry: {:?}", hp);
    match hp.remove("Davos") {
        None => {println!("No player found!");},
        Some(scos) => { println!("Scos pe: {}", scos);},
    }
    //iteram peste toti jucatorii
    for (i, (nume,viata)) in hp.iter().enumerate() {
        println!("Player {i}: Nume: {nume}: HP: {viata}");
    }

}
// ============================================================
// EXERCITIUL 5 — Combinat: inventar + scor
// ============================================================
// Creeaza:
//   - un Vec<&str> cu itemele unui jucator (ex: "Sabie", "Potion", "Armura")
//   - un HashMap<&str, i32> cu scorul fiecarui item (ex: "Sabie" -> 30)
//
// Itereaza peste inventar si pentru fiecare item,
// cauta scorul in HashMap si afiseaza "Item: X, Scor: Y"
// sau "Item: X, Scor: necunoscut" daca nu e in HashMap.
// ============================================================

fn ex05() {
    let mut rng = rand::thread_rng();
    let inventar: Vec<&str> = vec!["Sabie", "Poțiune", "Armura"];
    let mut hp: HashMap<&str, i32> = HashMap::new();
    // atribuire scor
    for item in inventar.iter() {
        let numar_aleator: i32 = rng.gen_range(0..=100);
        hp.insert(&item,numar_aleator);
        match hp.get(item) {
            None => {println!("No player found!");},
            Some(score) => {println!("Item: {} Score : {}", item, score)}
        }
    }
    println!("{:?}", hp);

    // iterez inventar si pentru item, if item == k atunci afisez valoarea
    for item in inventar.iter() {
        match hp.get(item) {
            None => {println!("Item: {}, Scor: necunoscut", item);}
            Some(score) => {println!("Item: {}, Scor: {}", item, score );}
        }
    }





}

fn main() {
    println!("=== Exercitii 06: Vec si HashMap ===\n");

    println!("--- Exercitiul 1 ---");
    ex1();

    println!("\n--- Exercitiul 2 ---");
    ex2();

    println!("\n--- Exercitiul 3 ---");
    ex03();

    println!("\n--- Exercitiul 4 ---");
    ex04();

    println!("\n--- Exercitiul 5 ---");
    ex05();
}
