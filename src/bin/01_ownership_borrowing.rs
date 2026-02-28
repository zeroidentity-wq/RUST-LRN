// ============================================================
// Ownership & Borrowing — cod demonstrativ
// Citeste notitele in: notes/01_ownership_borrowing.md
// Rulare: cargo run --bin 01_ownership_borrowing
// ============================================================

fn main() {
    demo_move();
    demo_copy();
    demo_borrowing();
    demo_mutable_borrow();
}

// --- MOVE ---
// Cand dai item-ul unui alt jucator, tu nu il mai ai
fn demo_move() {
    println!("=== MOVE ===");

    let jucator1 = String::from("Excalibur");
    let jucator2 = jucator1; // item mutat in inventarul lui jucator2
    // println!("{}", jucator1); // EROARE: jucator1 nu mai are item-ul
    println!("jucator2 are: {}", jucator2);
}

// --- COPY ---
// Gold-ul si consumabilele se copiaza automat (tipuri simple: i32, bool, etc.)
fn demo_copy() {
    println!("\n=== COPY (gold) ===");

    let gold = 500;
    let taxa = gold; // copie automata — gold ramane valid
    println!("gold: {}, taxa: {}", gold, taxa); // ambele valide
}

// --- BORROWING IMUTABIL ---
// Imprumuti item-ul coechipierului sa-l inspecteze — el nu il primeste
fn demo_borrowing() {
    println!("\n=== BORROWING (&) ===");

    let sabie = String::from("Excalibur");
    let lungime = inspecteaza_item(&sabie); // imprumut, nu move
    println!("Sabia '{}' are {} caractere", sabie, lungime); // sabia e inca a noastra
}

fn inspecteaza_item(item: &String) -> usize {
    item.len() // returnam lungimea, fara sa retinem ownership-ul
}

// --- BORROWING MUTABIL ---
// Dai item-ul la un blacksmith sa-l upgradeze — il primesti inapoi modificat
fn demo_mutable_borrow() {
    println!("\n=== BORROWING MUTABIL (&mut) ===");

    let mut sabie = String::from("Excalibur");
    println!("Inainte de upgrade: {}", sabie);
    upgrade_item(&mut sabie); // imprumut mutabil — blacksmith-ul modifica item-ul
    println!("Dupa upgrade: {}", sabie);
}

fn upgrade_item(item: &mut String) {
    item.push_str(" +1");
}