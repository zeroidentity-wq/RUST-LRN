// 07 — Fisiere si I/O de baza
// Rulare: cargo run --bin 07_files_io

use std::fs;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};

fn main() {
    // ============================================================
    // 1. SCRIEREA unui fisier — fs::write()
    // ============================================================
    // The simplest way: write a string directly to a file.
    // Creates the file if it doesn't exist, overwrites if it does.
    let content = "Linie 1: Salut din Rust!\nLinie 2: Fisierele sunt usor de folosit.\nLinie 3: Acesta e un test.\n";
    fs::write("test.txt", content).expect("Nu am putut scrie fisierul");
    println!("Fisier scris: test.txt");

    // ============================================================
    // 2. CITIREA unui fisier intreg — fs::read_to_string()
    // ============================================================
    // Reads the entire file into a String.
    // Returns Result<String, Error> — we use expect() for simplicity here.
    let citit = fs::read_to_string("test.txt").expect("Nu am putut citi fisierul");
    println!("\n--- Continut test.txt ---");
    println!("{}", citit);

    // ============================================================
    // 3. CITIRE LINIE CU LINIE — BufReader
    // ============================================================
    // For large files, reading everything into memory at once is expensive.
    // BufReader reads one line at a time — more efficient.
    let fisier = fs::File::open("test.txt").expect("Nu am putut deschide fisierul");
    let reader = io::BufReader::new(fisier);

    println!("--- Citire linie cu linie ---");
    for (i, linie) in reader.lines().enumerate() {
        let linie = linie.expect("Eroare la citirea liniei");
        println!("  Linie {}: {}", i + 1, linie);
    }

    // ============================================================
    // 4. ADAUGARE la un fisier existent — OpenOptions
    // ============================================================
    // fs::write() SUPRASCRIE. Pentru a adauga la sfarsit, folosim OpenOptions.
    let mut fisier_append = OpenOptions::new()
        .append(true)       // add to the end, don't overwrite
        .open("test.txt")
        .expect("Nu am putut deschide pentru append");

    writeln!(fisier_append, "Linie 4: Adaugata cu append!").expect("Eroare la append");
    println!("\nLinie adaugata cu append.");

    // Verify the append worked
    let dupa_append = fs::read_to_string("test.txt").expect("Eroare citire");
    println!("--- Dupa append ---");
    println!("{}", dupa_append);

    // ============================================================
    // 5. ERORI GESTIONATE CORECT — cu Result si ?
    // ============================================================
    // In real code, use ? instead of expect() to propagate errors.
    // See function below.
    match citeste_fisier("test.txt") {
        Ok(continut) => println!("Citit cu Result: {} caractere", continut.len()),
        Err(e) => println!("Eroare: {}", e),
    }

    match citeste_fisier("fisier_inexistent.txt") {
        Ok(_) => println!("Gasit"),
        Err(e) => println!("Eroare asteptata: {}", e),
    }

    // ============================================================
    // 6. VERIFICARE existenta fisier
    // ============================================================
    println!("\ntest.txt exista: {}", std::path::Path::new("test.txt").exists());
    println!("altceva.txt exista: {}", std::path::Path::new("altceva.txt").exists());

    // Cleanup
    fs::remove_file("test.txt").ok();
    println!("\ntest.txt sters.");
}

// Using ? operator — the function returns Result, errors bubble up automatically.
fn citeste_fisier(cale: &str) -> Result<String, io::Error> {
    let continut = fs::read_to_string(cale)?;
    Ok(continut)
}
