# 07 — Fisiere si I/O de baza

## Analogie

Fisierul de pe disc e ca un **caiet fizic**. Ca sa citesti din el, il deschizi, citesti, il inchizi. Ca sa scrii, il deschizi (sau iei unul nou), scrii, il inchizi. Rust face acelasi lucru — dar te obliga sa gestionezi erorile (caietul poate lipsi, poate fi blocat, etc.).

---

## Modulele necesare

```rust
use std::fs;                          // operatii cu fisiere
use std::io::{self, BufRead, Write};  // citire/scriere buffered
use std::fs::OpenOptions;             // control avansat la deschidere
```

---

## 1. Scriere simpla — `fs::write()`

Cel mai simplu mod. Creeaza fisierul daca nu exista, **suprascrie** daca exista.

```rust
fs::write("fisier.txt", "continut").expect("Eroare la scriere");
```

---

## 2. Citire simpla — `fs::read_to_string()`

Citeste **tot fisierul** intr-un `String`.

```rust
let continut = fs::read_to_string("fisier.txt").expect("Eroare la citire");
println!("{}", continut);
```

> Bun pentru fisiere mici. Pentru fisiere mari, foloseste `BufReader`.

---

## 3. Citire linie cu linie — `BufReader`

`BufReader` citeste **o linie pe rand** — eficient pentru fisiere mari.

```rust
let fisier = fs::File::open("fisier.txt").expect("Nu gasesc fisierul");
let reader = io::BufReader::new(fisier);

for linie in reader.lines() {
    let linie = linie.expect("Eroare linie");
    println!("{}", linie);
}
```

Cu index:
```rust
for (i, linie) in reader.lines().enumerate() {
    let linie = linie.expect("Eroare");
    println!("Linie {}: {}", i + 1, linie);
}
```

---

## 4. Adaugare la sfarsit — `OpenOptions`

`fs::write()` suprascrie. `OpenOptions` cu `.append(true)` adauga la sfarsit:

```rust
let mut f = OpenOptions::new()
    .append(true)
    .open("fisier.txt")
    .expect("Eroare");

writeln!(f, "linie noua").expect("Eroare scriere");
```

---

## 5. Gestionarea erorilor cu `?`

In loc de `.expect()` (care face panic), folosim `?` care **propaga eroarea** catre apelant:

```rust
fn citeste(cale: &str) -> Result<String, io::Error> {
    let continut = fs::read_to_string(cale)?;  // ? = returneaza Err daca esueaza
    Ok(continut)
}
```

Apelarea:
```rust
match citeste("fisier.txt") {
    Ok(text) => println!("{}", text),
    Err(e)   => println!("Eroare: {}", e),
}
```

---

## 6. Verificare existenta fisier

```rust
std::path::Path::new("fisier.txt").exists()  // true / false
```

---

## Rezumat operatii

| Operatie | Functie | Observatie |
|----------|---------|------------|
| Scriere (simpla) | `fs::write(path, content)` | Suprascrie |
| Citire (simpla) | `fs::read_to_string(path)` | Tot fisierul in String |
| Citire (linie) | `BufReader` + `.lines()` | Eficient pentru fisiere mari |
| Adaugare | `OpenOptions::new().append(true)` | Nu suprascrie |
| Stergere | `fs::remove_file(path)` | Returneaza `Result` |
| Existenta | `Path::new(path).exists()` | Returneaza `bool` |

---

## Exercitii

Fisier de lucru: `exercitii/src/bin/ex_07_files_io.rs`
Rulare: `cargo run --bin ex_07_files_io`

---

## Solutii si observatii (sesiunea 2026-03-23)

*(se completeaza dupa rezolvarea exercitiilor)*

---

*Nota: Fisier creat in sesiunea din 2026-03-23.*
