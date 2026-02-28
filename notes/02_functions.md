# 02 — Functii, parametri si return values

## Ce este o functie?

O functie este un bloc de cod cu un **nume**, care primeste **date de intrare** (parametri), face ceva cu ele si (optional) **returneaza un rezultat**.

> **Analogie**: O functie e ca o **abilitate a unui personaj** intr-un RPG.
> Ii dai input-uri (tinta, putere de atac), executa niste pasi, si primesti un rezultat (damage infaptuit, loot).
> Abilitatea nu tine minte nimic intre utilizari — de fiecare data porneste de la zero.

---

## Sintaxa

```rust
fn nume_abilitate(parametru1: Tip1, parametru2: Tip2) -> TipReturn {
    // corp
}
```

- `fn` — cuvantul cheie pentru functie
- Tipurile parametrilor sunt **obligatorii** — Rust nu face inferenta la semnatura
- `-> Tip` — tipul valorii returnate (se omite daca functia nu returneaza nimic)

---

## Functie fara return — Abilitate fara loot

Unele abilitati nu dau nimic inapoi (de ex. un taunt).
Tipul return e `()` (se numeste **unit**). De obicei il omitem:

```rust
fn taunt() {
    println!("Vino sa ma prinzi!");
}
// echivalent cu: fn taunt() -> () { ... }
```

---

## Parametri — Inputurile abilitatii

Fiecare parametru are tipul sau explicit, separati prin virgula:

```rust
fn atac(damage: i32, armor: i32) -> i32 {
    damage - armor
}
```

---

## Expresii vs Instructiuni — Loot dat vs loot aruncat

Aceasta e una dintre cele mai importante distinctii in Rust.

| | Expresie | Instructiune |
|---|---|---|
| **Forma** | `damage - armor` | `damage - armor;` |
| **Produce valoare?** | DA — dai loot | NU — loot-ul e aruncat |
| **Semn distinctiv** | fara `;` | cu `;` la final |

> **Analogie**: Expresia e ca o abilitate care iti da loot dupa ce omori inamicul.
> Instructiunea e ca o abilitate care omoara inamicul dar **arunca loot-ul** —
> calculul s-a facut, dar tu nu ai primit nimic.

---

## Return implicit — Loot automat

Ultima **expresie** dintr-o functie este automat valoarea returnata:

```rust
fn damage_final(atac: i32, armor: i32) -> i32 {
    atac - armor   // fara ; — expresie, e returnata automat
}
```

Echivalent cu:

```rust
fn damage_final(atac: i32, armor: i32) -> i32 {
    return atac - armor; // return explicit — corect, dar mai putin idiomatic
}
```

**Greseala comuna**:

```rust
fn damage_final(atac: i32, armor: i32) -> i32 {
    atac - armor;  // EROARE: ; transforma expresia in instructiune
                   // functia nu mai returneaza nimic (returneaza ())
}
```

---

## if/else ca expresie — Alegi ce loot primesti

In Rust, `if/else` este o **expresie** — poate produce o valoare:

```rust
fn cel_mai_puternic(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}
```

Sau asignat la o variabila:

```rust
let castigator = if a > b { a } else { b };
```

**Regula**: Ambele ramuri ale `if/else` trebuie sa returneze **acelasi tip**.

---

## Return timpuriu — Iesi din dungeon daca esti aproape mort

Poti iesi din functie inainte de final cu `return` explicit:

```rust
fn stare_erou(hp: i32) -> &'static str {
    if hp <= 0 {
        return "mort";    // iesim imediat — nu mai verificam altceva
    }
    if hp < 20 {
        return "critic";
    }
    "in viata"   // return implicit pentru cazul ramas
}
```

---

## Functii si Ownership — Cine detine item-ul?

Tipul parametrului determina ce se intampla cu ownership-ul:

| Semnatura | Ce se intampla |
|---|---|
| `fn f(s: String)` | item **mutat** in functie — originalul invalid dupa apel |
| `fn f(s: &String)` | item **imprumutat** — originalul ramine valid |
| `fn f(s: &mut String)` | item **imprumutat cu modificare** — poate fi upgradat |
| `fn f(s: String) -> String` | item mutat si **returnat inapoi** |

```rust
let sabie = String::from("Excalibur");
inspecteaza(&sabie);        // imprumut — sabia ramane la noi
println!("{}", sabie);      // OK

let sabie2 = String::from("Durandal");
let sabie2_upgraded = upgradeaza(sabie2);  // sabie2 mutat in functie
println!("{}", sabie2_upgraded);           // OK — ownership returnat
// println!("{}", sabie2);                 // EROARE — sabie2 invalid
```

---

## Rezumat vizual

```
fn abilitate(damage: i32, armor: i32) -> i32 {
    let net = damage - armor;  // instructiune — salveaza
    net + 5;                    // instructiune — calculeaza si ARUNCA loot-ul
    net                         // expresie — RETURNEAZA loot-ul
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

**1.** Scrie `patrat(n: i32) -> i32` — damage-ul unui atac la patrat (`n * n`). Foloseste return implicit.

**2.** Scrie `saluta_erou(nume: &String)` — anunta eroul care a intrat in arena. Nu returneaza nimic.

**3.** Scrie `cel_mai_puternic(a: i32, b: i32) -> i32` — returneaza damage-ul mai mare. Foloseste `if/else` ca expresie.

---

*Nota: Fisier generat in sesiunea din 2026-02-24. Refacut cu tema jocuri video in 2026-02-28.*