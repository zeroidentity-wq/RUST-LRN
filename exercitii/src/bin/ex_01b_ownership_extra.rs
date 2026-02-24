// Exercise 01b — Ownership & Borrowing (exercitii suplimentare)
// Enunturi complete in: notes/01_ownership_borrowing.md
// Rulare: cargo run --bin ex_01b_ownership_extra

fn main() {
    // Apeleaza functiile tale aici ca sa testezi
}

// ---------------------------------------------------------------------------
// SERIE 1 — DIFICULTATE MICA
// ---------------------------------------------------------------------------

// S1-A: Repara codul din enunt (rescrie main-ul de test de mai jos)
// fn main_s1a() { ... }

// S1-B: Returneaza true daca string-ul e gol
fn este_goala(s: &String) -> bool {
    todo!()
}

// S1-C: Returneaza numarul de CARACTERE (nu bytes)
fn lungime(s: &String) -> usize {
    todo!()
}

// ---------------------------------------------------------------------------
// SERIE 2 — DIFICULTATE MEDIE
// ---------------------------------------------------------------------------

// S2-A: Transforma textul in majuscule, in loc
fn fa_majuscule(s: &mut String) {
    todo!()
}

// S2-B: Returneaza un String nou = s1 + " " + s2, fara sa mute niciun owner
fn concateneaza(s1: &String, s2: &String) -> String {
    todo!()
}

// S2-C: Repara functia de mai jos (doua variante posibile)
// fn creeaza_string() -> &String {
//     let s = String::from("hello");
//     &s
// }