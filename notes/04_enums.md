# 04 — Enumeratii (`enum`) si Pattern Matching

## Ce este un `enum`?

Un `enum` defineste un tip de date care poate fi **exact una** dintre mai multe variante posibile.

> **Analogie**: Gandeste-te la **clasa unui erou** intr-un RPG.
> Un erou poate fi `Warrior`, `Mage` sau `Archer` — si **nimic altceva**.
> Nu poate fi jumatate Warrior, jumatate Mage. Alegerea este exclusiva dintr-o lista fixa.
>
> Daca ai fi folosit `String`, nimic nu te impiedica sa scrii `"Wariror"` (typo) sau `""` (gol).
> Cu `enum`, compilatorul garanteaza ca valoarea e **mereu valida**.

---

## Definirea unui `enum`

```rust
enum Clasa {
    Warrior,
    Mage,
    Archer,
}
```

- Fiecare optiune se numeste **varianta**
- Variantele sunt separate prin virgula `,`
- Conventie: numele variantelor incep cu **litera mare** (PascalCase)

Folosesti o varianta cu `::`:

```rust
let clasa_mea = Clasa::Warrior;
```

---

## `match` cu `enum`

`match` si `enum` sunt facute unul pentru celalalt. `match` testeaza ce varianta detine valoarea:

```rust
match clasa_mea {
    Clasa::Warrior => println!("Folosesti sabia!"),
    Clasa::Mage    => println!("Arunci o vraja!"),
    Clasa::Archer  => println!("Tragi cu arcul!"),
}
```

**Regula importanta**: `match` trebuie sa acopere **toate** variantele.
Daca uiti una, **compilatorul refuza sa compileze**. Nu exista cazuri uitate.

---

## `enum` cu date asociate

Variantele unui `enum` pot contine **date**. Aceasta e una dintre cele mai puternice trasaturi ale Rust.

> **Analogie**: Un item dintr-un RPG poate fi o Sabie (cu damage), o Armura (cu defense) sau o Potion (cu heal).
> Fiecare tip de item are **atribute diferite** — dar toate sunt "item".

```rust
enum Item {
    Sabie(i32),          // contine damage ca i32
    Armura(i32),         // contine defense ca i32
    Potion(i32),         // contine heal ca i32
    QuestItem(String),   // contine numele quest-ului ca String
}
```

Cand faci `match`, extragi si datele:

```rust
let item = Item::Sabie(45);

match item {
    Item::Sabie(dmg)      => println!("Sabie cu {} damage", dmg),
    Item::Armura(def)     => println!("Armura cu {} defense", def),
    Item::Potion(heal)    => println!("Potion cu {} heal", heal),
    Item::QuestItem(nume) => println!("Quest item: {}", nume),
}
```

---

## `enum` in `struct`

Un camp al unui struct poate fi de tip `enum`:

```rust
struct Erou {
    nume: String,
    clasa: Clasa,    // enum in loc de String
    hp: i32,
}
```

Acum compilatorul garanteaza ca `clasa` e intotdeauna o valoare valida:

```rust
let erou = Erou {
    nume: String::from("Arthas"),
    clasa: Clasa::Warrior,
    hp: 100,
};
```

---

## `impl` pe `enum`

Poti adauga metode pe un `enum`, la fel ca pe un `struct`:

```rust
impl Clasa {
    fn bonus_damage(&self) -> i32 {
        match self {
            Clasa::Warrior => 10,
            Clasa::Mage    => 20,
            Clasa::Archer  => 15,
        }
    }
}

// Apel:
let bonus = Clasa::Mage.bonus_damage();  // 20
```

---

## Varianta `_` — cazul default

Daca nu vrei sa tratezi toate variantele individual, folosesti `_` ca wildcard:

```rust
match clasa_mea {
    Clasa::Mage => println!("Magic user!"),
    _           => println!("Clasa fizica"),   // orice altceva
}
```

---

## `Option<T>` — enum-ul din standard library

`Option` este un `enum` built-in in Rust, si il vei intalni foarte des:

```rust
enum Option<T> {
    Some(T),   // contine o valoare de tip T
    None,      // nu contine nimic
}
```

> **Analogie**: E ca un **slot de inventar**. Poate contine un item (`Some(item)`) sau poate fi gol (`None`).
> Rust nu are `null` — in schimb foloseste `Option` pentru a reprezenta lipsa unei valori.

Ai folosit deja `Option` in exercitiul 3, la `use_potion()`:
```rust
let index = self.inventar.iter().position(|item| item.contains("Potion"));
// index e de tip Option<usize>
match index {
    Some(i) => { /* gasit */ },
    None    => { /* negasit */ },
}
```

---

## Rezumat vizual

```
enum Clasa {                    ← definesti variantele
    Warrior,
    Mage,
    Archer,
}

enum Item {
    Sabie(i32),                 ← varianta cu date asociate
    QuestItem(String),
}

impl Clasa {
    fn bonus(&self) -> i32 {   ← metode pe enum
        match self {            ← match acopera TOATE variantele
            Clasa::Warrior => 5,
            Clasa::Mage    => 10,
            Clasa::Archer  => 7,
        }
    }
}

let c = Clasa::Mage;           ← creare cu ::
let b = c.bonus();             ← apel metoda cu .
```

---

## Reguli de memorat

| # | Regula |
|---|--------|
| 1 | `enum` defineste un set fix de variante — valoarea e mereu una dintre ele |
| 2 | Variantele se acceseaza cu `::` |
| 3 | `match` pe `enum` trebuie sa acopere **toate** variantele |
| 4 | Variantele pot contine date de orice tip |
| 5 | `Option<T>` e un `enum` built-in: `Some(valoare)` sau `None` |

---

## Exercitii

Fisier de lucru: `exercitii/src/bin/ex_04_enum.rs`
Rulare: `cargo run --bin ex_04_enum`

Exercitiu cumulativ — combina **enum** (cap 4) cu **Struct + impl** (cap 3), **Functii** (cap 2) si **Ownership** (cap 1).

---

### Pasul 1 — Defineste enum-urile

**1.** `enum Clasa` cu cel putin 3 variante (alege tu clasele RPG preferate)

**2.** `enum Rasa` cu cel putin 3 variante (ex: Elf, Human, Orc, Dwarf...)

---

### Pasul 2 — Struct cu enum-uri

**3.** Rescrie `struct Erou` folosind `Clasa` si `Rasa` in loc de `String` pentru acele campuri.

---

### Pasul 3 — Metode pe enum

**4.** `impl Clasa` — adauga metoda `bonus_damage(&self) -> i32` care returneaza un bonus diferit per clasa.

**5.** `impl Rasa` — adauga metoda `bonus_hp(&self) -> i32` care returneaza HP bonus per rasa (ex: Orc +20, Elf +5, Human +10).

---

### Pasul 4 — Integreaza in Erou

**6.** In `impl Erou`, modifica constructorul `new(nume: &str, clasa: Clasa, rasa: Rasa) -> Erou`.
> Atentie: acum `clasa` si `rasa` sunt primite ca valori de tip `enum`, nu `&str`.
> HP-ul initial sa fie `100 + rasa.bonus_hp()`.

**7.** Metoda `info(&self)` — afiseaza toate campurile. Pentru `Clasa` si `Rasa`, foloseste `{:?}` (vom explica `Debug` mai jos).

---

### Pasul 5 — Inventar cu `enum Item`

**8.** Defineste `enum Item` cu cel putin 3 variante care contin date:
- `Sabie(i32)` — damage
- `Armura(i32)` — defense
- `Potion(i32)` — heal

**9.** Metoda `pick_up(&mut self, item: Item)` — adauga itemul in inventar.
> Schimba `Vec<String>` in `Vec<Item>` in struct.

**10.** Metoda `use_item(&mut self)` — cauta primul `Potion` din inventar, il scoate si adauga HP-ul asociat.
> Indiciu: `match` pe varianta extrasa din vector.

---

### Bonus — `#[derive(Debug)]`

Adauga `#[derive(Debug)]` deasupra fiecarui `enum` si `struct` pentru a putea folosi `{:?}` in `println!`:

```rust
#[derive(Debug)]
enum Clasa {
    Warrior,
    Mage,
    Archer,
}
```

---

*Nota: Fisier creat in sesiunea din 2026-03-08.*