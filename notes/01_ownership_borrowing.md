# 01 — Ownership & Borrowing in Rust

## De ce exista Ownership?

Orice program are nevoie de **memorie** ca sa stocheze date.
Limbajele de programare rezolva asta in moduri diferite:

| Limbaj | Cum gestioneaza memoria |
|--------|------------------------|
| C / C++ | Tu aloci si eliberezi manual (`malloc` / `free`) |
| Python / Java | Un **Garbage Collector** face curatenie automat |
| **Rust** | **Ownership** — compilatorul face curatenie, fara GC |

Rust nu are Garbage Collector. In schimb, are un set de **reguli** verificate la compilare.
Daca le respecti — codul e sigur si rapid. Daca nu — codul **nu compileaza**.

---
## Regula 1 — Fiecare valoare are un singur proprietar (owner)

```rust
let s = String::from("hello"); // `s` este proprietarul acestui String
```

- `s` **detine** valoarea `"hello"` in memorie.
- Cand `s` iese din scope (blocul `{}` se inchide), memoria e **eliberata automat**.

```rust
fn main() {
    let s = String::from("hello"); // s intra in scope, memorie alocata
    println!("{}", s);
} // s iese din scope, memorie eliberata automat (drop)
```

---

## Regula 2 — Poate exista un singur proprietar la un moment dat (Move)

Daca atribui o valoare altei variabile, **proprietatea se muta (move)**.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // proprietatea se muta de la s1 la s2

    // println!("{}", s1); // EROARE! s1 nu mai este valid
    println!("{}", s2);    // OK
}
```

> **Analogie**: E ca si cum ai da cuiva un document original.
> Dupa ce i l-ai dat, tu nu il mai ai.

### Atentie: tipurile simple (i32, bool, f64, char) se **copiaza**, nu se muta

```rust
fn main() {
    let x = 5;
    let y = x; // x se copiaza, ambele sunt valide

    println!("x = {}, y = {}", x, y); // OK
}
```

Aceste tipuri implementeaza trait-ul `Copy` — sunt mici si stau pe **stack**, nu pe **heap**.

---

## Regula 3 — Cand proprietarul iese din scope, valoarea e stearsa

```rust
fn main() {
    {
        let s = String::from("in bloc");
        println!("{}", s); // OK
    } // s iese din scope, memorie eliberata

    // println!("{}", s); // EROARE! s nu mai exista
}
```

---

## Move in functii

Cand trimiti o valoare la o functie, **proprietatea se muta** in acea functie.

```rust
fn main() {
    let s = String::from("hello");
    preia_proprietatea(s); // s se muta in functie

    // println!("{}", s); // EROARE! s nu mai apartine lui main
}

fn preia_proprietatea(text: String) {
    println!("{}", text);
} // text iese din scope, memorie eliberata
```

Aceasta e incomod. Solutia? **Borrowing**.

---

## Borrowing — Imprumutarea valorilor

In loc sa muti proprietatea, poti **imprumuta** valoarea cu `&`.

```rust
fn main() {
    let s = String::from("hello");
    calculeaza_lungime(&s); // imprumutam, nu mutam

    println!("{}", s); // OK! s inca ne apartine
}

fn calculeaza_lungime(text: &String) -> usize {
    text.len()
} // text (referinta) iese din scope, dar valoarea NU e stearsa
```

> **Analogie**: E ca si cum ai imprumuta cartea cuiva. Ei tot o detin,
> tu o citesti si o returnezi.

### Reguli pentru referinte

1. Poti avea **oricate referinte imutabile** (`&T`) simultan.
2. Sau **o singura referinta mutabila** (`&mut T`) — dar nu amandoua odata.

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;     // OK — referinta imutabila
    let r2 = &s;     // OK — inca o referinta imutabila
    println!("{} {}", r1, r2);

    // Dupa ce r1 si r2 nu mai sunt folosite, putem face:
    let r3 = &mut s; // OK — referinta mutabila
    r3.push_str(", world");
    println!("{}", r3);
}
```

---

## Referinta mutabila — modificarea prin imprumut

```rust
fn main() {
    let mut s = String::from("hello");
    adauga_text(&mut s); // imprumut mutabil
    println!("{}", s);   // "hello, world"
}

fn adauga_text(text: &mut String) {
    text.push_str(", world");
}
```

---

## &mut necesita o variabila mutabila

Nu poti face `&mut` catre o variabila imutabila — variabila trebuie declarata cu `mut`.

> **Analogie**: Un document laminat (imutabil) nu poate fi modificat chiar daca dai cuiva un pix (`&mut`). Foaia trebuie sa fie normala (`mut`) de la inceput.

```rust
let s = String::from("hello");      // imutabila
let r = &mut s;                     // EROARE — s nu e mut

let mut s = String::from("hello");  // mut obligatoriu
let r = &mut s;                     // OK
```

---

## Rezumat vizual

```
Ownership (proprietate):
  let s1 = String::from("hello");  →  s1 detine "hello"
  let s2 = s1;                      →  s2 detine "hello", s1 invalid

Borrowing imutabil (&):
  let r = &s1;   →  r imprumuta, s1 ramane proprietar
                    pot exista mai multe &s1 simultan

Borrowing mutabil (&mut):
  let r = &mut s1;  →  r poate modifica, dar e SINGURA referinta activa
```

---

## Reguli de memorat

| # | Regula |
|---|--------|
| 1 | Fiecare valoare are **un singur owner** |
| 2 | Cand owner-ul iese din scope, valoarea e **stearsa** |
| 3 | Poti imprumuta cu `&` (imutabil) sau `&mut` (mutabil) |
| 4 | Nu poti avea `&mut` si `&` active **in acelasi timp** |
| 5 | Nu poti avea mai mult de **un `&mut`** activ simultan |

---

## Exercitii si solutii

---

### Exercitiul 1 — Move vs Borrow

De ce da eroare codul urmator? Cum il repari?

```rust
fn main() {
    let s1 = String::from("test");
    let s2 = s1;        // s1 se muta in s2 — s1 nu mai e valid
    println!("{}", s1); // EROARE: s1 a fost mutat
}
```

**Eroare**: `s1` si-a pierdut proprietatea prin move la `s2`. Nu poti folosi o variabila dupa ce a fost mutata.

**Solutie**: Foloseste `&` pentru imprumut in loc de move.

```rust
fn main() {
    let mut s1 = String::from("Rust");
    let s2 = &s1;               // imprumut, nu move
    println!("{}, {}", s1, s2); // ambele valide
}
```

---

### Exercitiul 2 — `prima_litera`

Scrie o functie `prima_litera(s: &String) -> char` care returneaza primul caracter, fara sa mute proprietatea.

```rust
fn prima_litera(s: &String) -> char {
    s.chars().next().unwrap()
}
```

**Concepte**: `&String` — imprumut imutabil, proprietarul nu se schimba. `.chars().next()` returneaza primul caracter ca `Option<char>`, `.unwrap()` extrage valoarea.

---

### Exercitiul 3 — `adauga_exclamare`

Scrie o functie `adauga_exclamare(s: &mut String)` care adauga `"!"` la sfarsit.

```rust
fn adauga_exclamare(s: &mut String) {
    s.push_str("!");
}
```

**Concepte**: `&mut String` — imprumut mutabil. Variabila din `main` trebuie declarata cu `mut`.

---

### Exercitiul 4 — Doua referinte mutabile (eroare)

Codul urmator nu compileaza. Gaseste erorile si repara-l.

```rust
// Cod eronat:
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;  // EROARE: al doilea &mut activ simultan
    println!("{} {}", r1, r2);
}
```

**Erori**:
- Nu poti avea **doua `&mut`** active simultan
- Nu poti avea **`&mut` si `&`** active simultan

**Solutie**: Folosesti o singura referinta mutabila si o eliberezi inainte de a crea alta.

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    r1.push_str(", hello");
    // r1 nu mai e folosit dupa aceasta linie — borrow-ul se elibereaza
    println!("{}", s);
}
```

---

### Exercitiul 5 — `dublura`

Scrie o functie `dublura(s: &mut String)` care dubleaza continutul: `"Rust"` → `"RustRust"`.

```rust
fn dublura(s: &mut String) {
    let original = s.clone(); // salvezi copia INAINTE sa modifici
    s.push_str(&original);    // adaugi copia la sfarsitul originalului
}
```

**Greseala comuna**: `s.clone().push_str(s)` — clone-ul e temporar si se arunca imediat, `s` ramane nemodificat.

---

### Exercitiul 6 — `prima_si_ultima`

Scrie o functie `prima_si_ultima(s: &String) -> (char, char)` care returneaza primul si ultimul caracter.

```rust
fn prima_si_ultima(s: &String) -> (char, char) {
    let prima = s.chars().next().unwrap();
    let ultima = s.chars().last().unwrap();
    (prima, ultima)
}
```

**Exemplu**: `"Rust"` → `('R', 't')`

---

### Exercitiul 7 — Dangling reference (referinta suspendata)

Codul urmator nu compileaza. Explica de ce si repara-l.

```rust
// Cod eronat:
fn main() {
    let referinta;
    {
        let s = String::from("hello");
        referinta = &s;
    } // s iese din scope si e sters (drop)
    println!("{}", referinta); // EROARE: referinta pointeaza la memorie eliberata
}
```

**Eroare**: `s` e sters cand `{}` se inchide. `referinta` ar pointa la memorie invalida — Rust nu permite asta (dangling reference).

**Solutie**: `s` trebuie sa traiasca cel putin cat `referinta`.

```rust
fn main() {
    let referinta;
    let s = String::from("hello"); // s in acelasi scope cu referinta
    referinta = &s;
    println!("{}", referinta); // OK
}
```

---

### Exercitiul 8 — `inverseaza`

Scrie o functie `inverseaza(s: &mut String)` care inverseaza sirul in loc: `"Rust"` → `"tsuR"`.

```rust
fn inverseaza(s: &mut String) {
    *s = s.chars().rev().collect::<String>();
}
```

**Concepte**:
- `*s = ...` — dereferentiere: scrii la adresa la care pointeaza `s`
- `.chars().rev()` — iteratorul caracterelor, inversat
- `.collect::<String>()` — aduna caracterele inapoi intr-un `String`

---

### Exercitiul 9 — `proceseaza` (ownership complet)

Scrie o functie `proceseaza(s: String) -> String` care preia proprietatea, adauga `" [procesat]"` si returneaza proprietatea.

```rust
fn proceseaza(s: String) -> String {
    let mut de_procesat = s;            // preia ownership
    de_procesat.push_str(" [procesat]");
    de_procesat                         // returneaza ownership
}
```

**Raspuns la intrebarea din exercitiu**: Dupa `let rezultat = proceseaza(s)`, variabila `s` **nu mai poate fi folosita** — ownership-ul a fost mutat in functie si returnat in `rezultat`.

**Exemplu**: `"date"` → `"date [procesat]"`

---

*Nota: Fisier generat in sesiunea din 2026-02-23. Exercitii completate in sesiunea din 2026-02-24.*

---

## Exercitii suplimentare

Fisier de lucru: `exercitii/src/bin/ex_01b_ownership_extra.rs`
Rulare: `cargo run --bin ex_01b_ownership_extra`

---

### Serie 1 — Dificultate mica

**S1-A.** Codul urmator nu compileaza. Explica de ce si repara-l fara sa schimbi functia `afiseaza`.

```rust
fn main() {
    let s = String::from("buna ziua");
    afiseaza(s);
    println!("Original: {}", s); // de ce da eroare?
}

fn afiseaza(text: String) {
    println!("{}", text);
}
```

**S1-B.** Scrie o functie `este_goala(s: &String) -> bool` care returneaza `true` daca string-ul nu are niciun caracter.
- `"hello"` → `false`
- `""` → `true`
- Indiciu: `.is_empty()` sau `.len() == 0`

**S1-C.** Scrie o functie `lungime(s: &String) -> usize` care returneaza numarul de caractere (nu bytes).
- `"Rust"` → `4`
- Indiciu: `.chars().count()` numara caractere, `.len()` numara bytes — difera pentru caractere speciale.

---

### Serie 2 — Dificultate medie

**S2-A.** Scrie o functie `fa_majuscule(s: &mut String)` care transforma tot textul in majuscule, in loc.
- `"rust"` → `"RUST"`
- Indiciu: `.to_uppercase()` returneaza un String nou — cum il atribui inapoi?

**S2-B.** Scrie o functie `concateneaza(s1: &String, s2: &String) -> String` care returneaza un String nou format din `s1 + " " + s2`, fara sa mute proprietatea niciunuia.
- `"buna"`, `"ziua"` → `"buna ziua"`
- Indiciu: `format!("{} {}", s1, s2)`

**S2-C.** Codul urmator nu compileaza. Explica de ce si propune doua variante de rezolvare diferite.

```rust
fn creeaza_string() -> &String {
    let s = String::from("hello");
    &s
}

fn main() {
    let r = creeaza_string();
    println!("{}", r);
}
```
