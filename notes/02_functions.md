# 02 — Functii, parametri si return values

## Ce este o functie?

O functie este un bloc de cod cu un **nume**, care primeste **date de intrare** (parametri), face ceva cu ele si (optional) **returneaza un rezultat**.

> **Analogie**: O functie e ca o reteta de bucatarie. Ii dai ingrediente (parametri), urmeaza niste pasi, si primesti un rezultat (return value). Reteta nu tine minte nimic intre utilizari.

---

## Sintaxa

```rust
fn nume_functie(parametru1: Tip1, parametru2: Tip2) -> TipReturn {
    // corp
}
```

- `fn` — cuvantul cheie pentru functie
- Tipurile parametrilor sunt **obligatorii** — Rust nu face inferenta la semnatura
- `-> Tip` — tipul valorii returnate (se omite daca functia nu returneaza nimic)

---

## Functie fara return

Cand o functie nu returneaza nimic, tipul return e `()` (se numeste **unit**). De obicei il omitem:

```rust
fn salut() {
    println!("Salut!");
}
// echivalent cu: fn salut() -> () { ... }
```

---

## Parametri

Fiecare parametru are tipul sau explicit, separati prin virgula:

```rust
fn suma(a: i32, b: i32) -> i32 {
    a + b
}
```

---

## Expresii vs Instructiuni — conceptul cheie

Aceasta e una dintre cele mai importante distinctii in Rust.

| | Expresie | Instructiune |
|---|---|---|
| **Forma** | `a + b` | `a + b;` |
| **Produce valoare?** | DA | NU (valoarea e aruncata) |
| **Semn distinctiv** | fara `;` | cu `;` la final |

> **Analogie**: Expresia e ca un vanzator care iti da restul. Instructiunea e ca un vanzator care arunca restul la cos — calculul s-a facut, dar nu ai primit nimic.

---

## Return implicit

Ultima **expresie** dintr-o functie este automat valoarea returnata:

```rust
fn dublu(n: i32) -> i32 {
    n * 2        // fara ; — expresie, e returnata automat
}
```

Echivalent cu:

```rust
fn dublu(n: i32) -> i32 {
    return n * 2;  // return explicit — corect, dar mai putin idiomaic
}
```

**Greseala comuna**:

```rust
fn dublu(n: i32) -> i32 {
    n * 2;   // EROARE: ; transforma expresia in instructiune
             // functia nu mai returneaza nimic (returneaza ())
}
```

---

## if/else ca expresie

In Rust, `if/else` este o **expresie** — poate produce o valoare:

```rust
fn max_doua(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}
```

Sau asignat la o variabila:

```rust
let rezultat = if a > b { a } else { b };
```

**Regula**: Ambele ramuri ale `if/else` trebuie sa returneze **acelasi tip**.

---

## Return timpuriu

Poti iesi din functie inainte de final cu `return` explicit:

```rust
fn descrie_numar(n: i32) -> &'static str {
    if n == 0 {
        return "zero";
    }
    if n > 0 {
        return "pozitiv";
    }
    "negativ"  // return implicit pentru cazul ramas
}
```

---

## Functii si Ownership

Tipul parametrului determina ce se intampla cu ownership-ul:

| Semnatura | Ce se intampla |
|---|---|
| `fn f(s: String)` | ownership **mutat** in functie — originalul invalid dupa apel |
| `fn f(s: &String)` | **imprumut imutabil** — originalul ramine valid |
| `fn f(s: &mut String)` | **imprumut mutabil** — originalul poate fi modificat |
| `fn f(s: String) -> String` | ownership mutat, **returnat inapoi** |

```rust
let s = String::from("hello");
afiseaza(&s);        // imprumut — s ramine valid
println!("{}", s);   // OK

let s2 = String::from("hello");
let s3 = preia(s2);  // s2 mutat in functie
// println!("{}", s2); // EROARE — s2 invalid
println!("{}", s3);  // OK — ownership returnat in s3
```

---

## Rezumat vizual

```
fn nume(a: Tip, b: Tip) -> TipReturn {
    let x = a + b;   // instructiune — salveaza
    x + 1;           // instructiune — calculeaza si ARUNCA
    x                // expresie — RETURNEAZA
}
```

---

## Reguli de memorat

| # | Regula |
|---|--------|
| 1 | Tipurile parametrilor sunt **obligatorii** in semnatura |
| 2 | Ultima expresie (fara `;`) este **return implicit** |
| 3 | `;` transforma o expresie in instructiune — valoarea e **pierduta** |
| 4 | `if/else` este o **expresie** — poate returna valori |
| 5 | `return` explicit este folosit pentru **iesire timpurie** |

---

## Exercitii

Fisier de lucru: `exercitii/src/bin/ex_02_functions.rs`
Rulare: `cargo run --bin ex_02_functions`

**1.** Scrie `patrat(n: i32) -> i32` — returneaza `n * n`. Foloseste return implicit.

**2.** Scrie `saluta(nume: &String)` — printeaza `"Salut, <nume>!"`. Fara return.

**3.** Scrie `max_doua(a: i32, b: i32) -> i32` — returneaza cel mai mare. Foloseste `if/else` ca expresie.

---

*Nota: Fisier generat in sesiunea din 2026-02-24*
