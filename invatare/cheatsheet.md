# Rust Cheatsheet Complet — Nivel Începător

---

# 1. CARGO — Managerul de proiecte

## Comenzi esențiale

```bash
cargo new proiect           # Creează proiect binary (cu fn main)
cargo new --lib proiect     # Creează proiect library (fără fn main)
cargo build                 # Compilează în mode debug (rapid, neoptimizat)
cargo build --release       # Compilează optimizat (lent, dar programul e rapid)
cargo run                   # Compilează + rulează (cel mai folosit!)
cargo run -- arg1 arg2      # Rulează cu argumente
cargo check                 # Verifică erorile FĂRĂ a compila complet (super rapid)
cargo test                  # Rulează toate testele
cargo fmt                   # Formatează codul automat (face-l frumos)
cargo clippy                # Sugestii de îmbunătățire (linter)
cargo doc --open            # Generează documentație și o deschide în browser
cargo add serde             # Adaugă dependență (crate extern)
cargo add tokio --features full  # Adaugă dependență cu features specifice
cargo update                # Actualizează dependențele la ultima versiune compatibilă
cargo clean                 # Șterge fișierele compilate (eliberează spațiu)
```

## Structura unui proiect

```
proiect/
├── Cargo.toml              # Configurare: nume, versiune, dependențe
├── Cargo.lock              # Versiuni exacte (generat automat, nu-l edita)
├── src/
│   ├── main.rs             # Punct de intrare (binary)
│   └── lib.rs              # Punct de intrare (library)
├── tests/                  # Teste de integrare
└── target/                 # Fișiere compilate (ignorat de git)
```

## Cargo.toml — exemplu

```toml
[package]
name = "proiect"
version = "0.1.0"
edition = "2021"            # Ediția Rust (lasă 2021)

[dependencies]
rand = "0.8"                # Crate extern cu versiune
serde = { version = "1.0", features = ["derive"] }  # Cu features
```

---

# 2. VARIABILE ȘI CONSTANTE

## Declarare

```rust
let x = 5;                 // Imutabilă — NU se poate schimba (implicit în Rust!)
let mut y = 10;             // Mutabilă — se POATE schimba
y = 20;                     // OK, e mut
// x = 6;                   // EROARE! x nu e mut

const MAX: u32 = 100_000;   // Constantă — trebuie tip explicit, SCREAMING_SNAKE_CASE
                             // Separatorul _ în numere e opțional, ajută la lizibilitate
```

## Shadowing — redeclarare cu let

```rust
let x = 5;                 // x este i32, valoare 5
let x = x + 1;             // x NOU, valoare 6 (cel vechi dispare)
let x = "text";            // x NOU, acum e &str! (poți schimba tipul cu shadowing)
```

## Diferența: mut vs shadowing

```rust
let mut a = 5;
a = 6;                      // Modifici ACEEAȘI variabilă, ACELAȘI tip

let b = 5;
let b = "cinci";            // Creezi o variabilă NOUĂ, poți schimba tipul
```

---

# 3. TIPURI DE DATE

## Scalare (o singură valoare)

```
ÎNTREGI CU SEMN (pot fi negativi):
  i8     →  -128 până la 127
  i16    →  -32.768 până la 32.767
  i32    →  -2.147.483.648 până la 2.147.483.647     ← IMPLICIT (cel mai folosit)
  i64    →  ±9.2 × 10¹⁸
  i128   →  ±1.7 × 10³⁸
  isize  →  depinde de platformă (32 sau 64 biți)

ÎNTREGI FĂRĂ SEMN (doar pozitivi):
  u8     →  0 până la 255                              ← folosit pentru bytes
  u16    →  0 până la 65.535
  u32    →  0 până la 4.294.967.295
  u64    →  0 până la 1.8 × 10¹⁹
  u128   →  0 până la 3.4 × 10³⁸
  usize  →  depinde de platformă                       ← folosit pentru indecși

VIRGULĂ MOBILĂ:
  f32    →  ~7 cifre de precizie
  f64    →  ~15 cifre de precizie                      ← IMPLICIT

BOOLEAN:
  bool   →  true sau false

CARACTER:
  char   →  un singur caracter Unicode (4 bytes!)      → 'a', 'ș', '🎮'
```

## Literale numerice

```rust
let decimal     = 1_000_000;     // _ ca separator vizual (ignorat de compilator)
let hex         = 0xFF;          // hexadecimal
let octal       = 0o77;          // octal
let binar       = 0b1010_1010;   // binar
let byte        = b'A';          // byte literal (u8) — doar caractere ASCII
```

## Conversie între tipuri (casting)

```rust
let x: i32 = 42;
let y: f64 = x as f64;          // i32 → f64 (sigur, nu pierzi date)
let z: u8  = x as u8;           // i32 → u8 (PERICULOS dacă x > 255! se trunchiază)

let c: char = 'A';
let cod: u8 = c as u8;          // char → u8 (codul ASCII: 65)
let inapoi: char = 65u8 as char; // u8 → char ('A')
```

## Compuse

```rust
// TUPLU — dimensiune fixă, tipuri diferite
let persoana: (&str, i32, bool) = ("Ana", 25, true);
let nume   = persoana.0;        // Acces prin index cu punct
let varsta = persoana.1;
let (n, v, a) = persoana;       // Destructurare

// ARRAY — dimensiune fixă, ACELAȘI tip
let numere: [i32; 5] = [1, 2, 3, 4, 5];
let zerouri = [0; 10];          // 10 elemente, toate 0
let primul = numere[0];         // Acces prin index (de la 0!)
let lungime = numere.len();     // 5
```

---

# 4. STRINGURI — cele 2 tipuri esențiale

## &str vs String

```
&str (string slice):
  - Text FIX, imutabil
  - Stocat în binarul programului sau e o "fereastră" spre un String
  - Nu poți modifica conținutul
  - Folosit pentru: parametri de funcții, text literal

String:
  - Text DINAMIC, pe heap
  - Poți adăuga, șterge, modifica
  - Deține datele (e proprietar)
  - Folosit când: construiești text, citești input, modifici conținut
```

## Creare și conversie

```rust
// &str — text literal
let salut: &str = "Bună ziua";

// String — text dinamic
let mut mesaj = String::new();                   // String gol
let mesaj = String::from("Salut");               // Din &str literal
let mesaj = "Salut".to_string();                 // Alternativă
let mesaj = format!("{} are {} ani", "Ana", 25); // Cu formatare

// Conversii
let s: String = "text".to_string();    // &str → String (copiază datele pe heap)
let r: &str = &s;                      // String → &str (doar o referință, zero cost)
let r: &str = s.as_str();              // Alternativă explicită
```

## Operații pe String

```rust
let mut s = String::from("Salut");

s.push(' ');                    // Adaugă UN caracter
s.push_str("lume!");            // Adaugă un &str
s.len();                        // Lungime în BYTES (nu caractere!)
s.chars().count();              // Lungime în CARACTERE (pentru UTF-8)
s.is_empty();                   // true dacă e gol
s.contains("lume");             // true dacă conține substringul
s.starts_with("Sal");           // true
s.ends_with("!");               // true
s.trim();                       // Elimină spațiile de la capete → &str
s.to_uppercase();               // "SALUT LUME!"
s.to_lowercase();               // "salut lume!"
s.replace("lume", "Rust");      // "Salut Rust!"
s.replacen("l", "L", 1);       // Înlocuiește doar prima apariție

// Concatenare
let a = String::from("Hello");
let b = String::from(" World");
let c = a + &b;                 // a se consumă! (nu mai poți folosi a)
let c = format!("{}{}", "Hello", " World");  // Sigur, nimeni nu se consumă

// Parcurgere
for c in s.chars() {            // Caracter cu caracter
    print!("{c} ");
}
for b in s.bytes() {            // Byte cu byte
    print!("{b} ");
}

// Împărțire
let cuvinte: Vec<&str> = "ana are mere".split(' ').collect();
let linii: Vec<&str> = "linia1\nlinia2".lines().collect();
```

---

# 5. CONTROLUL FLUXULUI

## if / else

```rust
let x = 5;

// Standard
if x > 10 {
    println!("mare");
} else if x > 0 {
    println!("pozitiv");
} else {
    println!("zero sau negativ");
}

// Ca expresie (returnează valoare) — ambele ramuri trebuie să aibă ACELAȘI tip
let categorie = if x > 0 { "pozitiv" } else { "nepozitiv" };
```

## match — pattern matching (super putere Rust!)

```rust
let x = 3;

match x {
    1 => println!("unu"),                    // Valoare exactă
    2 | 3 => println!("doi sau trei"),       // Mai multe valori cu |
    4..=10 => println!("între 4 și 10"),     // Range inclusiv
    _ => println!("altceva"),                // Wildcard — prinde tot restul (OBLIGATORIU)
}

// match ca expresie
let text = match x {
    1 => "unu",
    2 => "doi",
    _ => "altceva",
};

// match pe String/&str
let comanda = "start";
match comanda {
    "start" => println!("Pornesc..."),
    "stop" => println!("Opresc..."),
    alt => println!("Comandă necunoscută: {alt}"),  // Capturare în variabilă
}

// match cu gardă (condiție suplimentară)
let numar = 15;
match numar {
    n if n % 15 == 0 => println!("FizzBuzz"),
    n if n % 3 == 0 => println!("Fizz"),
    n if n % 5 == 0 => println!("Buzz"),
    n => println!("{n}"),
}
```

## if let — match simplificat pentru un singur caz

```rust
let optional: Option<i32> = Some(42);

// În loc de match complet:
if let Some(valoare) = optional {
    println!("Am găsit: {valoare}");
} else {
    println!("Nimic");
}
```

---

# 6. BUCLE

## loop — buclă infinită

```rust
let mut i = 0;
loop {
    i += 1;
    if i == 5 { break; }      // Ieșire din buclă
}

// loop cu valoare returnată
let rezultat = loop {
    i += 1;
    if i == 10 { break i * 2; }  // break cu valoare → rezultat = 20
};
```

## while

```rust
let mut n = 5;
while n > 0 {
    println!("{n}");
    n -= 1;
}
```

## for — cel mai folosit

```rust
// Range-uri
for i in 0..5 { }              // 0, 1, 2, 3, 4 (exclusiv 5)
for i in 0..=5 { }             // 0, 1, 2, 3, 4, 5 (inclusiv 5)
for i in (0..5).rev() { }      // 4, 3, 2, 1, 0 (inversat)

// Prin colecții
let v = vec![10, 20, 30];

for element in &v { }           // Împrumut imutabil (v rămâne valid)
for element in &mut v { }       // Împrumut mutabil (poți modifica elementele)
for element in v { }            // Consumă v! (nu mai poți folosi v după)

// Cu index
for (i, val) in v.iter().enumerate() {
    println!("{i}: {val}");
}

// Parcurgere HashMap
for (cheie, valoare) in &hmap {
    println!("{cheie}: {valoare}");
}
```

## Controlul buclelor

```rust
for i in 0..10 {
    if i == 3 { continue; }     // Sare la iterația următoare
    if i == 7 { break; }        // Oprește bucla complet
    println!("{i}");             // Afișează: 0, 1, 2, 4, 5, 6
}

// Etichete pentru bucle imbricate
'exterior: for i in 0..5 {
    for j in 0..5 {
        if i + j == 6 {
            break 'exterior;     // Iese din AMBELE bucle
        }
    }
}
```

---

# 7. FUNCȚII

## Sintaxă

```rust
// Funcție simplă
fn saluta() {
    println!("Salut!");
}

// Cu parametri (tipul e OBLIGATORIU)
fn saluta_pe(nume: &str) {
    println!("Salut, {nume}!");
}

// Cu valoare returnată
fn aduna(a: i32, b: i32) -> i32 {
    a + b                       // FĂRĂ ; → expresie returnată (idiom Rust)
}

// Cu return explicit (funcționează, dar nu e stilul preferat)
fn verifica(x: i32) -> bool {
    if x < 0 {
        return false;           // Return timpuriu — aici e OK să folosești return
    }
    x > 10                      // Ultima expresie — fără ; și fără return
}

// Funcție care nu returnează nimic (returnează () implicit — "unit type")
fn afiseaza(x: i32) {
    println!("{x}");
}

// Parametri mutabili
fn dubleaza(mut x: i32) -> i32 {   // mut pe parametru local (o copie)
    x *= 2;
    x
}

// Parametri referință
fn lungime(s: &str) -> usize {      // & = împrumut imutabil
    s.len()
}

fn adauga(s: &mut String) {          // &mut = împrumut mutabil
    s.push_str(" extra");
}
```

## Closures (funcții anonime / lambda)

```rust
let aduna = |a, b| a + b;                       // Tip inferat
let aduna: fn(i32, i32) -> i32 = |a, b| a + b;  // Tip explicit

println!("{}", aduna(3, 5));     // 8

// Closures captează variabile din context
let multiplicator = 3;
let inmulteste = |x| x * multiplicator;   // "captează" multiplicator
println!("{}", inmulteste(5));              // 15

// Closures cu mai multe linii
let proceseaza = |x: i32| -> i32 {
    let dublu = x * 2;
    dublu + 1
};
```

---

# 8. COLECȚII

## Vec<T> — vector dinamic

```rust
// Creare
let mut v: Vec<i32> = Vec::new();        // Gol, cu tip explicit
let mut v = vec![1, 2, 3, 4, 5];        // Cu macro vec!
let v = vec![0; 10];                     // 10 zerouri

// Adăugare / Ștergere
v.push(6);                               // Adaugă la final
v.pop();                                 // Scoate ultimul → Option<i32>
v.insert(0, 99);                         // Inserează la index (mută restul)
v.remove(0);                             // Șterge de la index (mută restul)
v.clear();                               // Golește tot

// Acces
v[0];                                    // Acces direct (PANICEAZĂ dacă index invalid!)
v.get(0);                                // Acces safe → Option<&i32>
v.first();                               // Primul element → Option<&i32>
v.last();                                // Ultimul element → Option<&i32>

// Informații
v.len();                                 // Câte elemente
v.is_empty();                            // E gol?
v.contains(&3);                          // Conține valoarea 3?
v.capacity();                            // Câte elemente poate ține fără realocare

// Transformări
v.sort();                                // Sortare crescătoare
v.sort_by(|a, b| b.cmp(a));             // Sortare descrescătoare
v.reverse();                             // Inversare ordine
v.dedup();                               // Elimină duplicate CONSECUTIVE (sortează înainte!)
v.retain(|&x| x > 3);                   // Păstrează doar elementele care satisfac condiția
v.truncate(3);                           // Păstrează doar primele 3 elemente

// Slice (fereastră spre vector)
let slice: &[i32] = &v[1..4];           // Elementele de la index 1, 2, 3
```

## Iteratori — lanțuri de operații pe colecții

```rust
let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// Fiecare metodă returnează un iterator NOU (nu modifică originalul)
// .collect() la final materializează rezultatul într-o colecție

let pare: Vec<i32> = v.iter()
    .filter(|&&x| x % 2 == 0)           // Păstrează doar ce satisface condiția
    .cloned()                            // Transformă &i32 → i32
    .collect();                          // Colectează în Vec → [2, 4, 6, 8, 10]

let duble: Vec<i32> = v.iter()
    .map(|&x| x * 2)                    // Transformă fiecare element
    .collect();                          // [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]

let suma: i32 = v.iter().sum();          // 55
let produs: i32 = v.iter().product();    // 3628800
let minim = v.iter().min();              // Some(1)
let maxim = v.iter().max();              // Some(10)
let numar = v.iter().count();            // 10

// Verificări
v.iter().any(|&x| x > 5);              // true (cel puțin unul > 5?)
v.iter().all(|&x| x > 0);              // true (TOATE > 0?)
v.iter().find(|&&x| x > 5);            // Some(6) — primul care satisface
v.iter().position(|&x| x == 5);        // Some(4) — indexul primei apariții

// Înlănțuire complexă: "suma pătratelor numerelor pare"
let rezultat: i32 = v.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * x)
    .sum();                              // 4+16+36+64+100 = 220

// zip — combină doi iteratori
let nume = vec!["Ana", "Mihai"];
let note = vec![95, 87];
let perechi: Vec<_> = nume.iter().zip(note.iter()).collect();
// [("Ana", 95), ("Mihai", 87)]
```

## HashMap<K, V> — dicționar cheie → valoare

```rust
use std::collections::HashMap;

// Creare
let mut hmap: HashMap<String, i32> = HashMap::new();
let mut hmap = HashMap::from([("Ana", 95), ("Mihai", 87)]);

// Inserare / Modificare
hmap.insert("Elena", 92);                // Inserează (sau suprascrie dacă există)
hmap.entry("Dan").or_insert(80);         // Inserează DOAR dacă NU există deja
hmap.entry("Ana").and_modify(|v| *v += 5); // Modifică DOAR dacă există

// Acces
hmap.get("Ana");                          // Option<&i32> → Some(&95)
hmap["Ana"];                              // Acces direct (PANICEAZĂ dacă nu există!)

// Ștergere
hmap.remove("Mihai");                     // Șterge intrarea

// Informații
hmap.len();                               // Câte perechi
hmap.contains_key("Ana");                 // Există cheia?

// Parcurgere
for (cheie, valoare) in &hmap {
    println!("{cheie}: {valoare}");
}
for cheie in hmap.keys() { }              // Doar cheile
for valoare in hmap.values() { }          // Doar valorile

// Pattern clasic: numărare frecvențe
let text = "ana are ana mere";
let mut frecvente = HashMap::new();
for cuvant in text.split_whitespace() {
    let contor = frecvente.entry(cuvant).or_insert(0);
    *contor += 1;
}
// {"ana": 2, "are": 1, "mere": 1}
```

---

# 9. STRUCTS — structuri de date custom

## Definiție și utilizare

```rust
struct Utilizator {
    nume: String,
    email: String,
    varsta: u32,
    activ: bool,
}

// Creare instanță (TOATE câmpurile trebuie specificate)
let user = Utilizator {
    nume: String::from("Ana"),
    email: String::from("ana@email.com"),
    varsta: 25,
    activ: true,
};

// Acces
println!("{}", user.nume);

// Modificare (instanța trebuie să fie mut)
let mut user = user;
user.varsta = 26;

// Prescurtare când variabila are același nume ca field-ul
let nume = String::from("Ana");
let varsta = 25;
let user = Utilizator {
    nume,                        // Echivalent cu: nume: nume,
    email: String::from("x"),
    varsta,                      // Echivalent cu: varsta: varsta,
    activ: true,
};

// Struct update syntax — copiază restul câmpurilor din alt struct
let user2 = Utilizator {
    email: String::from("alta@email.com"),
    ..user                       // Restul câmpurilor vin din user
};                               // ATENȚIE: user.nume s-a mutat în user2!
```

## Metode (impl)

```rust
struct Cerc {
    raza: f64,
}

impl Cerc {
    // Funcție asociată (constructor) — nu are self, se apelează cu ::
    fn new(raza: f64) -> Cerc {
        Cerc { raza }
    }

    // Metodă imutabilă — citește dar nu modifică
    fn arie(&self) -> f64 {
        std::f64::consts::PI * self.raza * self.raza
    }

    // Metodă mutabilă — poate modifica instanța
    fn redimensioneaza(&mut self, factor: f64) {
        self.raza *= factor;
    }

    // Metodă care consumă instanța (self fără &)
    fn descrie(self) -> String {
        format!("Cerc cu raza {}", self.raza)
        // După apel, instanța originală nu mai e validă!
    }
}

// Utilizare
let mut c = Cerc::new(5.0);     // Funcție asociată — cu ::
println!("{}", c.arie());        // Metodă — cu .
c.redimensioneaza(2.0);         // Metodă mutabilă
let desc = c.descrie();         // Consumă c — nu mai poți folosi c
```

## Tuple structs și unit structs

```rust
struct Culoare(u8, u8, u8);              // Tuple struct — câmpuri fără nume
let rosu = Culoare(255, 0, 0);
println!("R: {}", rosu.0);

struct Marker;                            // Unit struct — zero câmpuri
```

---

# 10. ENUMS — tipuri cu variante

## Definiție

```rust
// Simplu (fără date)
enum Directie {
    Nord,
    Sud,
    Est,
    Vest,
}

// Cu date atașate (fiecare variantă poate avea date DIFERITE!)
enum Mesaj {
    Quit,                                // Fără date
    Text(String),                        // Un String
    Muta { x: i32, y: i32 },            // Câmpuri cu nume
    Culoare(u8, u8, u8),                 // Tuple
}

let m = Mesaj::Text(String::from("Salut"));
let q = Mesaj::Quit;
let c = Mesaj::Culoare(255, 0, 0);
```

## Match pe enum (TREBUIE să acoperi TOATE variantele)

```rust
fn proceseaza(msg: Mesaj) {
    match msg {
        Mesaj::Quit => println!("Ieșire"),
        Mesaj::Text(t) => println!("Text: {t}"),
        Mesaj::Muta { x, y } => println!("Mută la ({x}, {y})"),
        Mesaj::Culoare(r, g, b) => println!("RGB({r},{g},{b})"),
    }
}
```

## Metode pe enum

```rust
impl Mesaj {
    fn este_quit(&self) -> bool {
        matches!(self, Mesaj::Quit)      // Macro matches! — super util
    }
}
```

---

# 11. OPTION ȘI RESULT — gestionarea absenței și erorilor

## Option<T> — "poate am o valoare, poate nu"

```rust
// Option e un enum: Some(valoare) sau None
let ceva: Option<i32> = Some(42);
let nimic: Option<i32> = None;

// Extragere valoare — metode principale
ceva.unwrap();                   // Returnează 42 (PANICEAZĂ pe None!)
ceva.unwrap_or(0);               // Returnează 42, sau 0 dacă e None
ceva.unwrap_or_default();        // Returnează 42, sau valoarea default a tipului
ceva.expect("Mesaj eroare");     // Ca unwrap, dar cu mesaj personalizat la panic

ceva.is_some();                  // true
ceva.is_none();                  // false

// Transformări
ceva.map(|x| x * 2);            // Some(84) — aplică funcția dacă e Some
ceva.and_then(|x| if x > 0 { Some(x) } else { None }); // Chain de Option-uri
ceva.filter(|&x| x > 0);       // Some(42) dacă condiția e true, altfel None

// Pattern matching (cea mai sigură metodă)
match ceva {
    Some(val) => println!("Am: {val}"),
    None => println!("N-am nimic"),
}

// if let (când te interesează doar Some)
if let Some(val) = ceva {
    println!("Am: {val}");
}
```

## Result<T, E> — "a mers bine, sau a eșuat cu eroare"

```rust
// Result e un enum: Ok(valoare) sau Err(eroare)
let bine: Result<i32, String> = Ok(42);
let rau: Result<i32, String> = Err("ceva a mers prost".to_string());

// Extragere valoare
bine.unwrap();                   // 42 (PANICEAZĂ pe Err!)
bine.unwrap_or(0);               // 42, sau 0 dacă e Err
bine.expect("Mesaj");            // Ca unwrap, cu mesaj personalizat

bine.is_ok();                    // true
bine.is_err();                   // false

// Operatorul ? — propagare elegantă (DOAR în funcții care returnează Result)
fn citeste(cale: &str) -> Result<String, std::io::Error> {
    let continut = std::fs::read_to_string(cale)?;  // Dacă e Err, returnează Err imediat
    Ok(continut)                                     // Dacă e Ok, continuă
}

// Pattern matching
match std::fs::read_to_string("fisier.txt") {
    Ok(continut) => println!("{continut}"),
    Err(eroare) => eprintln!("Eroare: {eroare}"),   // eprintln! → stderr
}
```

---

# 12. OWNERSHIP, BORROWING, REFERENCES

## Cele 3 reguli ale ownership

```
1. Fiecare valoare are EXACT UN proprietar (owner).
2. Proprietatea poate fi transferată (move), dar nu duplicată automat.
3. Când proprietarul iese din scope → valoarea e distrusă (drop).
```

## Move (transfer de proprietate)

```rust
let s1 = String::from("salut");
let s2 = s1;                     // s1 se MUTĂ în s2
// println!("{s1}");              // EROARE! s1 nu mai e valid

let s3 = s2.clone();             // Copie EXPLICITĂ (ambele rămân valide)
println!("{s2} {s3}");           // OK!

// EXCEPȚIE: tipurile simple (i32, f64, bool, char) se COPIAZĂ automat
let x = 5;
let y = x;                       // x se copiază (nu se mută)
println!("{x} {y}");             // OK! Ambele valide
```

## References (borrowing)

```rust
// Referință imutabilă (&T) — poți CITI, nu modifica
let s = String::from("salut");
let r1 = &s;                     // Împrumut imutabil
let r2 = &s;                     // Poți avea ORICÂTE referințe imutabile
println!("{r1} {r2}");           // OK!

// Referință mutabilă (&mut T) — poți MODIFICA
let mut s = String::from("salut");
let r = &mut s;                  // Împrumut mutabil
r.push_str(" lume");             // Modificare prin referință
println!("{r}");                 // "salut lume"

// REGULĂ DE AUR:
// Oricâte &T (imutabile) SAU exact una &mut T (mutabilă) — NICIODATĂ ambele!

let mut s = String::from("x");
let r1 = &s;                    // OK — referință imutabilă
// let r2 = &mut s;             // EROARE! Nu poți avea &mut cât timp există &
println!("{r1}");                // r1 se folosește ultima dată aici
let r2 = &mut s;                // ACUM e OK — r1 nu mai e folosit (NLL: Non-Lexical Lifetimes)
```

## References în funcții

```rust
// Împrumut — funcția citește dar nu consumă
fn lungime(s: &String) -> usize {      // & = referință
    s.len()                             // Nu preia ownership
}

// Împrumut mutabil — funcția poate modifica
fn adauga(s: &mut String, text: &str) {
    s.push_str(text);
}

// Fără referință — funcția preia ownership (consumă)
fn consuma(s: String) {
    println!("{s}");
}                                       // s e distrus aici

fn main() {
    let mut s = String::from("Salut");
    println!("{}", lungime(&s));         // Împrumut — s rămâne al nostru
    adauga(&mut s, " lume");            // Împrumut mutabil
    consuma(s);                          // Move — s nu mai e valid
    // println!("{s}");                  // EROARE!
}
```

---

# 13. TRAITS — comportamente partajate (ca interfețele)

## Definiție și implementare

```rust
trait Descrie {
    fn descriere(&self) -> String;               // Metodă obligatorie

    fn descriere_scurta(&self) -> String {        // Metodă cu implementare default
        format!("(descriere scurtă indisponibilă)")
    }
}

struct Masina {
    marca: String,
    an: u32,
}

impl Descrie for Masina {
    fn descriere(&self) -> String {
        format!("{} din {}", self.marca, self.an)
    }
    // descriere_scurta() folosește implementarea default (nu trebuie scrisă)
}
```

## Traits ca parametri de funcție

```rust
// Funcție care acceptă ORICE tip care implementează Descrie
fn afiseaza(item: &impl Descrie) {
    println!("{}", item.descriere());
}

// Echivalent (syntax mai explicit):
fn afiseaza<T: Descrie>(item: &T) {
    println!("{}", item.descriere());
}

// Mai multe traits:
fn proceseaza(item: &(impl Descrie + std::fmt::Display)) { }
```

## Traits standard utile

```rust
// Display — permite println!("{}", obiect)
use std::fmt;
impl fmt::Display for Masina {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.marca, self.an)
    }
}

// Debug — permite println!("{:?}", obiect)
#[derive(Debug)]                         // Derivare automată — super convenabil!
struct Punct {
    x: f64,
    y: f64,
}

// Clone + Copy
#[derive(Debug, Clone, Copy)]           // Poți deriva mai multe traits simultan
struct Dimensiune {
    latime: f64,
    inaltime: f64,
}

// PartialEq — permite comparare cu ==
#[derive(Debug, PartialEq)]
struct Culoare {
    r: u8, g: u8, b: u8,
}
```

## derive — cele mai comune

```rust
#[derive(
    Debug,       // Permite {:?} — afișare pentru debugging
    Clone,       // Permite .clone() — copiere explicită
    Copy,        // Permite copiere implicită (doar dacă toate câmpurile sunt Copy)
    PartialEq,   // Permite == și !=
    Eq,          // Egalitate totală (necesită PartialEq)
    Hash,        // Permite utilizarea ca cheie în HashMap
    PartialOrd,  // Permite <, >, <=, >=
    Ord,         // Ordine totală (necesită PartialOrd + Eq)
    Default,     // Permite Tip::default() — valori implicite (0, "", false)
)]
struct Exemplu {
    camp: i32,
}
```

---

# 14. GENERICS — cod care funcționează cu orice tip

```rust
// Funcție generică
fn cel_mai_mare<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

cel_mai_mare(5, 10);              // T = i32
cel_mai_mare(3.14, 2.71);        // T = f64
cel_mai_mare("abc", "def");      // T = &str

// Struct generic
struct Punct<T> {
    x: T,
    y: T,
}

let intreg = Punct { x: 5, y: 10 };        // Punct<i32>
let flotant = Punct { x: 1.5, y: 2.5 };    // Punct<f64>

// Implementare pe struct generic
impl<T: std::fmt::Display> Punct<T> {
    fn afiseaza(&self) {
        println!("({}, {})", self.x, self.y);
    }
}
```

---

# 15. ERROR HANDLING — gestionarea erorilor

## Ierarhia de la periculos la sigur

```rust
// ❌ NIVEL 1: panic! — oprește programul brutal
panic!("Eroare fatală!");                     // Doar pentru buguri ireversibile

// ⚠️ NIVEL 2: unwrap / expect — panic dacă e Err/None
let val = some_option.unwrap();               // Panic dacă e None
let val = some_result.expect("Mesaj clar");   // Panic cu mesaj dacă e Err

// ✅ NIVEL 3: match — tratare completă
match rezultat {
    Ok(val) => println!("Succes: {val}"),
    Err(e) => println!("Eroare: {e}"),
}

// ✅ NIVEL 4: operatorul ? — propagare elegantă
fn proceseaza() -> Result<String, std::io::Error> {
    let continut = std::fs::read_to_string("fisier.txt")?;
    Ok(continut.to_uppercase())
}

// ✅ NIVEL 5: unwrap_or / unwrap_or_else — valori de fallback
let val = some_option.unwrap_or(0);                      // Valoare default fixă
let val = some_result.unwrap_or_else(|e| {               // Calcul de fallback
    eprintln!("Eroare: {e}");
    String::new()
});
```

---

# 16. INPUT / OUTPUT

## Citire de la tastatură

```rust
use std::io;

// Citire string
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Eroare la citire");
let input = input.trim();                  // Elimină \n de la final

// Citire număr
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Eroare");
let numar: i32 = input.trim().parse().expect("Nu e un număr valid!");

// Citire număr — safe (fără panic)
let numar: Result<i32, _> = input.trim().parse();
match numar {
    Ok(n) => println!("Ai scris: {n}"),
    Err(_) => println!("Nu e un număr valid"),
}
```

## Citire / Scriere fișiere

```rust
use std::fs;

// Citire completă
let continut = fs::read_to_string("fisier.txt")?;

// Citire linie cu linie
for linie in continut.lines() {
    println!("{linie}");
}

// Scriere (suprascrie fișierul!)
fs::write("output.txt", "Conținut nou")?;

// Adăugare la fișier existent
use std::fs::OpenOptions;
use std::io::Write;
let mut f = OpenOptions::new().append(true).open("output.txt")?;
writeln!(f, "Linie nouă")?;

// Citire bytes (pentru fișiere binare)
let bytes: Vec<u8> = fs::read("imagine.png")?;
```

## Formatare output

```rust
println!("Text simplu");
println!("Salut, {}!", "Ana");           // Placeholder simplu
println!("{0} are {1} ani, {0} e cool", "Ana", 25);  // Index
println!("{nume} are {varsta} ani", nume = "Ana", varsta = 25);

// Formatare numere
println!("{:.2}", 3.14159);              // "3.14" — 2 zecimale
println!("{:>10}", "dreapta");           // "   dreapta" — aliniat dreapta, 10 caractere
println!("{:<10}", "stânga");            // "stânga    " — aliniat stânga
println!("{:^10}", "centru");            // "  centru  " — centrat
println!("{:0>5}", 42);                 // "00042" — padding cu zerouri
println!("{:#b}", 42);                   // "0b101010" — binar cu prefix
println!("{:#x}", 255);                  // "0xff" — hexadecimal
println!("{:#o}", 8);                    // "0o10" — octal

// Debug printing
println!("{:?}", vec![1, 2, 3]);         // "[1, 2, 3]" — Debug format
println!("{:#?}", vec![1, 2, 3]);        // Pretty-printed Debug

// Print fără newline
print!("fără newline");

// Print la stderr
eprintln!("Eroare: ceva a mers rău");
```

---

# 17. TESTARE

```rust
// Teste în același fișier (mod test)
#[cfg(test)]
mod tests {
    use super::*;                        // Importă totul din modulul părinte

    #[test]
    fn test_adunare() {
        assert_eq!(aduna(2, 3), 5);      // Verifică egalitate
    }

    #[test]
    fn test_pozitiv() {
        assert!(este_pozitiv(5));         // Verifică true
    }

    #[test]
    fn test_mesaj() {
        assert_ne!(mesaj(), "");          // Verifică non-egalitate
    }

    #[test]
    #[should_panic]                       // Testul TREBUIE să facă panic
    fn test_panica() {
        panic!("Asta e așteptat!");
    }

    #[test]
    fn test_result() -> Result<(), String> {
        let val = funcție_care_poate_eșua()?;
        assert_eq!(val, 42);
        Ok(())
    }
}
```

## Rulare teste

```bash
cargo test                    # Toate testele
cargo test numele_testului    # Un test specific
cargo test -- --nocapture     # Afișează println! din teste
cargo test -- --test-threads=1  # Rulează secvențial (nu în paralel)
```

---

# 18. OPERATORI COMPLETI

```
ARITMETICI              COMPARAȚIE           LOGICI
+   adunare             ==  egal             &&  ȘI (AND)
-   scădere             !=  diferit          ||  SAU (OR)
*   înmulțire           <   mai mic          !   NEGARE (NOT)
/   împărțire           >   mai mare
%   rest (modulo)       <=  mai mic/egal
                        >=  mai mare/egal

ATRIBUIRE               PE BIȚI              REFERINȚE
=   atribuire           &   AND              &x   referință imutabilă
+=  adună și atribuie   |   OR               &mut x  referință mutabilă
-=  scade               ^   XOR              *x   dereferențiere
*=  înmulțește          !   NOT (pe biți)
/=  împarte             <<  shift stânga
%=  rest                >>  shift dreapta

RANGE
0..5     exclusiv: 0, 1, 2, 3, 4
0..=5    inclusiv: 0, 1, 2, 3, 4, 5
..5      de la început până la 4
5..      de la 5 până la final
..       tot
```

---

# 19. TIPARE COMUNE (PATTERNS)

## Citire input într-o buclă

```rust
loop {
    println!("Scrie ceva (sau 'quit' pentru ieșire):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input == "quit" { break; }

    println!("Ai scris: {input}");
}
```

## Conversie safe de string la număr

```rust
let text = "42abc";

match text.parse::<i32>() {
    Ok(n) => println!("Număr: {n}"),
    Err(e) => println!("Nu e valid: {e}"),
}

// Sau cu unwrap_or
let numar: i32 = text.parse().unwrap_or(0);
```

## Colectare cu index

```rust
for (i, element) in colectie.iter().enumerate() {
    println!("Index {i}: {element}");
}
```

## Swap două valori

```rust
let mut a = 1;
let mut b = 2;
std::mem::swap(&mut a, &mut b);   // a=2, b=1

// Sau în Vec:
let mut v = vec![1, 2, 3];
v.swap(0, 2);                     // [3, 2, 1]
```

## Grupare elemente

```rust
// Vec de tuple-uri → HashMap
let perechi = vec![("fructe", "măr"), ("legume", "morcov"), ("fructe", "pară")];
let mut grupuri: HashMap<&str, Vec<&str>> = HashMap::new();
for (categorie, item) in perechi {
    grupuri.entry(categorie).or_insert_with(Vec::new).push(item);
}
// {"fructe": ["măr", "pară"], "legume": ["morcov"]}
```

---

# 20. CONVENȚII DE NUMIRE RUST

```
snake_case       → variabile, funcții, module, fișiere:    let my_var = 5;
                                                            fn my_function() {}
CamelCase        → structuri, enums, traits, tipuri:       struct MyStruct {}
                                                            enum MyEnum {}
SCREAMING_SNAKE  → constante:                               const MAX_SIZE: u32 = 100;
```

---

# 21. ATRIBUTE UTILE

```rust
#[derive(Debug, Clone)]         // Generare automată de trait-uri
#[allow(unused_variables)]      // Suprimă warning-ul pentru variabile nefolosite
#[allow(dead_code)]             // Suprimă warning-ul pentru cod nefolosit
#[cfg(test)]                    // Compilează DOAR la rularea testelor
#[test]                         // Marchează funcția ca test
#[should_panic]                 // Testul trebuie să facă panic pentru a trece
#[ignore]                       // Ignoră testul (rulează doar cu cargo test -- --ignored)
```

---

