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

### Deconstructia variabilelor

```rust
fn main() {
    let (a, mut b): (bool, bool) = (true, false);
    b = true;
    assert_eq!(a, b);
}
```

### Atribuirea deconstructivă

```rust
struct Structura { e: u8 }

fn main() {
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Structura { e, .. } = Structura { e: 5 };
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}
```

### Practică — Variabile

```rust
// 1. O variabila trebuie initializata inainte de a fi folosita
fn main() {
    let x: i32 = 5;
    assert_eq!(x, 5);
}

// 2. Scope: y nu e vizibil in afara acoladelor
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("inner: x={} y={}", x, y);
    }
    println!("outer: x={}", x);
}

// 3. Shadowing
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }
    assert_eq!(x, 5);
    let x = 42;
    println!("{}", x); // 42
}

// 4. Deconstructie tuplu
fn main() {
    let (mut x, y) = (1, 2);
    x += 2;
    assert_eq!(x, 3);
    assert_eq!(y, 2);
}
```

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

### Practică — Tipuri de date

```rust
// 1. Tipuri mixte nu se pot atribui direct
fn main() {
    let x = 5;              // i32 (default)
    let mut y: u32 = 5;
    // y = x;              // EROARE: i32 != u32
    y = x as u32;          // OK cu conversie
}

// 2. Conversie u8 -> u16
fn main() {
    let v: u16 = 38_u8 as u16;
    println!("{}", v);
}

// 3. Limitele tipurilor
fn main() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
}

// 4. Literali mixti
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    // 1024 + 255 + 63 + 255 = 1597
    assert!(v == 1597);
}

// 5. char ocupa 4 bytes
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);
    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);
}

// 6. unit ocupa 0 bytes
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);
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

### Practică — Expresii

```rust
// 1. Bloc ca expresie
fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        x           // returneaza 3
    };
    assert_eq!(v, 3);
}

// 2. Expresie in linie
fn main() {
    let v = { let x = 3; x };
    assert!(v == 3);
}

// 3. Functie ca expresie
fn sum(x: i32, y: i32) -> i32 { x + y }
fn main() {
    assert_eq!(sum(1, 2), 3);
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

### Funcție divergentă `!`

Nu returnează niciodată (crash sau bucla infinita):

```rust
fn dead_end() -> ! {
    panic!("Crash!");
}

fn bucla_infinita() -> ! {
    loop {}
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

### Practică — Funcții

```rust
// 1. Functie simpla
fn sum(x: i32, y: i32) -> i32 { x + y }
fn main() {
    assert_eq!(sum(1, 2), 3);
}

// 2. Return tip static string
fn print() -> &'static str { "Success" }
fn main() {
    println!("{}", print());
}

// 3. Functie divergenta (nu lasa println! sa ruleze)
fn never_return() -> ! {
    loop {}
}
fn main() {
    never_return();
    println!("Nu ajunge aici niciodata");
}
```

### Demo din lectie — `src/bin/02_functions.rs`

Analogie RPG: functiile sunt abilitatile personajului.

```rust
// cargo run --bin 02_functions

fn patrat(n: i32) -> i32 { n * n }              // ability: critical strike

fn saluta(nume: &String) {                       // nu preia ownership
    println!("Salutare, {}!", nume);
}

fn max_doua(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }                    // if ca expresie
}
```

### Exercitii rezolvate — `exercitii/src/bin/ex_02_functions.rs`

```rust
// 1. Critical Strike — damage la patrat (return implicit)
fn patrat(n: i32) -> i32 { n * n }

// 2. Anunta eroul in arena (nu returneaza nimic, imprumut imutabil)
fn saluta(nume: &String) { println!("Salutare,{}!", nume); }

// 3. Atacantul mai puternic (if ca expresie)
fn max_doua(a: i32, b: i32) -> i32 { if a > b { a } else { b } }

// 4. Damage total din trei atacuri
fn suma_trei(a: i32, b: i32, c: i32) -> i32 { a + b + c }

// 5. Eroul e in viata? (if ca expresie, fara return explicit)
fn este_pozitiv(n: i32) -> bool { if n > 0 { true } else { false } }

// 6. Clasifica inamicul dupa nivel (return timpuriu)
fn clasa_numar(n: i32) -> &'static str {
    if n < 0 { return "negativ"; }
    if n == 0 { return "zero"; }
    if n <= 9 { return "mic"; }
    "mare"
}

// 7. Lungimea numelui fara ownership (imprumut imutabil)
fn lungime_imprumut(s: &String) -> usize { s.len() }

// 8. Blacksmith adauga prefix, preia si returneaza ownership
fn adauga_prefix(s: String) -> String { format!(">> {}", s) }

// 9. XP total de la nivelul 1 la n (for + acumulator)
fn suma_pana_la(n: u32) -> u32 {
    let mut suma = 0;
    for i in 1..=n { suma += i; }
    suma
}

// 10. Damage absolut (fara semn negativ)
fn abs_valoare(n: i32) -> i32 { if n < 0 { -n } else { n } }

// 11. Combo multiplier: factorial (return timpuriu pentru n=0)
fn factorial(n: u64) -> u64 {
    if n == 0 { return 1; }
    let mut rez = 1;
    for i in 1..=n { rez *= i; }
    rez
}
```

**Quiz — Ownership (recapitulare):**

```rust
// Q1: Compileaza sau nu?
// fn quiz1() { let s1 = String::from("hello"); let s2 = s1; println!("{}", s1); }
// NU — s1 mutat in s2, nu mai poate fi folosit.

// Q2: Compileaza sau nu?
// fn quiz2() { let mut s = String::from("hello"); let r1 = &mut s; let r2 = &mut s; }
// NU — doua &mut simultan sunt interzise.

// Q3: Ce afiseaza?
// fn quiz3() { let x = 5; let y = x; println!("{} {}", x, y); }
// "5 5" — i32 implementeaza Copy, x ramane valid.
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

### Drop ierarhic

Când se renunță la un struct, mai întâi se renunță la struct, apoi la câmpurile sale.

```rust
struct Bar { x: i32 }
struct Foo { bar: Bar }

fn main() {
    let foo = Foo { bar: Bar { x: 42 } };
    // drop foo -> drop foo.bar
}
```

---

### Practică — Ownership

```rust
// 1. Trei variante de a pastra proprietatea
fn main() {
    // Varianta A: referinta
    let x = String::from("hello");
    let y = &x;
    println!("{} {}", x, *y);

    // Varianta B: clone
    let x = String::from("hello");
    let y = x.clone();
    println!("{} {}", x, y);

    // Varianta C: &str nu se muta (tip Copy)
    let x: &str = "hello";
    let y = x;
    println!("{} {}", x, y);
}

// 2. Functie care returneaza ce a primit
fn take_ownership(s: String) -> String { s }
fn main() {
    let s1 = String::from("Hello");
    let s2 = take_ownership(s1);
    println!("{}", s2);
}

// 3. Clone pentru a pastra si original
fn main() {
    let s = String::from("Hello");
    print_str(s.clone()); // clone = trimite copie
    println!("{}", s);    // original intact
}
fn print_str(s: String) { println!("{}", s); }

// 4. Mutare partiala intr-un tuplu
fn main() {
    let t = (String::from("hello"), String::from("world"));
    let _s = t.0;           // t.0 este mutat
    println!("{}", t.1);    // t.1 inca valid
}

// 5. Referinte la ambele elemente (fara move)
fn main() {
    let t = (String::from("hello"), String::from("world"));
    let (s1, s2) = (&t.0, &t.1);
    println!("{} {} {:?}", s1, s2, t);
}
```

### Demo din lectie — `src/bin/01_ownership_borrowing.rs`

Analogie RPG: sabia, gold-ul si blacksmith-ul.

```rust
// cargo run --bin 01_ownership_borrowing

// MOVE: dai sabia — tu nu o mai ai
fn demo_move() {
    let jucator1 = String::from("Excalibur");
    let jucator2 = jucator1;
    // println!("{}", jucator1); // EROARE: mutat
    println!("jucator2 are: {}", jucator2);
}

// COPY: gold-ul se copiaza automat (i32 = tip primitiv)
fn demo_copy() {
    let gold = 500;
    let taxa = gold;
    println!("gold: {}, taxa: {}", gold, taxa); // ambele valide
}

// BORROWING (&): inspecteaza item-ul fara sa il ia
fn demo_borrowing() {
    let sabie = String::from("Excalibur");
    let lungime = inspecteaza_item(&sabie);
    println!("'{}' are {} caractere", sabie, lungime);
}
fn inspecteaza_item(item: &String) -> usize { item.len() }

// BORROWING MUTABIL (&mut): blacksmith upgradeaza item-ul
fn demo_mutable_borrow() {
    let mut sabie = String::from("Excalibur");
    upgrade_item(&mut sabie);
    println!("Dupa upgrade: {}", sabie); // "Excalibur +1"
}
fn upgrade_item(item: &mut String) { item.push_str(" +1"); }
```

### Exercitii rezolvate — `exercitii/src/bin/ex_01_ownership.rs`

```rust
// 2. Prima litera din numele eroului (fara move)
fn prima_litera(s: &String) -> char {
    s.chars().nth(0).unwrap()
}

// 3. Blacksmith adauga "!" (imprumut mutabil)
fn adauga_exclamare(s: &mut String) {
    s.push_str("!");
}

// 4. Fix: nu poti &mut si & simultan
fn main_ex4() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    r1.push_str(", hello");
    println!("{}", s); // "hello, hello"
}

// 5. Dubleaza continutul inventarului (clone pentru a citi inainte de modificare)
fn dublura(s: &mut String) {
    let original = s.clone();
    s.push_str(&original);
}

// 6. Prima si ultima litera (tuplu, fara move)
fn prima_si_ultima(s: &String) -> (char, char) {
    (s.chars().next().unwrap(), s.chars().last().unwrap())
}

// 7. Fix referinta dangling: muta s in acelasi scope cu referinta
fn main_ex7() {
    let referinta;
    let s = String::from("hello");
    referinta = &s;
    println!("{}", referinta);
}

// 8. Inverseaza string-ul in loc (*s = scriere la adresa)
fn inverseaza(s: &mut String) {
    *s = s.chars().rev().collect::<String>();
}

// 9. Quest log: preia ownership, adauga "[procesat]", returneaza ownership
fn proceseaza(s: String) -> String {
    let mut de_procesat = s;
    de_procesat.push_str(" [procesat]");
    de_procesat
}
```

### Exercitii rezolvate — `exercitii/src/bin/ex_01b_ownership_extra.rs`

```rust
// S1-A: NPC-ul distruge item-ul primit prin move.
//       Fix fara a schimba afiseaza(): trimite .clone()
fn main_s1a() {
    let item = String::from("Potion of Healing");
    afiseaza(item.clone());      // clone = trimite copie, originalul ramas
    println!("Am inca: {}", item);
}
fn afiseaza(text: String) { println!("{}", text); }

// S1-B: inventar_gol — TODO
// S1-C: lungime_nume — TODO
// S2-A: fa_majuscule — TODO
// S2-B: combina_titlu — TODO

// S2-C: Problema — de ce nu compileaza?
// fn creeaza_item() -> &String {
//     let s = String::from("Sabie Ruginita");
//     &s   // EROARE: s moare la sfarsit de functie, referinta ar fi dangling
// }
// Fix 1: returneaza String (owned), nu referinta
// fn creeaza_item() -> String { String::from("Sabie Ruginita") }
// Fix 2: returneaza &'static str (hardcodat)
// fn creeaza_item() -> &'static str { "Sabie Ruginita" }
```

> **Regula:** Nu poti returna o referinta la ceva creat in interiorul functiei.
> Ori returnezi valoarea (owned `String`), ori dai o referinta la ceva care exista **in afara** functiei.

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

### Unit Struct

Struct fără câmpuri — folosit rar, pentru tipuri marker.

```rust
struct Marcaj;
fn main() {
    let _m = Marcaj;
}
```

---

### Memorie — unde stau datele

- **Date statice** — text hardcodat (`"hello"`) — stocat în binar
- **Stack** — variabile locale de tip cunoscut la compilare
- **Heap** — date dinamice (`String`, `Vec`) accesate prin pointer de pe stack

Când instanțiezi un struct:
- Câmpurile simple (`i32`, `bool`) → **stack**
- `String` → metadata pe stack + datele reale pe **heap**

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

### Demo din lectie — `src/bin/03_structs.rs`

Analogie RPG: fisa de personaj + abilitatile lui.

```rust
// cargo run --bin 03_structs

struct Personaj { nume: String, hp: i32, mana: i32, nivel: u32 }

impl Personaj {
    fn new(nume: &str, nivel: u32) -> Personaj {
        Personaj { nume: String::from(nume), hp: 100, mana: 50, nivel }
    }
    fn descrie(&self) {
        println!("[{}] Nivel {} | HP: {} | Mana: {}", self.nume, self.nivel, self.hp, self.mana);
    }
    fn calculeaza_damage(&self, baza: i32) -> i32 { baza + self.nivel as i32 * 10 }
    fn bea_potion(&mut self) { self.hp += 50; }
    fn primeste_damage(&mut self, dmg: i32) {
        self.hp -= dmg;
        if self.hp <= 0 { println!("{} a cazut!", self.nume); }
        else { println!("{} are {} HP", self.nume, self.hp); }
    }
}

struct Echipa { lider: Personaj, dimensiune: u32 }

fn main() {
    let mag = Personaj::new("Jaina", 12);  // constructor ::
    mag.descrie();                          // metoda &self
    println!("Damage: {}", mag.calculeaza_damage(50)); // &self -> valoare

    let mut tank = Personaj::new("Thrall", 20);
    tank.bea_potion();                     // &mut self
    tank.primeste_damage(30);              // &mut self + parametru

    let echipa = Echipa { lider: Personaj::new("Sylvanas", 30), dimensiune: 5 };
    println!("Lider: {} (nivel {})", echipa.lider.nume, echipa.lider.nivel);
}
```

### Exercitii rezolvate — `exercitii/src/bin/ex_03_struct.rs`

Exercitiu cumulativ: **Struct + impl + Functii + Ownership & Borrowing**

```rust
struct Erou {
    nume: String,
    clasa: String,
    hp: i32,
    nivel: i32,
    inventar: Vec<String>,
}

impl Erou {
    // 1. Constructor
    fn new(nume: &str, clasa: &str) -> Erou {
        Erou {
            nume: String::from(nume),
            clasa: String::from(clasa),
            hp: 100,
            nivel: 1,
            inventar: Vec::new(),
        }
    }

    // 2. Afisare completa (imprumut imutabil)
    fn info(&self) {
        println!("[{} | {}] HP: {} | Nivel: {} | Inventar: {:?}",
            self.nume, self.clasa, self.hp, self.nivel, self.inventar);
    }

    // 3. Preia ownership-ul unui item si il adauga in inventar
    //    Parametrul e `String` (nu `&String`) — eroul detine itemul
    fn pick_up(&mut self, item: String) {
        self.inventar.push(item);
    }

    // 4. Scade HP, afiseaza daca eroul cade (ordinea conteaza!)
    fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
        if self.hp <= 0 {
            println!("{} a cazut in lupta!", self.nume);
        }
    }

    // 5. Verifica daca eroul e in viata
    fn is_alive(&self) -> bool {
        if self.hp > 0 { true } else { false }
    }

    // 6. Cauta "Potion" in inventar, scoate-l si da +30 HP
    fn use_potion(&mut self) {
        let index = self.inventar.iter().position(|item| item.contains("Potion"));
        match index {
            Some(i) => {
                self.inventar.remove(i);
                self.hp += 30;
            }
            None => println!("Nu ai nicio potiune in inventar!"),
        }
    }
}

// 7. putere_totala — TODO
// 8. ataca       — TODO
```

**Lectii invatate:**
- `take_damage`: scade HP **inainte** de verificare, altfel mesajul nu apare niciodata
- `pick_up` primeste `String` (owned) nu `&String` — Vec preia ownership-ul itemului
- `.iter().position(|x| conditie)` returneaza `Option<usize>` — se trateaza cu `match`
- `nivel: i32` in loc de `u32` — de corectat ca sa nu apara erori de tip la calcule

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

### `.collect()` pentru `HashMap` — combinarea a două liste cu `.zip()`

`.zip()` funcționează ca un fermoar: ia elementele din două iteratoare și le împerechează unu la unu, producând tupluri `(cheie, valoare)`. `.collect()` vede aceste tupluri și construiește un `HashMap`.

```rust
use std::collections::HashMap;

fn main() {
    let fructe = vec!["Mere", "Pere", "Banane"];
    let cantitati = vec![10, 5, 12];

    let inventar: HashMap<_, _> = fructe.into_iter()
        .zip(cantitati.into_iter())
        .collect();

    println!("{:#?}", inventar);
    // Ordinea poate varia — HashMap nu pastreaza ordinea inserarii
}
```

**Pas cu pas:**
1. `into_iter()` transformă fiecare vector în iterator
2. `.zip()` împerechează: `("Mere", 10)`, `("Pere", 5)`, `("Banane", 12)`
3. `.collect()` vede tuplurile și, datorită `HashMap<_, _>`, le asamblează ca hartă

> **`HashMap<_, _>`** — underscore-urile îi spun compilatorului "știu că vreau un HashMap, deduce tu tipurile". Rust deduce singur `HashMap<&str, i32>`.

#### Dacă ai deja datele ca perechi

```rust
use std::collections::HashMap;

fn main() {
    let setari = vec![
        ("tema", "intunecata"),
        ("limba", "ro"),
        ("notificari", "pornite"),
    ];

    let setari: HashMap<&str, &str> = setari.into_iter().collect();

    println!("Tema: {}", setari.get("tema").unwrap());
}
```

Fără `.zip()` — datele sunt deja perechi, `.collect()` le preia direct.

### `.collect()` pentru gestionarea erorilor într-o listă

`.collect()` poate "extrage" erorile dintr-o listă de `Result<T, E>`: evaluează toate elementele și returnează fie `Ok(Vec<T>)` cu toate valorile, fie **prima eroare** `Err(E)` întâlnită.

```rust
fn main() {
    let numere_text = vec!["1", "2", "trei", "4"];

    let rezultat: Result<Vec<i32>, _> = numere_text.iter()
        .map(|s| s.parse::<i32>())
        .collect();

    match rezultat {
        Ok(nums) => println!("Toate parsate: {:?}", nums),
        Err(e)   => println!("Eroare la parsare: {}", e),
    }
    // "Eroare la parsare: invalid digit found in string"
}
```

**De ce funcționează?** `Result` implementează `FromIterator` — `.collect()` știe că dacă tipul țintă e `Result<Vec<T>, E>`, trebuie să oprească la prima eroare.

Dacă toate textele sunt valide:
```rust
let valide = vec!["1", "2", "3"];
let rezultat: Result<Vec<i32>, _> = valide.iter()
    .map(|s| s.parse::<i32>())
    .collect();
// Ok([1, 2, 3])
```

#### Rezumat: ce poate produce `.collect()`

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
| Existenta | `Path::new(path).exists()` | `bool` |
- `Iterator` — pentru bucle `for`