// Exercitii 07 — Fisiere si I/O de baza
// Rulare: cargo run --bin ex_07_files_io

use std::fs;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};

// ============================================================
// EXERCITIUL 1 — Scriere si citire simpla
// ============================================================
// 1. Scrie un fisier numit "jucatori.txt" care contine
//    3 nume de jucatori, fiecare pe o linie separata.
// 2. Citeste fisierul inapoi si printeaza continutul.
// 3. Printeaza numarul total de caractere din fisier.
// ============================================================

fn ex1() {
    // TODO
}

// ============================================================
// EXERCITIUL 2 — Citire linie cu linie
// ============================================================
// Pornind de la fisierul "jucatori.txt" creat la ex1:
// Citeste-l linie cu linie folosind BufReader
// si printeaza fiecare linie cu numarul ei (incepand de la 1).
// Exemplu output: "1: Albastros"
// ============================================================

fn ex2() {
    // TODO
}

// ============================================================
// EXERCITIUL 3 — Adaugare la fisier
// ============================================================
// Adauga inca 2 jucatori la sfarsitul fisierului "jucatori.txt"
// folosind OpenOptions cu append.
// Dupa adaugare, citeste si afiseaza tot fisierul
// pentru a confirma ca datele noi sunt la sfarsit.
// ============================================================

fn ex3() {
    // TODO
}

// ============================================================
// EXERCITIUL 4 — Gestionarea erorilor cu Result
// ============================================================
// Scrie o functie:
//   fn citeste_fisier(cale: &str) -> Result<String, io::Error>
// care citeste un fisier si returneaza continutul.
//
// In main (ex4), apeleaz-o de doua ori:
//   - o data cu "jucatori.txt" (exista)
//   - o data cu "inexistent.txt" (nu exista)
// Trateaza ambele cazuri cu match si printeaza mesaj corespunzator.
// ============================================================

fn citeste_fisier(cale: &str) -> Result<String, io::Error> {
    // TODO
    todo!()
}

fn ex4() {
    // TODO
}

// ============================================================
// EXERCITIUL 5 — Combinat: scor per jucator din fisier
// ============================================================
// Fisierul "scoruri.txt" are formatul:
//   NumeJucator:Scor
//   Ex: "Albastros:150"
//
// 1. Creeaza fisierul "scoruri.txt" cu minim 3 intrari in formatul de mai sus.
// 2. Citeste-l linie cu linie.
// 3. Pentru fiecare linie, desparte numele de scor folosind .split_once(':')
//    si printeaza: "Jucator: X | Scor: Y"
// 4. La final, sterge ambele fisiere create (jucatori.txt, scoruri.txt).
// ============================================================

fn ex5() {
    // Hint: "Albastros:150".split_once(':') returneaza Some(("Albastros", "150"))
    // Hint: "150".parse::<i32>() converteste &str in i32 — returneaza Result
    // TODO
}

fn main() {
    println!("=== Exercitii 07: Fisiere si I/O ===\n");

    println!("--- Exercitiul 1 ---");
    ex1();

    println!("\n--- Exercitiul 2 ---");
    ex2();

    println!("\n--- Exercitiul 3 ---");
    ex3();

    println!("\n--- Exercitiul 4 ---");
    ex4();

    println!("\n--- Exercitiul 5 ---");
    ex5();
}
