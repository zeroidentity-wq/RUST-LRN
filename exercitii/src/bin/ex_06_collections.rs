// Exercitii 06 — Vec si HashMap
// Rulare: cargo run --bin ex_06_collections

use std::collections::HashMap;

// ============================================================
// EXERCITIUL 1 — Vec: operatii de baza
// ============================================================
// Creeaza un Vec<String> cu numele a 3 jucatori.
// Adauga un al 4-lea jucator cu push.
// Printeaza toti jucatorii cu un for si index (foloseste enumerate).
// Scoate ultimul cu pop si printeaza ce ai scos (e Option — foloseste match).
// ============================================================

// ============================================================
// EXERCITIUL 2 — Vec: acces sigur
// ============================================================
// Pornind de la Vec-ul de mai sus (sau unul nou),
// incearca sa accesezi un index care nu exista folosind .get().
// Printeaza "Gasit: X" sau "Index inexistent" cu match.
// ============================================================

// ============================================================
// EXERCITIUL 3 — HashMap: construieste un registru de HP
// ============================================================
// Creeaza un HashMap<String, i32> care mapeaza
// numele jucatorilor la HP-ul lor.
// Adauga minim 3 jucatori.
// Afiseaza HP-ul unui jucator care EXISTA si al unuia care NU exista,
// folosind match pe rezultatul lui .get().
// ============================================================

// ============================================================
// EXERCITIUL 4 — HashMap: entry si modificare
// ============================================================
// Pornind de la HashMap-ul din ex 3:
// - Incearca sa adaugi un jucator deja existent cu entry().or_insert()
//   si verifica ca valoarea nu s-a schimbat.
// - Sterge un jucator cu remove() si printeaza ce ai scos.
// - Itereaza peste toti jucatorii ramasi si afiseaza "Nume: X, HP: Y".
// ============================================================

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

fn main() {
    println!("=== Exercitii 06: Vec si HashMap ===\n");

    println!("--- Exercitiul 1 ---");
    // apeleaza codul pentru ex 1

    println!("\n--- Exercitiul 2 ---");
    // apeleaza codul pentru ex 2

    println!("\n--- Exercitiul 3 ---");
    // apeleaza codul pentru ex 3

    println!("\n--- Exercitiul 4 ---");
    // apeleaza codul pentru ex 4

    println!("\n--- Exercitiul 5 ---");
    // apeleaza codul pentru ex 5
}
