// Lectia 06 — Colectii: Vec si HashMap
// Rulare: cargo run --bin 06_collections

use std::collections::HashMap;

// ============================================================
// 1. Vec<T> — lista dinamica
// ============================================================
// Vec e ca un sertar cu sloturi numerotate.
// Nu ai nevoie de dimensiune fixa — creste si scade la runtime.
//
// Diferenta fata de array:
//   [i32; 5]  — dimensiune FIXA, stocata pe stack
//   Vec<i32>  — dimensiune DINAMICA, stocata pe heap

fn demo_vec_creare() {
    println!("--- Creare Vec ---");

    let mut v: Vec<i32> = Vec::new(); // gol, tip explicit
    let v2 = vec![10, 20, 30]; // macro vec! cu valori initiale

    v.push(1);
    v.push(2);
    v.push(3);

    println!("  v  = {:?}", v);
    println!("  v2 = {:?}", v2);
}

// ============================================================
// 2. Acces la elemente — doua moduri
// ============================================================
// v[i]      — acces direct, face PANIC daca indexul nu exista
// v.get(i)  — returneaza Option<&T>, sigur

fn demo_vec_acces() {
    println!("\n--- Acces Vec ---");

    let v = vec!["Arthas", "Zara", "Goblin"];

    // acces direct
    println!("  v[0] = {}", v[0]);

    // acces sigur cu get() — returneaza Option
    match v.get(1) {
        Some(nume) => println!("  v.get(1) = {}", nume),
        None => println!("  Index inexistent"),
    }

    match v.get(99) {
        Some(nume) => println!("  v.get(99) = {}", nume),
        None => println!("  v.get(99) = None (index inexistent)"),
    }

    // v[99]  <-- ar face panic! nu incerca
}

// ============================================================
// 3. Modificare — push, pop, remove
// ============================================================
//
//  push(val)      — adauga la sfarsit
//  pop()          — scoate ultimul, returneaza Option<T>
//  remove(index)  — scoate de la index, returneaza T (panic daca index invalid)
//  len()          — numarul de elemente
//  is_empty()     — true daca nu are niciun element

fn demo_vec_modificare() {
    println!("\n--- Modificare Vec ---");

    let mut inventar = vec!["Sabie", "Armura", "Potion"];
    println!("  Initial:  {:?}", inventar);

    inventar.push("Cheie");
    println!("  push:     {:?}", inventar);

    let scos = inventar.pop(); // returneaza Option<T>
    println!("  pop:      {:?} | scos: {:?}", inventar, scos);

    let item = inventar.remove(1); // scoate "Armura"
    println!("  remove(1): {:?} | scos: {}", inventar, item);

    println!("  len: {}", inventar.len());
}

// ============================================================
// 4. Iterare peste Vec
// ============================================================
// for x in &v      — imprumut (cel mai comun)
// for x in &mut v  — imprumut mutabil (ca sa modifici valorile)
// for x in v       — consuma Vec-ul (il muta, nu mai poti folosi v dupa)

fn demo_vec_iterare() {
    println!("\n--- Iterare Vec ---");

    let jucatori = vec!["Arthas", "Zara", "Goblin"];

    // iterare simpla
    for jucator in &jucatori {
        println!("  Jucator: {}", jucator);
    }

    // iterare cu index — folosim enumerate()
    for (i, jucator) in jucatori.iter().enumerate() {
        println!("  [{}] {}", i, jucator);
    }
}

// ============================================================
// 5. HashMap<K, V> — dictionar cheie→valoare
// ============================================================
// HashMap e ca un dictionar real: cauti dupa CHEIE si gasesti VALOAREA.
// Nu are ordine garantata — elementele nu sunt in ordinea inserarii.
//
// IMPORTANT: trebuie importat — nu e in prelude ca Vec.
//   use std::collections::HashMap;

fn demo_hashmap_creare() {
    println!("\n--- Creare HashMap ---");

    let mut hp: HashMap<String, i32> = HashMap::new();

    // insert(cheie, valoare)
    hp.insert(String::from("Arthas"), 100);
    hp.insert(String::from("Zara"), 80);
    hp.insert(String::from("Goblin"), 30);

    println!("  hp = {:?}", hp);
}

// ============================================================
// 6. Acces in HashMap
// ============================================================
//  map.get(cheie)          — returneaza Option<&V>
//  map.contains_key(cheie) — true/false
//  map[cheie]              — acces direct, panic daca nu exista

fn demo_hashmap_acces() {
    println!("\n--- Acces HashMap ---");

    let mut hp: HashMap<&str, i32> = HashMap::new();
    hp.insert("Arthas", 100);
    hp.insert("Goblin", 30);

    // get returneaza Option<&V>
    match hp.get("Arthas") {
        Some(viata) => println!("  Arthas are {} HP", viata),
        None => println!("  Arthas nu exista"),
    }

    match hp.get("Dragon") {
        Some(viata) => println!("  Dragon are {} HP", viata),
        None => println!("  Dragon nu exista in mapa"),
    }

    println!("  Goblin exista? {}", hp.contains_key("Goblin"));
}

// ============================================================
// 7. Modificare HashMap
// ============================================================
//  insert(k, v)           — adauga sau SUPRASCRIE daca cheia exista deja
//  remove(k)              — sterge, returneaza Option<V>
//  entry(k).or_insert(v)  — insereaza DOAR daca cheia nu exista

fn demo_hashmap_modificare() {
    println!("\n--- Modificare HashMap ---");

    let mut scoruri: HashMap<&str, i32> = HashMap::new();
    scoruri.insert("Arthas", 50);
    println!("  Initial: {:?}", scoruri);

    // suprascrie valoarea existenta
    scoruri.insert("Arthas", 75);
    println!("  Dupa insert din nou: {:?}", scoruri);

    // entry: adauga DOAR daca nu exista
    scoruri.entry("Zara").or_insert(40);
    scoruri.entry("Arthas").or_insert(0); // nu face nimic — Arthas deja exista
    println!("  Dupa entry: {:?}", scoruri);

    // remove
    let scos = scoruri.remove("Zara");
    println!("  Dupa remove Zara: {:?} | scos: {:?}", scoruri, scos);
}

// ============================================================
// 8. Iterare peste HashMap
// ============================================================

fn demo_hashmap_iterare() {
    println!("\n--- Iterare HashMap ---");

    let mut echipa: HashMap<&str, &str> = HashMap::new();
    echipa.insert("Arthas", "Warrior");
    echipa.insert("Zara", "Archer");
    echipa.insert("Mira", "Mage");

    for (nume, clasa) in &echipa {
        println!("  {} joaca ca {}", nume, clasa);
    }
}

// ============================================================
// 9. Vec vs HashMap — cand le folosesti
// ============================================================
//
//  Vec<T>            HashMap<K,V>
//  ──────────────    ──────────────────────────────
//  Ordine conteza    Ordine nu conteaza
//  Acces dupa index  Acces dupa cheie
//  Liste, cozi       Dictionare, tabele de cautare
//  Inventar ordonat  HP per jucator, scoruri

// ============================================================
// MAIN — demonstratie
// ============================================================

fn main() {
    println!("=== Lectia 06: Vec si HashMap ===\n");

    demo_vec_creare();
    demo_vec_acces();
    demo_vec_modificare();
    demo_vec_iterare();

    demo_hashmap_creare();
    demo_hashmap_acces();
    demo_hashmap_modificare();
    demo_hashmap_iterare();
}
