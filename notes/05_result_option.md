# 05 — Tratarea erorilor: `Option` si `Result`

## De ce nu exista `null` in Rust?

In majoritatea limbajelor, orice variabila poate fi `null`.
Daca uiti sa verifici, programul crapa la **runtime** — uneori dupa ore de rulare in productie.

Rust elimina `null` complet. In schimb, ai doua enum-uri built-in care te **forteaza sa recunosti** la compilare ca ceva poate lipsi sau poate esua.

---

## `Option<T>` — o valoare care poate lipsi

> **Analogie**: Esti intr-un RPG si vrei sa echipezi sabia din inventar.
> Fie o gasesti (`Some(sabie)`), fie slotul e gol (`None`).
> Jocul nu iti da un "sabie null" — iti spune clar daca exista sau nu.

```rust
enum Option<T> {
    Some(T),   // exista o valoare de tip T
    None,      // nu exista nimic
}
```

`T` e un **tip generic** — `Option<i32>`, `Option<String>`, `Option<Item>` etc.
Nu trebuie sa importi `Option` — e disponibil automat.

### Exemplu

```rust
fn cauta_sabie(inventar: &Vec<Item>) -> Option<i32> {
    for item in inventar {
        if let Item::Sabie(dmg) = item {
            return Some(*dmg);
        }
    }
    None
}
```

---

## Cum tratezi un `Option`

### 1. `match` — cel mai explicit

```rust
match cauta_sabie(&inventar) {
    Some(dmg) => println!("Sabie cu {} damage", dmg),
    None      => println!("Nu ai sabie"),
}
```

### 2. `if let` — cand te intereseaza doar cazul `Some`

```rust
if let Some(heal) = cauta_potion(&inventar) {
    println!("Potion cu {} HP", heal);
}
// Daca e None, nu se intampla nimic — nu ai nevoie de else
```

Poti adauga si `else` daca vrei sa tratezi cazul `None`:

```rust
if let Some(heal) = cauta_potion(&inventar) {
    println!("Potion cu {} HP", heal);
} else {
    println!("Nu ai nicio potion.");
}
```

### 3. `.unwrap_or(val)` — valoare default daca e `None`

```rust
let dmg = cauta_sabie(&inventar).unwrap_or(0);
// Daca nu ai sabie, dmg = 0
```

### Alte metode utile

| Metoda | Ce face |
|--------|---------|
| `.unwrap_or(val)` | Returneaza valoarea sau `val` daca e `None` |
| `.is_some()` | `true` daca e `Some` |
| `.is_none()` | `true` daca e `None` |
| `.unwrap()` | Extrage valoarea, **panic** daca e `None` — evita in productie! |

---

## `Result<T, E>` — o operatie care poate esua

> **Analogie**: Incerci sa ataci un inamic. Fie reusesti si faci damage (`Ok(damage)`),
> fie ceva merge prost — esti mort, n-ai mana — si primesti o eroare (`Err("mesaj")`).
> Jocul nu te lasa sa ignori eroarea.

```rust
enum Result<T, E> {
    Ok(T),    // succes — contine rezultatul de tip T
    Err(E),   // esec   — contine eroarea de tip E
}
```

### Exemplu

```rust
fn ataca(atacator: &Erou, tinta: &Erou) -> Result<i32, String> {
    if atacator.hp <= 0 {
        return Err(format!("{} este mort!", atacator.nume));
    }
    let dmg = 10 + damage_sabie(&atacator.inventar);
    Ok(dmg)
}
```

### `match` pe `Result`

```rust
match ataca(&erou, &goblin) {
    Ok(dmg)  => println!("Damage facut: {}", dmg),
    Err(msg) => println!("Eroare: {}", msg),
}
```

---

## Operatorul `?` — propagarea erorii

In loc sa scrii `match` peste tot, `?` face automat:
- daca e `Ok(val)` — extrage `val` si continua
- daca e `Err(e)` — returneaza imediat `Err(e)` din functia curenta

```rust
fn quest_atac(atacator: &Erou, tinta: &Erou) -> Result<String, String> {
    let dmg = ataca(atacator, tinta)?;  // iese automat daca e Err

    // Ajungem aici DOAR daca ataca() a reusit
    Ok(format!("{} face {} damage si completeaza quest-ul!", atacator.nume, dmg))
}
```

> **Regula**: `?` functioneaza doar in functii care returneaza `Result` (sau `Option`).

---

## `Option` vs `Result` — cand le folosesti

| Situatie | Folosesti |
|----------|-----------|
| Cauti ceva care poate sa nu existe | `Option<T>` |
| O operatie care poate esua din mai multe motive | `Result<T, E>` |
| Lipsa valorii e normala (nu e o eroare) | `Option<T>` |
| Vrei sa comunici DE CE a esuat | `Result<T, E>` |

---

## Rezumat vizual

```
Option<T>                          Result<T, E>
─────────────────────              ─────────────────────────────
Some(valoare)  ← exista            Ok(valoare)   ← succes
None           ← lipseste          Err(eroare)   ← esec cu motiv

match opt {                        match res {
    Some(v) => { ... },                Ok(v)  => { ... },
    None    => { ... },                Err(e) => { ... },
}                                  }

if let Some(v) = opt { ... }       if let Ok(v) = res { ... }

opt.unwrap_or(0)                   res.unwrap_or(0)
```

---

## Reguli de memorat

| # | Regula |
|---|--------|
| 1 | `Option<T>` = valoare care poate lipsi; `Result<T, E>` = operatie care poate esua |
| 2 | `match` trebuie sa acopere ambele variante (`Some`/`None` sau `Ok`/`Err`) |
| 3 | `if let Some(v) = opt` — sintaxa scurta cand te intereseaza doar un caz |
| 4 | `.unwrap_or(val)` — valoare default in loc de `None`/`Err` |
| 5 | `?` — propaga automat `Err` din functia curenta |
| 6 | `.unwrap()` fara verificare poate face `panic!` — evita-l |

---

## Exercitii

Fisier de lucru: `exercitii/src/bin/ex_05_result_option.rs`
Rulare: `cargo run --bin ex_05_result_option`

Exercitiu cumulativ — combina **Option/Result** (cap 5) cu **enum** (cap 4), **struct + impl** (cap 3) si **functii** (cap 2).

---

### Pasul 1 — `Option`: cauta in inventar

**1.** Scrie `cauta_sabie` — returneaza damage-ul primei sabii din inventar, sau `None`

**2.** Scrie `cauta_potion` — similar, dar pentru potions

---

### Pasul 2 — `Result`: atac care poate esua

**3.** Scrie `ataca` — returneaza damage-ul facut sau o eroare cu motiv
> Gandeste-te: ce conditii pot face un atac sa esueze?

---

### Pasul 3 — Operatorul `?`

**4.** Scrie `executa_quest` — apeleaza `ataca()` si propaga eroarea cu `?`

---

### Pasul 4 — Testeaza in `main()`

**5.** Testeaza fiecare functie folosind `match`, `if let` si `unwrap_or`

---

*Nota: Fisier creat in sesiunea din 2026-03-14.*
