# 06 — Colectii: `Vec` si `HashMap`

## De ce ai nevoie de colectii?

Variabilele normale stocheaza o singura valoare. Colectiile stocheaza **mai multe valori** intr-o singura structura.

Rust are doua colectii principale pe care le vei folosi in 90% din cazuri:

| Colectie | Analogie | Cand o folosesti |
|----------|----------|-----------------|
| `Vec<T>` | Sertar cu sloturi numerotate | Lista ordonata, acces dupa index |
| `HashMap<K,V>` | Dictionar real | Cauti dupa cheie, nu dupa pozitie |

---

## `Vec<T>` — lista dinamica

> **Analogie**: Un inventar RPG cu sloturi numerotate. Poti adauga iteme la sfarsit, scoate ultimul, sau sterge de la o pozitie specifica.

```rust
// Creare
let mut v: Vec<i32> = Vec::new();   // gol
let v2 = vec![1, 2, 3];            // cu valori initiale (macro)

// Adaugare / stergere
v.push(10);          // adauga la sfarsit
v.pop()              // scoate ultimul → Option<T>
v.remove(0)          // scoate de la index → T (panic daca invalid)

// Acces
v[0]                 // acces direct → panic daca index invalid!
v.get(0)             // acces sigur  → Option<&T>

// Info
v.len()              // numarul de elemente
v.is_empty()         // true daca e gol

// Iterare
for x in &v { ... }                        // imprumut
for (i, x) in v.iter().enumerate() { ... } // cu index
```

### Acces sigur vs acces direct

```rust
let v = vec!["a", "b", "c"];

v[1]        // "b" — dar v[99] face PANIC la runtime
v.get(1)    // Some("b")
v.get(99)   // None — sigur, nu face panic
```

Regula: foloseste `.get()` cand nu esti sigur ca indexul exista.

---

## `HashMap<K, V>` — dictionar cheie→valoare

> **Analogie**: Un registru al jucatorilor online — cauti dupa **nume** si gasesti **HP-ul**. Nu ai nevoie sa stii pozitia lui in lista.

```rust
use std::collections::HashMap;  // OBLIGATORIU — nu e in prelude

let mut hp: HashMap<String, i32> = HashMap::new();

// Inserare
hp.insert(String::from("Arthas"), 100);

// Acces sigur
hp.get("Arthas")          // Some(&100) sau None

// Verificare
hp.contains_key("Arthas") // true / false

// Stergere
hp.remove("Arthas")       // Option<i32>

// Iterare
for (cheie, valoare) in &hp { ... }
```

### `entry` — insereaza doar daca nu exista

```rust
// Adauga "Zara" cu 40 DOAR daca nu exista deja
hp.entry(String::from("Zara")).or_insert(40);
```

Util cand vrei sa initializezi o valoare fara sa suprascrii una existenta.

---

## Vec vs HashMap

```
Vec<T>                          HashMap<K, V>
──────────────────────          ──────────────────────────────
Ordonat (0, 1, 2, ...)          Neordonat
Acces dupa INDEX numeric        Acces dupa CHEIE (orice tip)
push/pop/remove                 insert/remove/get
Inventar de iteme               HP per jucator, scoruri, config
```

**Regula practica:**
- Stii pozitia / ordinea conteaza → `Vec`
- Cauti dupa un nume / ID unic → `HashMap`

---

## Import

`Vec` e disponibil automat (in prelude).
`HashMap` trebuie importat explicit:

```rust
use std::collections::HashMap;
```

---

## Rezumat metode

### Vec
| Metoda | Ce face | Returneaza |
|--------|---------|------------|
| `push(v)` | Adauga la sfarsit | `()` |
| `pop()` | Scoate ultimul | `Option<T>` |
| `remove(i)` | Scoate de la index | `T` |
| `get(i)` | Acces sigur | `Option<&T>` |
| `len()` | Numar elemente | `usize` |
| `is_empty()` | E gol? | `bool` |

### HashMap
| Metoda | Ce face | Returneaza |
|--------|---------|------------|
| `insert(k, v)` | Adauga / suprascrie | `Option<V>` |
| `get(k)` | Cauta dupa cheie | `Option<&V>` |
| `remove(k)` | Sterge | `Option<V>` |
| `contains_key(k)` | Exista cheia? | `bool` |
| `entry(k).or_insert(v)` | Insereaza daca lipseste | `&mut V` |

---

## Exercitii

Fisier de lucru: `exercitii/src/bin/ex_06_collections.rs`
Rulare: `cargo run --bin ex_06_collections`

---

*Nota: Fisier creat in sesiunea din 2026-03-17.*