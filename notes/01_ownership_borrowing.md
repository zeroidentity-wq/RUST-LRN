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

## Exercitii

**1.** De ce da eroare codul urmator? Cum il repari?
```rust
fn main() {
    let s1 = String::from("test");
    let s2 = s1;
    println!("{}", s1);
}
```

**2.** Scrie o functie `prima_litera(s: &String) -> char` care
returneaza primul caracter dintr-un String, fara sa mute proprietatea.

**3.** Scrie o functie `adauga_exclamare(s: &mut String)` care
adauga `"!"` la sfarsitul unui String.

---

*Nota: Fisier generat in sesiunea din 2026-02-23*
