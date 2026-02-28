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

## Regula 1 — Fiecare item are un singur proprietar (owner)

> **Analogie**: Intr-un RPG, o sabie legendara poate fi detinuta de **un singur jucator**.
> Nu exista doua copii ale aceluiasi item unic — daca tu il ai, altcineva nu il poate avea simultan.

```rust
let sabie = String::from("Excalibur"); // `sabie` este proprietarul
```

- `sabie` **detine** valoarea `"Excalibur"` in memorie.
- Cand `sabie` iese din scope (se inchide `{}`), item-ul e **sters automat (drop)**.

```rust
fn main() {
    let sabie = String::from("Excalibur"); // item intrat in inventar
    println!("{}", sabie);
} // sabie iese din scope — item sters automat
```

---

## Regula 2 — Poate exista un singur proprietar la un moment dat (Move)

Daca dai item-ul unui alt jucator, **tu nu il mai ai**.

```rust
fn main() {
    let jucator1 = String::from("Excalibur");
    let jucator2 = jucator1; // item mutat in inventarul lui jucator2

    // println!("{}", jucator1); // EROARE! jucator1 nu mai are item-ul
    println!("{}", jucator2);    // OK
}
```

> Dupa ce ai dat sabia, inventarul tau e gol. Nu poti echipa ceva ce nu mai ai.

---

## Tipurile simple — Gold si consumabile

Gold-ul se copiaza automat — dai 50g cuiva si tot mai ai 50g la tine.
Asa functioneaza tipurile simple (`i32`, `bool`, `f64`, `char`) in Rust.

```rust
fn main() {
    let gold = 100;
    let taxa = gold; // copie automata — gold ramane valid
    println!("gold: {}, taxa: {}", gold, taxa); // OK
}
```

Aceste tipuri implementeaza trait-ul `Copy` — sunt mici si stau pe **stack**, nu pe **heap**.

---

## Regula 3 — Cand proprietarul iese din scope, item-ul e sters

```rust
fn main() {
    {
        let armura = String::from("Armura de Fier");
        println!("{}", armura); // OK
    } // armura iese din scope — item sters din memorie

    // println!("{}", armura); // EROARE! armura nu mai exista
}
```

---

## Move in functii — Dai item-ul unui NPC

Cand trimiti un item la o functie, **proprietatea se muta** in acea functie.

```rust
fn main() {
    let sabie = String::from("Excalibur");
    npc_primeste(sabie); // sabia pleaca la NPC

    // println!("{}", sabie); // EROARE! sabia nu mai e la tine
}

fn npc_primeste(item: String) {
    println!("Am primit: {}", item);
} // item sters — NPC-ul a distrus item-ul la final
```

Aceasta e incomod. Solutia? **Borrowing** — imprumuti, nu dai.

---

## Borrowing — Imprumuti item-ul unui coechipier

In loc sa muti ownership-ul, poti **imprumuta** item-ul cu `&`.

```rust
fn main() {
    let sabie = String::from("Excalibur");
    inspecteaza(&sabie); // coechipierul o inspecteaza, nu o primeste

    println!("Sabia e inca la mine: {}", sabie); // OK!
}

fn inspecteaza(item: &String) {
    println!("Inspecteaza: {}", item);
} // referinta expira — sabia ramane la proprietar
```

> **Analogie**: Dai sabia coechipierului s-o vada. El o inspecteaza, dar nu o detine.
> Ti-o returneaza automat dupa.

### Reguli pentru referinte

1. Poti imprumuta de **oricate ori** imutabil (`&T`) simultan.
2. Sau **o singura data** mutabil (`&mut T`) — dar nu impreuna cu alte referinte.

---

## Referinta mutabila — Coechipierul face upgrade la item

```rust
fn main() {
    let mut sabie = String::from("Excalibur");
    upgrade_item(&mut sabie); // imprumut cu permisiunea de a modifica
    println!("{}", sabie);    // "Excalibur +1"
}

fn upgrade_item(item: &mut String) {
    item.push_str(" +1");
}
```

---

## &mut necesita o variabila mutabila

Nu poti face upgrade unui item daca e "blocat" (imutabil).
Item-ul trebuie declarat `mut` de la inceput.

```rust
let sabie = String::from("Excalibur");  // blocat
let r = &mut sabie;                      // EROARE — nu poate fi modificat

let mut sabie = String::from("Excalibur"); // deblocat
let r = &mut sabie;                         // OK
```

---

## Rezumat vizual

```
Ownership:
  let jucator1 = String::from("Excalibur");  →  jucator1 detine item-ul
  let jucator2 = jucator1;                   →  jucator2 il detine, jucator1 nu mai are nimic

Borrowing imutabil (&):
  let r = &sabie;     →  r il inspecteaza, sabie ramane proprietar
                         pot exista mai multi &sabie simultan

Borrowing mutabil (&mut):
  let r = &mut sabie; →  r poate modifica, dar e SINGURA referinta activa
```

---

## Reguli de memorat

| # | Regula |
|---|--------|
| 1 | Fiecare item are **un singur owner** |
| 2 | Cand owner-ul iese din scope, item-ul e **sters automat** |
| 3 | Poti imprumuta cu `&` (imutabil) sau `&mut` (mutabil) |
| 4 | Nu poti avea `&mut` si `&` active **in acelasi timp** |
| 5 | Nu poti avea mai mult de **un `&mut`** activ simultan |

---

## Exercitii si solutii

### Exercitiul 1 — Move vs Borrow

De ce da eroare codul urmator? Cum il repari?

```rust
fn main() {
    let sabie = String::from("Excalibur");
    let alt_jucator = sabie;      // sabia e mutata
    println!("{}", sabie);        // EROARE: sabia nu mai e la tine
}
```

**Eroare**: `sabie` si-a pierdut ownership prin move. Nu poti folosi un item dupa ce l-ai dat.

**Solutie**: Imprumuta cu `&`.

```rust
fn main() {
    let sabie = String::from("Excalibur");
    let alt_jucator = &sabie;               // imprumut, nu move
    println!("{}, {}", sabie, alt_jucator); // ambele valide
}
```

---

### Exercitiul 2 — `prima_litera`

Scrie `prima_litera(s: &String) -> char` — returneaza primul caracter al numelui eroului, fara sa mute ownership.

```rust
fn prima_litera(s: &String) -> char {
    s.chars().next().unwrap()
}
```

**Concepte**: `&String` — imprumut imutabil. `.chars().next()` returneaza `Option<char>`, `.unwrap()` extrage valoarea.

---

### Exercitiul 3 — `adauga_exclamare`

Scrie `adauga_exclamare(s: &mut String)` — adauga `"!"` la sfarsitul numelui eroului.

```rust
fn adauga_exclamare(s: &mut String) {
    s.push_str("!");
}
```

---

### Exercitiul 4 — Doua referinte mutabile (eroare)

Codul urmator nu compileaza. Gaseste erorile si repara-l.

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;  // EROARE: al doilea &mut activ simultan
    println!("{} {}", r1, r2);
}
```

**Solutie**: Termini un borrow inainte de a-l incepe pe altul.

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    r1.push_str(", hello");
    // r1 nu mai e activ dupa aceasta linie
    println!("{}", s); // OK
}
```

---

### Exercitiul 5 — `dublura`

Scrie `dublura(s: &mut String)` — dubleaza continutul: `"Rust"` → `"RustRust"`.

```rust
fn dublura(s: &mut String) {
    let original = s.clone(); // salvezi copia INAINTE sa modifici
    s.push_str(&original);
}
```

---

### Exercitiul 6 — `prima_si_ultima`

Scrie `prima_si_ultima(s: &String) -> (char, char)` — returneaza primul si ultimul caracter din numele eroului.

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

```rust
// Cod eronat:
fn main() {
    let referinta;
    {
        let s = String::from("hello");
        referinta = &s;
    } // s sters — referinta pointeaza la memorie invalida
    println!("{}", referinta); // EROARE
}
```

**Solutie**: `s` trebuie sa traiasca cel putin cat `referinta`.

```rust
fn main() {
    let referinta;
    let s = String::from("hello"); // s in acelasi scope
    referinta = &s;
    println!("{}", referinta); // OK
}
```

---

### Exercitiul 8 — `inverseaza`

Scrie `inverseaza(s: &mut String)` — inverseaza numele eroului: `"Rust"` → `"tsuR"`.

```rust
fn inverseaza(s: &mut String) {
    *s = s.chars().rev().collect::<String>();
}
```

---

### Exercitiul 9 — `proceseaza`

Scrie `proceseaza(s: String) -> String` — preia ownership-ul unui item, adauga `" [procesat]"`, returneaza ownership-ul inapoi.

```rust
fn proceseaza(s: String) -> String {
    let mut de_procesat = s;
    de_procesat.push_str(" [procesat]");
    de_procesat
}
```

---

## Exercitii suplimentare

Fisier de lucru: `exercitii/src/bin/ex_01b_ownership_extra.rs`
Rulare: `cargo run --bin ex_01b_ownership_extra`

---

### Serie 1 — Dificultate mica

**S1-A.** NPC-ul `afiseaza` primeste item-ul prin move si il distruge. Repara codul din main **fara sa schimbi functia `afiseaza`**, astfel incat sa poti folosi item-ul si dupa apel.

```rust
fn main() {
    let item = String::from("Potion of Healing");
    afiseaza(item);
    println!("Am inca: {}", item); // de ce da eroare?
}

fn afiseaza(text: String) { println!("{}", text); }
```

**S1-B.** Scrie `inventar_gol(s: &String) -> bool` — returneaza `true` daca inventarul (string-ul) e gol.
- `"Sabie"` → `false`
- `""` → `true`
- Indiciu: `.is_empty()` sau `.len() == 0`

**S1-C.** Scrie `lungime_nume(s: &String) -> usize` — returneaza numarul de **caractere** (nu bytes) din numele personajului.
- `"Rust"` → `4`
- Indiciu: `.chars().count()` numara caractere Unicode, `.len()` numara bytes.

---

### Serie 2 — Dificultate medie

**S2-A.** Scrie `fa_majuscule(s: &mut String)` — transforma numele personajului in majuscule, in loc.
- `"arthur"` → `"ARTHUR"`
- Indiciu: `.to_uppercase()` returneaza un String nou — cum il atribui inapoi?

**S2-B.** Scrie `combina_titlu(s1: &String, s2: &String) -> String` — returneaza `"<s1> <s2>"` fara sa mute niciun owner.
- `"Lord"`, `"Arthur"` → `"Lord Arthur"`
- Indiciu: `format!("{} {}", s1, s2)`

**S2-C.** Functia de mai jos nu compileaza. Explica de ce si repara-o in doua moduri diferite.

```rust
fn creeaza_item() -> &String {
    let s = String::from("Sabie Ruginita");
    &s
}
```

---

*Nota: Fisier generat in sesiunea din 2026-02-23. Refacut cu tema jocuri video in 2026-02-28.*
