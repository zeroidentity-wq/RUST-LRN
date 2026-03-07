# 03 — Struct-uri si implementari

## Ce este un struct?

Un struct este un **tip de date custom** care grupeaza mai multe valori sub un singur nume.

> **Analogie**: Un struct e ca o **fisa de personaj** intr-un RPG (Diablo, WoW, D&D).
> In loc sa tii variabile separate (`hp`, `mana`, `nivel`, `nume`) prin tot codul,
> le pui impreuna intr-un singur tip — `Personaj`. Ai mereu "pachetul complet".

---

## Definirea unui struct

```rust
struct Personaj {
    nume: String,
    hp: i32,
    mana: i32,
    nivel: u32,
}
```

- Fiecare linie din corp se numeste **camp** (field)
- Fiecare camp are un **nume** si un **tip**, separate prin `:`
- Campurile sunt separate prin virgula `,`

---

## Crearea unei instante

```rust
let erou = Personaj {
    nume: String::from("Arthas"),
    hp: 100,
    mana: 80,
    nivel: 5,
};
```

- Trebuie sa specifici **toate campurile** la creare
- Ordinea nu conteaza, dar toate trebuie prezente

**Acces la campuri** — cu punct `.`:

```rust
println!("{} are {} HP", erou.nume, erou.hp);
```

---

## Struct mutabil

Ca sa poti modifica campurile, instanta trebuie declarata cu `mut`:

```rust
let mut razboinic = Personaj { ... };
razboinic.hp -= 30;  // OK — instanta e mut
```

**Regula Rust**: In Rust, fie TOATE campurile sunt mutabile, fie NICIUNUL.
Nu poti marca doar un camp ca `mut` — muti intreaga instanta.

---

## impl — Abilitatile structului

`impl` (implementation) adauga **metode** unui struct.
E ca pagina de abilitati atasata fisei de personaj.

```rust
impl Personaj {
    fn descrie(&self) {
        println!("[{}] HP: {}", self.nome, self.hp);
    }
}
```

Apelul se face cu `.`:
```rust
erou.descrie();
```

---

## Tipuri de metode in impl

| Semnatura | Rol | Cand il folosesti |
|---|---|---|
| `fn f(&self)` | **Citire** — nu modifica nimic | Afisare, calcule |
| `fn f(&mut self)` | **Modificare** — poate schimba campuri | Damage, healing |
| `fn f(...) -> Tip` | **Functie asociata** — fara self, constructor | `Personaj::new(...)` |

---

## &self — Inspectie

Metoda primeste o **referinta imutabila** la instanta — poate citi, nu poate modifica:

```rust
fn descrie(&self) {
    println!("[{}] Nivel {} | HP: {}", self.nume, self.nivel, self.hp);
}

fn calculeaza_damage(&self, baza: i32) -> i32 {
    baza + self.nivel as i32 * 10
}
```

---

## &mut self — Actiune

Metoda primeste o **referinta mutabila** — poate modifica campurile.
Instanta pe care o apelezi trebuie sa fie `mut`:

```rust
fn bea_potion(&mut self) {
    self.hp += 50;
}

// la apel:
let mut erou = Personaj::new("Uther", 5);
erou.bea_potion();  // OK — erou e mut
```

---

## Functii asociate — Constructori

O functie asociata **nu primeste `self`** — nu e legata de o instanta existenta.
Cel mai frecvent rol: **constructor** care creeaza o instanta noua.

```rust
fn new(nume: &str, nivel: u32) -> Personaj {
    Personaj {
        nume: String::from(nume),
        hp: 100,
        mana: 50,
        nivel,      // shorthand pentru `nivel: nivel`
    }
}
```

Apelul se face cu `::` (nu cu `.`):
```rust
let mag = Personaj::new("Jaina", 12);
//                   ^^ :: nu .
```

> **De ce `::`?** — Functiile asociate apartin **tipului** (`Personaj`), nu unei instante.
> Metodele (cu `self`) apartin **instantei** (`erou`, `mag`).

---

## Struct in struct

Campurile unui struct pot fi ele insele struct-uri:

```rust
struct Echipa {
    lider: Personaj,
    dimensiune: u32,
}

let echipa = Echipa {
    lider: Personaj::new("Sylvanas", 30),
    dimensiune: 5,
};

println!("{}", echipa.lider.nume);  // acces in lant
```

---

## Rezumat vizual

```
struct Personaj {           ← definesti structura
    hp: i32,
    nivel: u32,
}

impl Personaj {
    fn new(...) -> Personaj { ... }   ← constructor (fara self)
    fn descrie(&self) { ... }         ← citire (cu &self)
    fn bea_potion(&mut self) { ... }  ← modificare (cu &mut self)
}

let mut erou = Personaj::new("X", 5);  ← creare cu ::
erou.descrie();                         ← apel metoda cu .
erou.bea_potion();                      ← apel metoda mut cu .
```

---

## Reguli de memorat

| # | Regula |
|---|--------|
| 1 | `struct` grupeaza campuri — toate trebuie specificate la creare |
| 2 | Pentru a modifica campuri, instanta trebuie sa fie `mut` |
| 3 | `impl` adauga metode structului |
| 4 | `&self` = citire / `&mut self` = modificare |
| 5 | Fara `self` = functie asociata, apelata cu `::` |

---

## Exercitii

Fisier de lucru: `exercitii/src/bin/ex_03_struct.rs`
Rulare: `cargo run --bin ex_03_struct`

Exercitiu cumulativ — combina **Struct + impl** (cap 3) cu **Functii** (cap 2) si **Ownership & Borrowing** (cap 1).

---

### Struct `Erou` — metode (impl)

**1.** `new(nume: &str, clasa: &str) -> Erou` — constructor, valorile initiale: hp=100, nivel=1, inventar gol.

**2.** `info(&self)` — afiseaza toate campurile, inclusiv inventarul.

**3.** `pick_up(&mut self, item: String)` — preia ownership-ul unui item si il adauga in inventar.
> De ce `String` si nu `&String`? — ca eroul sa **detina** itemul, nu doar sa il inspecteze.

**4.** `take_damage(&mut self, dmg: i32)` — scade dmg din HP; daca HP <= 0, afiseaza `"<nume> a cazut in lupta!"`.
> Atentie la ordine: scade **inainte** de verificare.

**5.** `is_alive(&self) -> bool` — returneaza `true` daca HP > 0.

**6.** `use_potion(&mut self)` — cauta primul item care contine `"Potion"`, il scoate din inventar si da +30 HP. Daca nu exista, afiseaza mesaj.
> Indiciu: `.iter().position(|item| item.contains("Potion"))` returneaza `Option<usize>`.

---

### Functii libere

**7.** `putere_totala(erou: &Erou) -> i32` — `nivel * 10 + nr_iteme * 5`. Primeste imprumut imutabil.

**8.** `ataca(atacator: &Erou, tinta: &mut Erou)` — damage = `nivel_atacator * 5`. Apeleaza `take_damage` pe tinta.
> De ce `&Erou` si `&mut Erou`? — atacatorul doar citeste, tinta se modifica.

---

### Solutii complete

```rust
use rand::Rng;

struct Erou {
    nume: String,
    clasa: String,
    hp: i32,
    nivel: i32,       // ales i32 in loc de u32 pentru compatibilitate cu calcule de damage
    inventar: Vec<String>,
}

impl Erou {
    fn new(nume: &str, clasa: &str) -> Erou {
        Erou {
            nume: String::from(nume),
            clasa: String::from(clasa),
            hp: 100,
            nivel: 1,
            inventar: Vec::new(),
        }
    }

    // Extra: info() afiseaza si putere_totala, apeland functia libera
    fn info(&self) {
        println!("[{} | {}] HP: {} | Nivel: {} | Inventar: {:?}, putere_totala: {}",
            self.nume, self.clasa, self.hp, self.nivel, self.inventar, putere_totala(self));
    }

    fn pick_up(&mut self, item: String) {
        self.inventar.push(item);
        // item va fi owner al valorii in cadrul functiei pana va fi facut push in vector
        // din acest moment vectorul detine item-ul in inventar
    }

    fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
        if self.hp <= 0 {
            println!("Erou {} a cazut in lupta!", self.nume);
        }
    }

    fn is_alive(&self) -> bool {
        if self.hp > 0 { true } else { false }
        // Nota: se poate simplifica la: self.hp > 0
    }

    fn use_potion(&mut self) {
        let index = self.inventar.iter().position(|item| item.contains("Potion"));
        match index {
            Some(potiune) => {
                self.inventar.remove(potiune);
                self.hp += 30;
            }
            None => println!("Nu ai nici o potiune in inventar!"),
        }
    }
}

fn putere_totala(erou: &Erou) -> i32 {
    erou.nivel * 10 + erou.inventar.len() as i32 * 5
    // &Erou si nu Erou — nu preluam ownership, doar citim
}

// Extra: damage random in loc de nivel * 5 fix
fn ataca(atacator: &Erou, tinta: &mut Erou) {
    let damage = rand::thread_rng().gen_range(1..=20) * atacator.nivel;
    println!("{} ataca {} => {} damage.", atacator.nume, tinta.nume, damage);
    tinta.take_damage(damage);
    // atacator: &Erou    — doar citim nivelul
    // tinta: &mut Erou   — modificam hp-ul
}

fn main() {
    let mut erou_1 = Erou::new("Arthemus", "Demi-Wolf");
    erou_1.info();
    let mut erou_2 = Erou::new("Davous", "Archer");
    erou_2.info();

    erou_1.pick_up(String::from("Potion"));
    erou_2.pick_up(String::from("Potion"));

    ataca(&erou_1, &mut erou_2);

    if erou_2.is_alive() {
        println!("Erou {} is alive!", erou_2.nume);
    }

    erou_2.info();
    erou_2.use_potion();
    erou_2.info();
}
```

---

*Nota: Fisier generat in sesiunea din 2026-02-28. Exercitii rescrise cumulativ in sesiunea din 2026-03-01. Solutii complete adaugate in sesiunea din 2026-03-07.*
