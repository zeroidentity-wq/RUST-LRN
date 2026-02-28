// Exercise 01b — Ownership & Borrowing (exercitii suplimentare)
// Enunturi complete in: notes/01_ownership_borrowing.md
// Rulare: cargo run --bin ex_01b_ownership_extra

fn main() {
    // Apeleaza functiile tale aici ca sa testezi
}

// ---------------------------------------------------------------------------
// SERIE 1 — DIFICULTATE MICA
// ---------------------------------------------------------------------------

// S1-A: NPC-ul `afiseaza` primeste item-ul prin move si il distruge.
//       Repara codul din main FARA sa schimbi functia `afiseaza`,
//       astfel incat sa poti folosi item-ul si dupa apel.
//
// fn main_s1a() {
//     let item = String::from("Potion of Healing");
//     afiseaza(item);
//     println!("Am inca: {}", item); // de ce da eroare?
// }
// fn afiseaza(text: String) { println!("{}", text); }

// S1-B: Returneaza true daca inventarul eroului e gol (string-ul e gol)
//       Exemplu: "" -> true, "Sabie" -> false
//       Indiciu: .is_empty() sau .len() == 0
fn inventar_gol(s: &String) -> bool {
    todo!()
}

// S1-C: Returneaza numarul de CARACTERE (nu bytes) din numele personajului
//       Exemplu: "Rust" -> 4
//       Indiciu: .chars().count() numara caractere Unicode, .len() numara bytes
fn lungime_nume(s: &String) -> usize {
    todo!()
}

// ---------------------------------------------------------------------------
// SERIE 2 — DIFICULTATE MEDIE
// ---------------------------------------------------------------------------

// S2-A: Blacksmith-ul transforma numele eroului in majuscule, in loc
//       Exemplu: "arthur" -> "ARTHUR"
//       Indiciu: .to_uppercase() returneaza un String nou — cum il atribui inapoi?
fn fa_majuscule(s: &mut String) {
    todo!()
}

// S2-B: Combina titlul si numele eroului fara sa muti niciun owner
//       Exemplu: "Lord", "Arthur" -> "Lord Arthur"
//       Indiciu: format!("{} {}", s1, s2)
fn combina_titlu(s1: &String, s2: &String) -> String {
    todo!()
}

// S2-C: Functia de mai jos nu compileaza.
//       Explica de ce in comentariu, apoi repara-o in doua moduri diferite.
//
// fn creeaza_item() -> &String {
//     let s = String::from("Sabie Ruginita");
//     &s
// }