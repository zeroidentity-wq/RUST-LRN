// Exercise 01b — Ownership & Borrowing (exercitii suplimentare)
// Enunturi complete in: notes/01_ownership_borrowing.md
// Rulare: cargo run --bin ex_01b_ownership_extra

use std::fmt::format;

fn main() {
    main_s1a();
    let inventar_player1 = String::from("Sabie Mare");
    inventar_gol(&inventar_player1);
    println!("{}", inventar_gol(&inventar_player1));
    let nume_player1 = String::from("exeduss");
    let nr_charactere = lungime_nume(&nume_player1);
    println!("Nr charactere nume_player1: {}", nr_charactere);
    let mut nume_player2 = String::from("Arthus");
    fa_majuscule(&mut nume_player2);
    println!("{}", nume_player2);
    let titluri_combinate = combina_titlu(&nume_player1, &nume_player2);
    println!("{}", titluri_combinate);

    let mut titlu = String::from("NOOB");
    schimba_titlu(&mut titlu, "Intermediar");
    println!("Titlul nou este: {}", titlu);
}

// ---------------------------------------------------------------------------
// SERIE 1 — DIFICULTATE MICA
// ---------------------------------------------------------------------------

// S1-A: NPC-ul `afiseaza` primeste item-ul prin move si il distruge.
//       Repara codul din main FARA sa schimbi functia `afiseaza`,
//       astfel incat sa poti folosi item-ul si dupa apel.
//
fn main_s1a() {
    let item = String::from("Potion of Healing");
    afiseaza(item.clone());
    println!("Am inca: {}", item); // pt ca aic item este deja drop din cauza functiei afiseaza
}
fn afiseaza(text: String) {
    println!("{}", text);
}

// S1-B: Returneaza true daca inventarul eroului e gol (string-ul e gol)
//       Exemplu: "" -> true, "Sabie" -> false
//       Indiciu: .is_empty() sau .len() == 0
fn inventar_gol(s: &String) -> bool {
    let is_empty = s.is_empty();
    is_empty
}

// S1-C: Returneaza numarul de CARACTERE (nu bytes) din numele personajului
//       Exemplu: "Rust" -> 4
//       Indiciu: .chars().count() numara caractere Unicode, .len() numara bytes
fn lungime_nume(s: &String) -> usize {
    s.chars().count()
}

// ---------------------------------------------------------------------------
// SERIE 2 — DIFICULTATE MEDIE
// ---------------------------------------------------------------------------

// S2-A: Blacksmith-ul transforma numele eroului in majuscule, in loc
//       Exemplu: "arthur" -> "ARTHUR"
//       Indiciu: .to_uppercase() returneaza un String nou — cum il atribui inapoi?
fn fa_majuscule(s: &mut String) {
    *s = s.to_uppercase();
}

// S2-B: Combina titlul si numele eroului fara sa muti niciun owner
//       Exemplu: "Lord", "Arthur" -> "Lord Arthur"
//       Indiciu: format!("{} {}", s1, s2)
fn combina_titlu(s1: &String, s2: &String) -> String {
    format!("{} {}", s1, s2)
}

// S2-C: Functia de mai jos nu compileaza.
//       Explica de ce in comentariu, apoi repara-o in doua moduri diferite.
//       Pare ca este o eroare de lifetime specifier
fn creeaza_item1() -> String {
    let s = String::from("Sabie Ruginita");
    s
}

fn creeaza_item2() -> &'static str {
    "Sabie Ruginita"
}

// S2-D: functie schimba_titlu(titlu: &mut String, nou: &str)
//
fn schimba_titlu(titlu: &mut String, nou: &str) {
    *titlu = nou.to_string().to_uppercase();
}
