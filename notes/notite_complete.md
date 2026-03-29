# Notițe Complete Rust

## Cuprins

1. [Variabile și Constante](#cap1)
2. [Tipuri de Date](#cap2)
3. [Statement-uri și Expresii](#cap3)
4. [Funcții](#cap4)
5. [Control Flow](#cap5)
6. [Ownership & Borrowing](#cap6)
7. [Referințe: &, &mut, *](#cap7)
8. [Durate de Viață (Lifetimes)](#cap8)
9. [Structuri de Date: struct și enum](#cap9)
10. [Generice, Option, Result, Vec](#cap10)
11. [Șiruri de Caractere](#cap11)
12. [OOP în Rust](#cap12)
13. [Colecții: Vec și HashMap](#cap13)
14. [Fisiere si I/O](#cap14)
15. [Closures si Iteratori](#cap15)

---

<h2 id="cap1">1. Variabile și Constante</h2>

### Constante

- Scrise **SNAKE_CASE** cu majuscule
- Tipul trebuie specificat explicit

```rust
const PI: f32 = 3.14159;
```

---

### `let` — Declararea variabilelor

- Rust deduce tipul în ~99% din cazuri
- Numele variabilelor sunt `snake_case`

```rust
fn main() {
    let x = 13;             // tip dedus: i32
    let x: f32 = 3.35;      // tip explicit
    let x;                  // declarare fara initializare
    x = 335;                // initializare mai tarziu
}
```

### `mut` — Variabile mutabile

- **Imutabil** (implicit): poate fi doar citit
- **Mutabil** (`mut`): poate fi modificat

```rust
fn main() {
    let mut var_mutabila = 5;
    var_mutabila = 10;      // OK
    let imutabil = 33;
    // imutabil = 99;       // EROARE
}
```

### `_` — Ignorarea variabilelor nefolosite

Rust avertizează dacă o variabilă nu e folosită. Prefixul `_` suprimă avertizamentul.

```rust
fn main() {
    let x: i32 = 5;
    let _y: u8 = 10;        // nu genereaza warning
}
```

### Shadowing — Mascarea variabilelor

O a doua variabilă cu același nume o ascunde pe cea anterioară.

```rust
fn main() {
    let x = 5;
    let x = x + 1;          // x = 6
    {
        let x = x * 2;
        println!("inner: {}", x); // 12
    }
    println!("outer: {}", x);    // 6
}
```

> **Shadowing vs mut:** Shadowing permite schimbarea tipului, `mut` nu.
> ```rust
> let spaces = "    ";
> let spaces = spaces.len();  // OK: &str -> usize prin shadowing
>
> let mut spaces = "    ";
> spaces = spaces.len();      // EROARE: tip nepotrivit
> ```

---

<h2 id="cap2">2. Tipuri de Date</h2>

### Tipuri numerice

| Categorie | Tipuri | Descriere |
|---|---|---|
| **Întregi fără semn** | `u8 u16 u32 u64 u128` | Numere naturale |
| **Întregi cu semn** | `i8 i16 i32 i64 i128` | Numere întregi |
| **Index / dimensiune** | `usize isize` | Indici și dimensiuni |
| **Virgulă mobilă** | `f32 f64` | Numere reale |

### Literali numerici

| Literal | Exemplu |
|---|---|
| Decimal | `92_200` |
| Hexadecimal | `0xff` |
| Octal | `0o77` |
| Binar | `0b1111_0000` |
| Byte | `b'A'` |

### Conversia numerică (`as`)

```rust
fn main() {
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;   // conversie explicita
    println!("{}", c);       // 20

    let t = true;
    println!("{}", t as u8); // 1
}
```

### Char, bool, unit ()

```rust
fn main() {
    // char: 4 bytes, Unicode
    let d = 'r';
    let ferris = '🦀';

    // bool: 1 byte
    let activ: bool = true;

    // unit (): tuplu gol, 0 bytes
    let nimic: () = ();
}
```

> **unit ()** — o funcție fără tip de retur returnează implicit `()`.
> ```rust
> fn make_nothing() -> () { return (); }
> fn make_nothing2() { }  // echivalent
> ```

### Tablouri (Arrays)

```rust
fn main() {
    let tablou: [i32; 5] = [1, 2, 3, 4, 5]; // [TIP; DIMENSIUNE]

    println!("{:?}", tablou);          // intreg tabloul
    println!("{}", tablou[0]);         // element individual

    for i in 0..tablou.len() {
        print!("{} ", tablou[i]);
    }
}
```

---

<h2 id="cap3">3. Statement-uri și Expresii</h2>

> **Statement** = o operație care se execută, nu returnează o valoare.
> **Expresie** = ceva care este evaluat și returnează o valoare.

```rust
fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1;   // statement
    let y = y + 5;   // statement
    x + y            // expresie returnata (fara `;`)
}
```

Blocurile `{}` sunt expresii:

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1       // valoarea blocului = 4
    };
    println!("y = {}", y); // 4

    let z = {
        2 * 5;      // `;` suprima expresia -> () este atribuit lui z
    };
    println!("z = {:?}", z); // ()
}
```

---

<h2 id="cap4">4. Funcții</h2>

- Denumite **snake_case**
- Fiecare parametru trebuie să aibă tipul specificat
- Valoarea returnată = ultima expresie din funcție (fără `;`)

```rust
fn add(x: i32, y: i32) -> i32 {
    return x + y;           // cu return explicit
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y                   // fara return (mod idiomatic)
}
```

### Returnarea mai multor valori (tuplu)

```rust
fn swap(x: i32, y: i32) -> (i32, i32) {
    (y, x)
}

fn main() {
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);
}
```

### Returnare fara valoare `()`

```rust
fn raporteaza(mesaj: &str) {
    println!("{}", mesaj);
    // returneaza implicit ()
}

fn curata(text: &mut String) -> () {
    *text = String::from("");
}
```

### Early return

```rust
fn plus_sau_minus(x: i32) -> i32 {
    if x > 5 {
        return x;   // iesire timpurie
    }
    x + 5
}
```

---

<h2 id="cap5">5. Control Flow</h2>

### if / else if / else

> Condițiile nu au nevoie de paranteze.

```rust
fn main() {
    let x = 42;
    if x < 42 {
        println!("mai mic");
    } else if x == 42 {
        println!("egal");
    } else {
        println!("mai mare");
    }
}
```

### if ca expresie ternară

```rust
let v = if x < 42 { -1 } else { 1 };
```

### loop — buclă infinită

```rust
fn main() {
    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 13 {
            break "am gasit 13"; // returneaza valoare din loop
        }
    };
    println!("v = {}", v);
}
```

### while

```rust
fn main() {
    let mut x = 0;
    while x != 42 {
        x += 3;
    }
}
```

### for

```rust
fn main() {
    for x in 0..5 {       // 0, 1, 2, 3, 4  (fara 5)
        print!("{} ", x);
    }
    for x in 0..=5 {      // 0, 1, 2, 3, 4, 5  (cu 5)
        print!("{} ", x);
    }
}
```

### match

> Exhaustiv — toate cazurile trebuie acoperite.

```rust
fn main() {
    let x = 42;
    match x {
        0 => println!("zero"),
        1 | 2 => println!("unu sau doi"),
        3..=9 => println!("intre 3 si 9"),
        matched_num @ 10..=100 => println!("gasit {}", matched_num),
        _ => println!("altceva"),
    }
}
```

### match ca expresie

```rust
let food = "hamburger";
let result = match food {
    "hotdog" => "este un hotdog",
    _ => "nu este un hotdog",
};
```

---

<h2 id="cap6">6. Ownership & Borrowing</h2>

### Stack vs Heap

**Stack (Stivă):**
- LIFO — "ultimul intrat, primul ieșit"
- Dimensiune fixă și cunoscută la compilare
- Alocare/dealocare rapidă
- Tipuri primitive (`i32`, `bool`, `char`, etc.)

**Heap:**
- Dimensiune dinamică, necunoscută la compilare
- Alocare mai lentă (sistemul caută spațiu liber)
- Accesat prin pointer stocat pe stack
- Tipuri complexe (`String`, `Vec`, etc.)

---

### Principiile Ownership

1. **Fiecare valoare are un proprietar** (o variabilă)
2. **O valoare poate avea un singur proprietar** la un moment dat
3. **Când proprietarul iese din scope, valoarea este eliberată** (drop)

```rust
{
    let s = String::from("buna"); // s intra in scope
    // folosesti s...
}   // s iese din scope -> drop automat
```

---

### Move (Mutare)

Tipurile alocate pe heap **nu se copiaza automat**. Atribuirea transferă ownership-ul.

```rust
let s1 = String::from("hello");
let s2 = s1;    // s1 este MUTAT in s2

// println!("{}", s1); // EROARE: s1 nu mai este valid
println!("{}", s2);    // OK
```

**De ce?** Un `String` pe stack conține: pointer, lungime, capacitate. Dacă s-ar copia doar aceasta (shallow copy), ambele variabile ar pointa la aceeasi memorie → double free la drop. Rust invalidează `s1` imediat → se numește **Move**.

---

### Copy (Copiere automată pe stivă)

Tipurile primitive implementează trăsătura `Copy` → se copiaza automat, fara move.

```rust
let x = 5;
let y = x;      // x este COPIAT, nu mutat
println!("{} {}", x, y); // ambele valide
```

**Tipuri Copy:** `i32`, `u32`, `f64`, `bool`, `char`, `(i32, i32)`, `&T` (referinte imutabile)
**Tipuri NON-Copy:** `String`, `Vec<T>`, orice alocat pe heap

---

### Clone (Deep Copy)

Dacă vrei explicit o copie completă a datelor de pe heap:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();   // copie completa (date + pointer)

println!("{} {}", s1, s2); // ambele valide
```

> ⚠️ `clone()` este costisitor — copiaza date de pe heap. Foloseste-l constient.

---

### Ownership și funcții

Transmiterea valorii la o funcție = **Move** sau **Copy** (la fel ca atribuirea).

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);           // s este MUTAT
    // println!("{}", s);         // EROARE: s invalid

    let x = 5;
    makes_copy(x);                // x este COPIAT
    println!("{}", x);            // OK: i32 este Copy
}

fn takes_ownership(s: String) {
    println!("{}", s);
}   // s este dropped aici

fn makes_copy(n: i32) {
    println!("{}", n);
}
```

Funcțiile pot **returna ownership**:

```rust
fn gives_ownership() -> String {
    String::from("hello")   // ownership transferat la apelant
}

fn main() {
    let s = gives_ownership();  // s devine proprietar
}
```

---

---

<h2 id="cap7">7. Referințe: &, &mut, *</h2>

### `&` — Referință imutabilă (Shared Reference)

- Permite **doar citirea** valorii
- Poți avea **oricâte** referințe `&` simultan
- Nu transferă ownership

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculeaza_lungimea(&s1);  // imprumuta, nu muta
    println!("'{}' are {} caractere", s1, len);
}

fn calculeaza_lungimea(s: &String) -> usize {
    s.len()
}   // s (referinta) este dropped; String-ul original ramas intact
```

### `&mut` — Referință mutabilă (Exclusive Reference)

- Permite **citirea și modificarea** valorii
- **O singura** referință mutabilă activă la un moment dat
- Nu poate coexista cu nicio altă referință (`&` sau `&mut`)

```rust
fn main() {
    let mut s = String::from("hello");
    adauga_lume(&mut s);
    println!("{}", s); // "hello world"
}

fn adauga_lume(s: &mut String) {
    s.push_str(" world");
}
```

### Regula Borrow Checker-ului

> **ORI** mai multe `&` (readonly) **ORI** un singur `&mut` (exclusive) — **niciodată ambele simultan.**

```rust
fn main() {
    let mut x = 10;
    let r1 = &x;      // OK
    // let r2 = &mut x; // EROARE: r1 e activ
    println!("{}", r1);
    // r1 e done aici
    let r2 = &mut x;  // OK acum
    *r2 += 5;
}
```

**De ce?** Previne **Data Races** — citire în timp ce altcineva scrie = comportament nedefinit.

### Analogie — Google Docs

- `&` = link "View Only" → oricâți cititori simultani, OK
- `&mut` = mod "Edit exclusiv" → dai afara pe toată lumea, editezi singur

### Rezumat `&` vs `&mut`

| Caracteristică | `&` | `&mut` |
|---|---|---|
| Drepturi | Doar citire | Citire + Scriere |
| Câte simultan | Nelimitate | Doar una |
| Coexistă cu | Alte `&` | Nimic |

---

### `*` — Dereferențierea

Dacă `&` creează o referință (adresa), `*` urmărește adresa pentru a accesa valoarea.

```rust
fn main() {
    let x = 10;
    let r = &x;

    println!("{}", *r);   // urmareste referinta -> 10
    println!("{}", r);    // Rust face auto-deref pentru println
}
```

**Modificare prin referință mutabilă:**

```rust
fn main() {
    let mut x = 10;
    let r = &mut x;
    *r = 20;              // scrie la adresa lui x
    println!("{}", x);    // 20
}
```

**Auto-deref cu `.`** — Rust adaugă automat `*` când accesezi câmpuri/metode:

```rust
let s = String::from("Salut");
let r = &s;
let len1 = (*r).len();  // manual
let len2 = r.len();     // Rust face auto-deref
```

### Trimiterea datelor prin referință — rezumat

| Tip | Semnătură funcție | Apel | Efect |
|---|---|---|---|
| **Move** | `fn f(x: String)` | `f(s)` | `s` invalid după apel |
| **Borrow** | `fn f(x: &String)` | `f(&s)` | `s` intact, citire |
| **Mut Borrow** | `fn f(x: &mut String)` | `f(&mut s)` | `s` intact, modificat |

> **Tipurile primitive** (`i32`, `bool`, etc.) se copiaza automat — nu ai nevoie de `&`.

---

<h2 id="cap8">8. Durate de Viață (Lifetimes)</h2>

> Lifetimes **nu schimbă** durata de viață a variabilelor — doar **explică** compilatorului relațiile dintre ele.

### De ce există?

Când o funcție primește mai multe referințe și returnează una, compilatorul nu știe pe care:

```rust
// EROARE: compilatorul nu stie daca returneaza x sau y
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

### Lifetime explicit `'a`

```rust
// Citim: "rezultatul traieste cel putin cat cel mai scurt dintre x si y"
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### Lifetime-uri multiple `'a`, `'b`

Când returni mereu din primul parametru, al doilea poate fi independent:

```rust
fn prima_parte<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    println!("y: {}", y);
    x  // rezultatul depinde doar de 'a
}
```

### Structuri cu referințe

Dacă un struct conține referințe, trebuie specificat lifetime-ul:

```rust
struct Citat<'a> {
    parte: &'a str,  // Citatul nu poate trai mai mult decat sursa
}
```

### `'static` — Durată de viață statică

Date care trăiesc pe toată durata programului.

**1. String literals** — hardcodate în binar:

```rust
let s: &'static str = "Salut"; // mereu valid
```

**2. Variabile globale:**

```rust
static VERSIUNE: &str = "1.0";
```

**3. Trait bound `T: 'static`** — (folosit la thread-uri):
- Nu "trăiește veșnic", ci "nu conține referințe temporare"
- Un `String` (Owned) satisface `'static` chiar dacă nu e literal

```rust
// EROARE: nu poti returna referinta la variabila locala
fn gresit() -> &'static str {
    let s = String::from("Salut");
    &s  // s moare la sfarsitul functiei!
}
```

### Cât de des folosești lifetimes?

- **Cod aplicatie** (web, CLI): **rar** — Rust deduce singur in cazuri simple
- **Biblioteci / parsere**: **des** — optimizare zero-copy
- **Sfat pentru incepatori**: dacă compilatorul cere lifetime, întreabă-te: "Pot folosi `String` în loc de `&str`?" — clonarea simplifică viața.

---

<h2 id="cap9">9. Structuri de Date: struct și enum</h2>

### Struct

> Un `struct` grupează mai multe câmpuri (valori asociate) într-un singur tip.
> Folosim `String` (nu `&str`) în struct-uri ca fiecare instanță să dețină propriile date.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

**Creare instanță:**

```rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@email.com"),
        sign_in_count: 1,
    };
    user1.active = false;   // instanta trebuie sa fie mut
}
```

**Constructor (functie care returneaza instanta):**

```rust
fn creaza_user(username: String, email: String) -> User {
    User {
        active: true,
        username,   // shorthand: echivalent cu username: username
        email,
        sign_in_count: 1,
    }
}
```

**Initializare cu campuri din alta instanta:**

```rust
let user2 = User {
    email: String::from("altul@email.com"),
    ..user1     // restul campurilor din user1 (MOVE pentru non-Copy)
};
```

---

### `impl` — Metode și funcții asociate

`impl` adaugă comportament unui struct.

| Tip | Semnătură | Apelat cu |
|---|---|---|
| Funcție asociată (constructor) | `fn new(...) -> Self` | `Struct::new()` |
| Metodă de citire | `fn f(&self)` | `instanta.f()` |
| Metodă de modificare | `fn f(&mut self)` | `instanta.f()` |

```rust
struct Patrat {
    latime: u32,
    inaltime: u32,
}

impl Patrat {
    // Constructor
    fn new(latime: u32, inaltime: u32) -> Self {
        Self { latime, inaltime }
    }

    // Metoda de citire
    fn aria(&self) -> u32 {
        self.latime * self.inaltime
    }

    // Metoda cu parametru extra
    fn poate_contine(&self, alt: &Patrat) -> bool {
        self.latime > alt.latime && self.inaltime > alt.inaltime
    }

    // Metoda de modificare
    fn largeste(&mut self, factor: u32) {
        self.latime *= factor;
        self.inaltime *= factor;
    }
}

fn main() {
    let mut p1 = Patrat::new(30, 20);
    let p2 = Patrat::new(10, 5);
    println!("Aria: {}", p1.aria());
    println!("Poate contine p2: {}", p1.poate_contine(&p2));
    p1.largeste(2);
}
```

---

### Struct Tuplu

Struct fără nume de câmpuri — accesate prin index.

```rust
struct Culoare(i32, i32, i32);
struct Punct(f64, f64);

fn main() {
    let negru = Culoare(0, 0, 0);
    let origine = Punct(0.0, 0.0);
    println!("R={}", negru.0);
    println!("x={} y={}", origine.0, origine.1);
}
```

**Shortcut initializare din alt struct (tuple struct):**

```rust
let ts2 = TupleStruct { 0: 1, ..ts };
```

**Deconstructie:**

```rust
let Culoare(r, g, b) = negru;
println!("{} {} {}", r, g, b);
```

---

### Enum

```rust
#[allow(dead_code)]
enum Specii {
    Crab,
    Caracatita,
    Peste,
    Scoica,
}

fn main() {
    let animal = Specii::Crab;
    match animal {
        Specii::Crab => println!("e crab"),
        Specii::Caracatita => println!("e caracatita"),
        Specii::Peste => println!("e peste"),
        Specii::Scoica => println!("e scoica"),
    }
}
```

**Enum cu date asociate (Tagged Union):**

```rust
enum Arma {
    Ghiara(i32, Marime),
    Otrava(TipOtrava),
    None,
}

// Folosire in match:
match personaj.arma {
    Arma::Ghiara(nr, Marime::Mare) => println!("{} ghiare mari", nr),
    Arma::Otrava(_) => println!("otrava"),
    Arma::None => println!("fara arma"),
}
```

> **Memorie:** Un enum aloca spațiu egal cu cel mai mare variant al său + un discriminant (index).

---

<h2 id="cap10">10. Generice, Option, Result, Vec</h2>

### Tipuri generice

Definesc parțial un tip, compilatorul completează la folosire:

```rust
struct BagOfHolding<T> {
    item: T,
}

fn main() {
    let i32_bag = BagOfHolding::<i32> { item: 42 };  // explicit (turbofish)
    let float_bag = BagOfHolding { item: 3.14 };      // dedus automat
}
```

---

### `Option<T>` — Valori care pot lipsi

```rust
enum Option<T> {
    Some(T),
    None,
}
```

```rust
fn main() {
    let cu_valoare: Option<i32> = Some(42);
    let fara_valoare: Option<i32> = None;

    match cu_valoare {
        Some(v) => println!("gasit: {}", v),
        None => println!("nimic"),
    }

    // Scurtaturi utile:
    println!("{}", cu_valoare.is_some()); // true
    println!("{}", fara_valoare.is_none()); // true
}
```

---

### `Result<T, E>` — Operații care pot eșua

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

```rust
fn executa(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("numar gresit"))
    }
}

fn main() {
    match executa(12) {
        Ok(v) => println!("succes: {}", v),
        Err(e) => println!("eroare: {}", e),
    }
}
```

**Operatorul `?` — gestionare elegantă a erorilor:**

```rust
fn main() -> Result<(), String> {
    let v = executa(42)?;   // daca Err -> return Err automat
    println!("gasit {}", v);
    Ok(())
}
```

**`unwrap()` — rapid dar periculos:**

```rust
let v = executa(42).unwrap(); // panic! daca e Err
```

> Foloseste `match` sau `?` in loc de `unwrap` in cod de productie.

---

### `Vec<T>` — Liste dinamice

```rust
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // Sau mai rapid cu macro:
    let v2 = vec![10, 20, 30];

    for elem in v2.iter() {
        println!("{}", elem);
    }

    // Vec de Strings:
    let cuvinte = vec![String::from("salut"), String::from("lume")];
    for c in cuvinte.iter() {
        println!("{}", c);
    }
}
```

> **Memorie:** `Vec` este un struct pe stack care conține pointer + lungime + capacitate. Datele reale sunt pe heap. Când se umple, se realocă automat mai mare.

---

<h2 id="cap11">11. Șiruri de Caractere</h2>

### Două tipuri principale

| Tip | Stocat | Mutabil | Ownership |
|---|---|---|---|
| `&str` (string slice) | date statice / heap | NU | Referință |
| `String` | heap | DA | Deține datele |

```rust
let literal: &'static str = "salut";       // stocat in binar
let dinamic: String = String::from("salut"); // alocat pe heap
```

### Secvențe escape

```
\n  linie noua
\r  carriage return
\t  tab
\\  backslash
\'  apostrof
\"  ghilimele
\0  null
```

### Operații utile pe `&str`

```rust
fn main() {
    let a = "salut 🦀";
    println!("{}", a.len());           // 10 (bytes, nu caractere!)
    println!("{}", a.starts_with("salut")); // true
    println!("{}", a.is_empty());     // false

    let prim_cuvant = &a[0..5];       // "salut"
    let emoji = &a[6..10];            // "🦀"
    println!("{} {}", prim_cuvant, emoji);
}
```

### Construire șiruri

```rust
// concat si join
let concatenat = ["a1", "a2", "a3"].concat();   // "a1a2a3"
let joinuit = ["b1", "b2", "b3"].join(", ");    // "b1, b2, b3"

// format! (cel mai folosit)
let s = format!("Hello {}!", "world");

// push_str pe String mutabil
let mut s = String::from("Hello");
s.push_str(", world!");
```

### String pe mai multe linii

```rust
let haiku = "
    Scriu si scriu
    pe mai multe randuri
";

// \ la sfarsit suprim newline-ul:
println!("Salutare \
         lume"); // "Salutare lume"
```

### Raw strings

```rust
let json = r#"{"cheie": "valoare", "nr": 42}"#;
```

### `.collect::<String>()` — asamblarea iteratorilor în text

Metoda `.collect()` ia o colecție de elemente (de obicei caractere sau șiruri) și le asamblează într-un singur obiect.

**Analogie:** Imaginează-ți că ai piesele unui echipament dezmembrat (fiecare piesă = un caracter). `.collect()` este mecanicul care le asamblează înapoi într-un obiect întreg.

```rust
// Transformăm în iterator, filtrăm, și colectăm înapoi în String
let nume = "Robert";
let fara_vocale: String = nume.chars()
    .filter(|c| !"aeiouAEIOU".contains(*c))
    .collect();

println!("{}", fara_vocale); // "Rbrt"
```

#### De ce avem nevoie de `::<String>`? — Turbofish

`.collect()` este extrem de versatilă: poate produce un `Vec`, un `HashMap`, un `String` etc. Deoarece Rust nu știe ce vrei la final, trebuie să îi specifici tipul. Sintaxa `::<>` se numește **turbofish**:

```rust
// Varianta turbofish:
let inversat = "Salut".chars().rev().collect::<String>();

// Varianta cu tip explicit pe variabilă (echivalent):
let inversat: String = "Salut".chars().rev().collect();
```

> **Sfat:** Dacă ai deja tipul specificat pe variabilă, nu mai e nevoie de turbofish.

#### Exemple comune

| Sursă | Cod | Rezultat |
|---|---|---|
| Vector de caractere | `vec!['a', 'b', 'c'].into_iter().collect::<String>()` | `"abc"` |
| Inversare text | `"Salut".chars().rev().collect::<String>()` | `"tulaS"` |
| Vector de `&str` | `vec!["Asta", "e", "text"].into_iter().collect::<String>()` | `"Astaetext"` |

#### `.collect()` vs `.join()`

- **`.join("")`** — mai rapid și clar când vrei să lipești un `Vec<&str>` fără transformări
- **`.collect()`** — indispensabil când faci transformări la nivel de caracter (`.map()`, `.filter()`)

```rust
// join — simplu, fara transformari
let cuvinte = vec!["Salut", "lume"];
let text = cuvinte.join(" "); // "Salut lume"

// collect — cu transformare
let doar_mari: String = "salut lume".chars()
    .map(|c| c.to_uppercase().next().unwrap())
    .collect();
// "SALUT LUME"
```

### `.collect()` pentru `HashMap` — cu `.zip()`

```rust
use std::collections::HashMap;

let fructe = vec!["Mere", "Pere", "Banane"];
let cantitati = vec![10, 5, 12];

let inventar: HashMap<_, _> = fructe.into_iter()
    .zip(cantitati.into_iter())
    .collect();
// {"Mere": 10, "Pere": 5, "Banane": 12}
```

> `HashMap<_, _>` — underscore-urile ii spun compilatorului "deduce tu tipurile".

### Rezumat: ce poate produce `.collect()`

| Tip țintă | Sursa iteratorului | Comportament |
|---|---|---|
| `String` | `char` / `&str` | Asamblează text |
| `Vec<T>` | orice `T` | Colectează într-un vector |
| `HashMap<K, V>` | tupluri `(K, V)` | Construiește hartă |
| `Result<Vec<T>, E>` | `Result<T, E>` | Ok dacă toate reușesc, Err la prima eroare |

---

<h2 id="cap12">12. OOP în Rust</h2>

Rust **nu are clase și nu are moștenire**. Echivalentele sunt:

| OOP Clasic | Rust |
|---|---|
| Class | `struct` (date) + `impl` (metode) |
| Constructor | Funcție asociată (convenție: `new`) |
| Interface | `trait` |
| Inheritance | **Nu există** — se folosesc `trait`-uri + compoziție |
| Polymorphism | Generics (static) sau `dyn Trait` (dinamic) |
| Destructor | Trait-ul `Drop` (automat la end of scope) |

### Encapsulare cu `pub`

```rust
pub struct ContBancar {
    titular: String,    // privat (nevazut din afara modulului)
    sold: f64,          // privat
}

impl ContBancar {
    pub fn new(nume: String, suma: f64) -> Self {
        Self { titular: nume, sold: suma }
    }

    pub fn arata_sold(&self) {
        println!("{}: {}", self.titular, self.sold);
    }

    pub fn depune(&mut self, suma: f64) {
        self.sold += suma;
    }
}
```

### Traits (Interfețe)

```rust
trait Animal {
    fn scoate_sunet(&self);
}

struct Caine { nume: String }
struct Pisica { nume: String }

impl Animal for Caine {
    fn scoate_sunet(&self) { println!("{}: Ham!", self.nume); }
}

impl Animal for Pisica {
    fn scoate_sunet(&self) { println!("{}: Miau!", self.nume); }
}
```

### Polimorfism Static (Generics) — performant

Compilatorul generează o versiune pentru fiecare tip concret:

```rust
fn fa_galagie<T: Animal>(animal: T) {
    animal.scoate_sunet();
}
```

### Polimorfism Dinamic (`dyn`) — flexibil

Util când vrei o colecție mixtă de tipuri:

```rust
fn main() {
    let animale: Vec<Box<dyn Animal>> = vec![
        Box::new(Caine { nume: String::from("Azorel") }),
        Box::new(Pisica { nume: String::from("Tom") }),
    ];

    for a in &animale {
        a.scoate_sunet(); // Rust decide la runtime ce metoda apeleaza
    }
}
```

> `Box<T>` = pointer cu ownership pe heap. Are dimensiune fixă, deci putem pune tipuri diferite în același `Vec`.

### Compoziție în loc de moștenire

In loc de `class Caine extends Animal`, in Rust:

```rust
struct Animal {
    nume: String,
    hp: i32,
}

struct Caine {
    baza: Animal,   // "mostenire" prin compozitie
    rasa: String,
}

impl Caine {
    fn latra(&self) {
        println!("{} latra!", self.baza.nume);
    }
}
```

### Trait-uri standard utile

- `Display` — pentru `println!("{}", ...)`
- `Debug` — pentru `println!("{:?}", ...)`
- `Clone` — pentru `.clone()`
- `Copy` — copiere automata
- `Drop` — cleanup la sfarsit de scope

---

<h2 id="cap13">13. Colecții: Vec și HashMap</h2>

```rust
use std::collections::HashMap;  // Vec e in prelude, HashMap nu
```

### Vec\<T\> — lista dinamica

```rust
let mut v: Vec<i32> = Vec::new();
let v2 = vec![1, 2, 3];          // macro cu valori initiale

v.push(10);                       // adauga la sfarsit
v.pop()                           // scoate ultimul → Option<T>
v.remove(0)                       // scoate de la index → T
v[0]                              // acces direct (panic daca invalid)
v.get(0)                          // acces sigur → Option<&T>
v.len()
v.is_empty()

for x in &v { }
for (i, x) in v.iter().enumerate() { }
```

### HashMap\<K, V\> — dictionar cheie→valoare

```rust
let mut hp: HashMap<String, i32> = HashMap::new();

hp.insert("Arthas".to_string(), 100);
hp.get("Arthas")                          // Option<&V>
hp.contains_key("Arthas")                // bool
hp.remove("Arthas")                       // Option<V>

// Insereaza doar daca cheia nu exista
hp.entry("Zara".to_string()).or_insert(40);

for (cheie, valoare) in &hp { }
for (i, (cheie, valoare)) in hp.iter().enumerate() { }
```

> HashMap-ul **nu e ordonat** — ordinea iterarii nu e garantata.

---

<h2 id="cap14">14. Fisiere si I/O</h2>

```rust
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};
```

### Scriere simpla

```rust
fs::write("fisier.txt", "continut").expect("Eroare");
// Suprascrie daca fisierul exista deja
```

### Citire simpla

```rust
let text = fs::read_to_string("fisier.txt").expect("Eroare");
```

### Citire linie cu linie

```rust
let fisier = fs::File::open("fisier.txt").expect("Eroare");
let reader = io::BufReader::new(fisier);

for (i, linie) in reader.lines().enumerate() {
    let linie = linie.expect("Eroare");
    println!("Linie {}: {}", i + 1, linie);
}
```

### Adaugare la sfarsit

```rust
let mut f = OpenOptions::new()
    .append(true)
    .open("fisier.txt")
    .expect("Eroare");

writeln!(f, "linie noua").expect("Eroare");
```

### Gestionare erori cu `?`

```rust
fn citeste(cale: &str) -> Result<String, io::Error> {
    let continut = fs::read_to_string(cale)?;
    Ok(continut)
}
```

### Alte operatii utile

```rust
fs::remove_file("fisier.txt").ok();               // sterge
std::path::Path::new("fisier.txt").exists()       // bool
```

### Rezumat

| Operatie | Functie | Observatie |
|----------|---------|------------|
| Scriere | `fs::write(path, content)` | Suprascrie |
| Citire tot | `fs::read_to_string(path)` | In String |
| Citire linie | `BufReader` + `.lines()` | Eficient |
| Adaugare | `OpenOptions::new().append(true)` | Nu suprascrie |
| Stergere | `fs::remove_file(path)` | `Result` |

---

<h2 id="cap15">15. Closures si Iteratori</h2>

### Closures

Un **closure** este o functie anonima (fara `fn` si fara nume) care poate captura variabile din contextul in care a fost creata.

```rust
// Functie clasica
fn aduna_cinci(x: i32) -> i32 { x + 5 }

// Closure echivalent — tipurile se deduc automat
let aduna_cinci = |x| x + 5;
```

**Capturarea mediului** — superputerea fata de `fn`:

```rust
let taxa = 10;
let calculeaza = |pret| pret + taxa;  // captureaza `taxa` din context
// fn clasica nu poate vedea `taxa` fara sa fie parametru
println!("{}", calculeaza(50));        // 60
```

**Closure mutabil** — daca modifica o variabila din exterior:

```rust
let mut contor = 0;
let mut incrementeaza = || { contor += 1; };  // mut pe closure SI pe variabila
incrementeaza(); // contor = 1
incrementeaza(); // contor = 2
```

**Closure ca argument** (cazul cel mai frecvent):

```rust
let v = vec![1, 2, 3, 4, 5];
let duble: Vec<i32> = v.iter().map(|&x| x * 2).collect();
```

---

### Iteratori

Un **iterator** produce valori una cate una, la cerere. Metodele de iterare primesc closures.

**Regula de aur (lazy evaluation)**: Adaptoarele nu fac nimic singure — consumatorul porneste banda.

```rust
let plan = v.iter().map(|x| x * 2).filter(|x| x > &2);  // nimic nu se executa
let result: Vec<i32> = plan.collect();                    // ACUM ruleaza totul
```

**Cele 3 moduri de intrare:**

```rust
v.iter()        // &T      — imprumut, v ramane valid
v.iter_mut()    // &mut T  — modifica in-place
v.into_iter()   // T       — consuma v (v nu mai exista dupa)
```

**Straturi de referinta (`&T` vs `&&T`):**

`.iter()` da `&T`. Dar `.map()` si `.filter()` trateaza referinta diferit:

```
Vec<i32> → .iter() → &i32
                       ├── .map(|x| ...)     → x = &i32   (direct)
                       └── .filter(|x| ...)  → x = &&i32  (referinta la referinta)
```

`.filter()` ia referinta in plus pentru ca doar inspecteaza elementul, nu il consuma. Solutii:

```rust
.filter(|&&x| x > 2)     // destructurare in pattern
.filter(|x| **x > 2)     // dereferentiere cu *
.filter(|x| *x > 2)      // auto-deref (functioneaza la comparatii)
```

**Adaptoare (lazy):**

```rust
.map(|&x| x * 2)          // transforma fiecare element
.filter(|&&x| x % 2 == 0) // pastreaza ce trece conditia
.take(3)                   // primele n elemente
.skip(2)                   // sare primele n elemente
.enumerate()               // adauga index: (usize, &T)
.zip(alt_iter)             // combina in perechi: (T, U)
.chain(alt_iter)           // concateneaza doi iteratori
.inspect(|x| println!("{}", x))  // debug — vede elementul fara sa-l modifice
```

**Closure cu bloc `{}`** — mai multe operatii intr-un closure:

```rust
.map(|x| {
    println!("Procesez: {}", x);
    x + taxa  // ultima expresie fara ; = valoarea returnata
})
```

**Consumatori (eager):**

```rust
.collect::<Vec<_>>()       // aduna intr-o colectie
.count()                   // usize
.sum::<i32>()              // T
.any(|&x| x > 5)           // bool — cel putin unul
.all(|&x| x > 0)           // bool — toti
.find(|&&x| x > 3)         // Option<&T> — primul potrivit
.max() / .min()            // Option<&T>
```

**Inlantuire:**

```rust
let suma_patrate_pare: i32 = v.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * x)
    .sum();
```

**Range-uri ca iteratori:**

```rust
let suma: i32         = (1..=100).sum();
let patrate: Vec<i32> = (1..=5).map(|x| x * x).collect();
```

**Stil idiomatic — iterator vs `for`:**

```rust
// FOR — cu mut, imperativ
let mut result = Vec::new();
for &x in &v { if x % 2 == 0 { result.push(x * x); } }

// ITERATOR — fara mut, declarativ, mai sigur
let result: Vec<i32> = v.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * x)
    .collect();
```
| Existenta | `Path::new(path).exists()` | `bool` |
- `Iterator` — pentru bucle `for`