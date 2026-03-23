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

// Iterare cu index
for (i, (cheie, valoare)) in hp.iter().enumerate() { ... }
```

### `iter()` si `enumerate()` pe HashMap

`iter()` pe un HashMap returneaza **perechi `(&cheie, &valoare)`** — dar fara ordine garantata (HashMap-ul nu e sortat).

`enumerate()` **impacheteaza** orice iterator si adauga un contor automat. Transforma fiecare element `X` in `(index, X)`:

```
hp.iter()            →  (&"Player1", &100), (&"Player2", &100), ...
hp.iter().enumerate() →  (0, (&"Player1", &100)), (1, (&"Player2", &100)), ...
```

In bucla `for`, destructurezi ambele niveluri:

```rust
for (i, (nume, viata)) in hp.iter().enumerate() {
    println!("{}: {} are {} HP", i, nume, viata);
}
// i     = usize (0, 1, 2...)
// nume  = &String
// viata = &i32
```

> **Atentie la tipuri**: `HashMap<String, i32>` asteapta cheie `String`.
> `"Player1"` este `&str` — trebuie convertit:
> ```rust
> hp.insert("Player1".to_string(), 100);  // corect
> // sau schimbi tipul in HashMap<&str, i32>
> ```

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

## Solutii si observatii (sesiunea 2026-03-23)

### Ex1 — Vec: operatii de baza ✓

```rust
fn ex1() {
    let mut v: Vec<String> = vec!["Albastros".to_string(), "Murish".to_string(), "Gablov".to_string()];
    v.push(String::from("Cameros"));
    for (i, player) in v.iter().enumerate() {
        println!("Player {}: {}", i, player);
    }
    match v.pop() {
        None => { println!("No player"); },
        Some(scos) => { println!("Scos: {}", scos); },
    }
}
```

### Ex2 — Vec: acces sigur ✓

```rust
fn ex2() {
    let v: Vec<String> = vec!["Albastros".to_string(), "Murish".to_string(), "Gablov".to_string()];
    for (i, player) in v.iter().enumerate() {
        println!("Player {}: {}", i, player);
    }
    match v.get(3) {
        None => { println!("No player gasit la index 3."); },
        Some(gasit) => { println!("Gasit: {}", gasit); },
    }
}
```

> `v.get(3)` returneaza `None` deoarece Vec-ul are 3 elemente (indecsi 0, 1, 2).

### Ex3 — HashMap: registru de HP ✓

```rust
fn ex03() {
    let mut hp: HashMap<String, i32> = HashMap::new();
    hp.insert("Damos".to_string(), 100);
    hp.insert("Galos".to_string(), 22);
    hp.insert("Helios".to_string(), 55);

    match hp.get("Dragon") {
        None => { println!("No dragon"); },
        Some(viata) => { println!("Dragon: {}", viata); },
    }
    match hp.get("Damos") {
        None => { println!("No player found!"); },
        Some(viata) => { println!("Player: Damos : {}", viata); },
    }
}
```

### Ex4 — HashMap: entry si modificare ✓

```rust
fn ex04() {
    let mut hp: HashMap<String, i32> = HashMap::new();
    hp.insert("Damos".to_string(), 100);
    hp.insert("Galos".to_string(), 22);
    hp.insert("Helios".to_string(), 55);

    hp.entry("Davos".to_string()).or_insert(50);
    hp.entry("Davos".to_string()).or_insert(50); // al doilea apel nu schimba nimic

    match hp.remove("Davos") {
        None => { println!("No player found!"); },
        Some(scos) => { println!("Scos pe: {}", scos); },
    }
    for (i, (nume, viata)) in hp.iter().enumerate() {
        println!("Player {i}: Nume: {nume}: HP: {viata}");
    }
}
```

> Destructurarea `(i, (nume, viata))` in `enumerate()` pe HashMap — `i` e indexul, `(nume, viata)` e perechea cheie-valoare.

### Ex5 — Combinat: inventar + scor ✓

```rust
fn ex05() {
    let mut rng = rand::thread_rng();
    let inventar: Vec<&str> = vec!["Sabie", "Potiune", "Armura"];
    let mut scoruri: HashMap<&str, i32> = HashMap::new();

    // insereaza scoruri aleatoare per item
    for item in inventar.iter() {
        scoruri.insert(item, rng.gen_range(0..=100));
    }

    // afiseaza fiecare item cu scorul sau
    for item in inventar.iter() {
        match scoruri.get(item) {
            None => { println!("Item: {}, Scor: necunoscut", item); }
            Some(score) => { println!("Item: {}, Scor: {}", item, score); }
        }
    }
}
```

> **Observatie**: scorurile se genereaza intr-un prim loop, afisarea se face intr-un al doilea loop separat — separarea responsabilitatilor.

---

*Nota: Fisier creat in sesiunea din 2026-03-17. Solutii adaugate in sesiunea din 2026-03-23.*