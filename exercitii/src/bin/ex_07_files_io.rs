// Exercitii 07 — Fisiere si I/O de baza
// Rulare: cargo run --bin ex_07_files_io

use std::fs;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Error, Write};

// ============================================================
// EXERCITIUL 1 — Scriere si citire simpla
// ============================================================
// 1. Scrie un fisier numit "jucatori.txt" care contine
//    3 nume de jucatori, fiecare pe o linie separata.
// 2. Citeste fisierul inapoi si printeaza continutul.
// 3. Printeaza numarul total de caractere din fisier.
// ============================================================

fn ex1() {
    fs::write("jucatori.txt", "Jucator 1\nJucator 2\nJucator 3\n").expect("Eroare la scriere.");
    if std::path::Path::new("E:\\RustRoverProjects\\RUST-LRN\\jucatori.txt").exists() {
        println!("fisier.txt exista;");
    }
    let continut = fs::read_to_string("E:\\RustRoverProjects\\RUST-LRN\\jucatori.txt").expect("Content not found.");
    println!("Content :\n{}", continut);
    println!("Nr total de caractere: {}", continut.chars().count());
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
    let fisier = fs::File::open("E:\\RustRoverProjects\\RUST-LRN\\jucatori.txt").expect("Nu am gasit fisierul!");
    let reader = BufReader::new(fisier);
    for (index, linie) in reader.lines().enumerate() {
        let linie = linie.expect("Could not read line.");
        println!("{} {}", index+1, linie);
    }
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
    let mut f = OpenOptions::new().append(true).open("E:\\RustRoverProjects\\RUST-LRN\\jucatori.txt").expect("Nu am gasit fisier!");
    match writeln!(f, "Jucator 4\nJucator 5") {
        Ok(_) => {println!("S-a reusit scrierea!")}
        Err(_) => {println!("Nu s-a reusit scrierea!")}
    }
    let continut = fs::read_to_string("E:\\RustRoverProjects\\RUST-LRN\\jucatori.txt").expect("Content not found.");
    println!("{}", continut)

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
    let continut = fs::read_to_string(&cale)?;
    Ok(continut)
}

fn ex4() {
    match citeste_fisier("E:\\RustRoverProjects\\RUST-LRN\\jucatori.txt") {
        Ok(citit) => {println!("{}", citit)}
        Err(_) => {println!("Nu am gasit fisierul!")}
    }
    match citeste_fisier("E:\\RustRoverProjects\\RUST-LRN\\instructiuni.txt") {
        Ok(citit) => {println!("{}", citit)}
        Err(_) => {println!("Nu am gasit fisier!")}
    }

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
    fs::write("scoruri.txt", "Albastros:50\nHoham:50\nKepolis:45\nKheops:45\n").expect("Eroare la scriere scoruri");
    let fisier = fs::File::open("scoruri.txt").expect("Nu am gasit fisierul scoruri!");
    let reader = BufReader::new(fisier);
    for line in reader.lines() {
        let line = line.expect("Could not read line.");
        match line.split_once(':') {
            None => {}
            Some(pereche) => {
                    let scor_parsat = pereche.1.parse::<i32>().expect("Nu am gasit perechement!");
                println!("Jucator: {}| Scor: {}", pereche.0, scor_parsat);
                }
        }
    }
    match fs::remove_file("E:\\RustRoverProjects\\RUST-LRN\\jucatori.txt") {
        Ok(_) => {println!("Am sters fisierul jucatori!");}
        Err(_) => {println!("Nu am gasit fisierul!");}
    }
    match fs::remove_file("E:\\RustRoverProjects\\RUST-LRN\\scoruri.txt") {
        Ok(_) => {println!("Am sters fisierul! scoruri");}
        Err(_) => {println!("Nu am gasit fisierul!");}
    }
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
