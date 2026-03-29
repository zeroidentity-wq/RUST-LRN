# 08 — Closures si Iteratori

---

## Partea 1: Closures

### De ce ai nevoie de closures?

Pana acum, daca voiai sa aplici o operatie pe o lista, scriai un `for` complet. Dar in Rust exista un mecanism mai expresiv: **closures** — functii anonime pe care le scrii exact acolo unde ai nevoie de ele.

De fapt, ai vazut deja un closure fara sa-i spunem pe nume — in exemplul cu `.collect()`:
```rust
.filter(|c| !c.is_numeric())
```
Acea expresie `|c| !c.is_numeric()` este un closure.

---

### Ce este un closure?

Un **closure** este o functie anonima (fara `fn` si fara nume) pe care o poti salva intr-o variabila sau trimite ca argument altei functii.

> **Analogie**: Gandeste-te la un closure ca la un **bilet cu instructiuni** pe care il scrii pe loc si il dai cuiva. Nu trebuie sa mergi la birou, sa completezi un formular oficial (`fn`) si sa-i dai un nume — scrii biletul si il dai imediat.

**Superputerea fata de functii normale:** Un closure poate **vedea variabilele din contextul in care a fost creat**. O functie `fn` clasica nu poate face asta.

---

### Sintaxa: closure vs functie normala

```rust
// Functie normala — declarata separat, tipuri explicite obligatorii
fn aduna_cinci(x: i32) -> i32 { x + 5 }

// Closure echivalent — inline, tipurile se deduc automat
let aduna_cinci = |x| x + 5;

// Apelul este identic in ambele cazuri
println!("{}", aduna_cinci(10)); // 15
```

Closures folosesc `|parametri|` in loc de `(parametri)`. Rust deduce tipurile automat.

---

### Formele closures

**1. Capturarea mediului** — closure-ul vede variabile din afara lui:

```rust
let taxa_livrare = 15;

let calculeaza_total = |pret| pret + taxa_livrare; // captureaza taxa_livrare

println!("{}", calculeaza_total(50));  // 65
println!("{}", calculeaza_total(100)); // 115
```

`taxa_livrare` nu este parametru — closure-ul il imprumuta din context. O functie `fn` ar da eroare de compilare aici.

---

**2. Closure mutabil** — daca modifica o variabila din exterior:

```rust
let mut contor = 0;

let mut incrementeaza = || {   // mut pe closure deoarece modifica `contor`
    contor += 1;
    println!("Contor: {}", contor);
};

incrementeaza(); // Contor: 1
incrementeaza(); // Contor: 2
```

> **Regula**: `mut` pe closure = closure-ul schimba ceva din afara lui.

---

**3. Closure ca argument** — trimis direct unei functii:

```rust
let v = vec![1, 2, 3, 4, 5];

// Closure-ul |n| n * 2 este trimis ca argument lui .map()
let duble: Vec<i32> = v.iter().map(|n| n * 2).collect();
```

Acesta este cazul cel mai frecvent in practica — closures ca argumente pentru metodele de iterare.

---

## Partea 2: Iteratori

### Ce este un iterator?

Un **iterator** este un obiect care produce valori **una cate una**, la cerere. Closures si iteratorii merg mana in mana: metodele de iterare primesc un closure care descrie *ce* sa faca cu fiecare valoare.

> **Analogie**: O **linie de asamblare**. Produsele inainteaza pe banda rulanta statie cu statie: la prima sunt vopsite (`.map()`), la a doua rejectele sunt scoase (`.filter()`), la final sunt puse in cutie (`.collect()`).
>
> **Banda nu porneste** pana nu apesi butonul de start — acesta este `.collect()` sau orice alt consumator. Pana atunci, nu se intampla nimic.

Aceasta proprietate se numeste **evaluare lenesa** (lazy evaluation).

---

### Cum obtii un iterator?

| Metoda | Ownership | Elementul in closure |
|--------|-----------|----------------------|
| `.iter()` | Imprumut | `&T` |
| `.iter_mut()` | Imprumut mutabil | `&mut T` |
| `.into_iter()` | Consuma colectia | `T` |

```rust
let v = vec![1, 2, 3];

for x in v.iter()      { /* x: &i32,     v ramane valid */ }
for x in v.iter_mut()  { /* x: &mut i32, poti modifica in-place */ }
// for x in v.into_iter() { /* x: i32,   v NU mai exista dupa */ }
```

**Regula practica**: `.iter()` in 90% din cazuri. `.into_iter()` doar cand vrei sa transformi colectia intr-una noua si nu mai ai nevoie de original.

---

### De ce `&x` si `&&x`? — straturile de referinta

`.iter()` produce `&T` (referinta). Dar `.map()` si `.filter()` trateaza acea referinta diferit:

- **`.map()`** primeste elementul **direct** (il consuma/transforma) → `&T`
- **`.filter()`** primeste o **referinta la element** (doar il inspecteaza, nu il consuma) → `&&T`

```
Vec<i32> → .iter() → &i32
                       │
                       ├── .map(|x| ...)     → x = &i32   (un strat)
                       │
                       └── .filter(|x| ...)  → x = &&i32  (doua straturi)
```

Daca colectia contine deja referinte (ex: `Vec<&str>`), se adauga inca un strat:

```
Vec<&str> → .iter() → &&str
                        │
                        ├── .map(|x| ...)     → x = &&str   (doua straturi)
                        │
                        └── .filter(|x| ...)  → x = &&&str  (trei straturi)
```

**Cum scapi de straturile extra?** Trei variante echivalente:

```rust
// 1. Destructurare in pattern (recomandat)
.filter(|&&x| x > 2)     // &&x destructureaza &&i32 → x devine i32

// 2. Dereferentiere cu *
.filter(|x| **x > 2)     // **x: de la &&i32 la i32

// 3. Auto-deref la comparatie (Rust e inteligent)
.filter(|x| *x > 2)      // functioneaza pentru operatori ca >, ==, etc.
```

> **De retinut**: `.map()` transforma, deci primeste elementul direct. `.filter()` doar inspecteaza, deci ia o referinta in plus — de-asta apare `&&`.

---

### Evaluare lenesa — cheia intelegerii

> **Regula de aur**: Daca doar creezi un iterator si ii spui ce sa faca, el **nu va face absolut nimic** pana cand nu adaugi un consumator care sa "porneasca banda". Adaptoarele descriu planul; consumatorul il executa.

Adaptoarele nu fac nimic singure. Ele construiesc un plan. Procesarea are loc abia la consumator:

```rust
let v = vec![1, 2, 3, 4, 5];

// Aceasta linie NU proceseaza inca nimic
let plan = v.iter().map(|x| x * 2).filter(|x| x > &4);

// Abia .collect() porneste banda
let rezultat: Vec<i32> = plan.collect();
println!("{:?}", rezultat); // [6, 8, 10]
```

---

### Adaptoare (lazy — transforma iteratorul)

`.map()` — transforma fiecare element:
```rust
let duble: Vec<i32> = v.iter().map(|x| x * 2).collect();
```

`.filter()` — pastreaza elementele care trec conditia:
```rust
let pare: Vec<&i32> = v.iter().filter(|&&x| x % 2 == 0).collect();
```
> `&&x` in filter: `.iter()` da `&i32`, filter primeste `&&i32` — pattern `&&x` destructureaza ambele niveluri.

`.take(n)` / `.skip(n)` — limiteaza sau sare elemente:
```rust
let primele_trei: Vec<&i32> = v.iter().take(3).collect();
let fara_primul: Vec<&i32>  = v.iter().skip(1).collect();
```

`.enumerate()` — adauga index automat:
```rust
for (i, val) in v.iter().enumerate() {
    println!("[{}] = {}", i, val);
}
```

`.zip()` — combina doi iteratori in perechi:
```rust
let nume   = vec!["Ana", "Bob"];
let scoruri = vec![95, 80];
let perechi: Vec<(&&str, &i32)> = nume.iter().zip(scoruri.iter()).collect();
// [("Ana", 95), ("Bob", 80)]
```

`.chain()` — concateneaza doi iteratori:
```rust
let tot: Vec<&i32> = a.iter().chain(b.iter()).collect();
```

`.inspect()` — "spioneaza" fiecare element fara sa-l modifice (util pentru debug):
```rust
let rezultat: Vec<i32> = preturi.iter()
    .inspect(|x| println!("Inainte: {}", x))
    .map(|x| x + taxa)
    .inspect(|x| println!("Dupa: {}", x))
    .collect();
```
> `.inspect()` primeste o referinta la element si returneaza elementul neschimbat. Poti pune `.inspect()` oriunde in lant.

**Alternativa: closure cu bloc `{}`** — daca vrei sa faci mai multe operatii intr-un `.map()`:
```rust
.map(|x| {
    println!("Procesez: {} -> {}", x, x + taxa);
    x + taxa  // ultima expresie FARA ; = valoarea returnata
})
```
> Ultimul rand fara `;` este valoarea pe care closure-ul o returneaza — exact ca la functii.

---

### Consumatori (eager — produc valoarea finala)

| Metoda | Ce returneaza | Exemplu |
|--------|---------------|---------|
| `.collect()` | `Vec<T>`, `HashMap`, etc. | aduna rezultatele |
| `.count()` | `usize` | numara elementele |
| `.sum()` | `T` | aduna valorile numerice |
| `.any(\|x\| ...)` | `bool` | cel putin unul trece conditia |
| `.all(\|x\| ...)` | `bool` | toate trec conditia |
| `.find(\|x\| ...)` | `Option<&T>` | primul element potrivit |
| `.for_each(\|x\| ...)` | `()` | executa o actiune, fara rezultat |
| `.max()` / `.min()` | `Option<&T>` | valoarea maxima / minima |

```rust
let v = vec![1, 2, 3, 4, 5];

let suma: i32       = v.iter().sum();                           // 15
let nr_pare         = v.iter().filter(|&&x| x % 2 == 0).count(); // 2
let are_mare        = v.iter().any(|&x| x > 4);                // true
let primul_par      = v.iter().find(|&&x| x % 2 == 0);         // Some(2)
```

---

### Inlantuirea metodelor

Puterea reala apare cand combini mai multe operatii:

```rust
let note = vec![3, 7, 5, 9, 2, 8, 4, 6];

// Suma notelor de trecere (>= 5), doar primele 3
let rezultat: i32 = note.iter()
    .filter(|&&n| n >= 5)  // pastreaza notele de trecere
    .take(3)               // primele 3 dintre ele
    .sum();                // aduna

println!("{}", rezultat);  // 7 + 5 + 9 = 21
```

---

### Stil imperativ vs. stil idiomatic

Acelasi rezultat se poate scrie in doua feluri. Intelege diferenta — in Rust, stilul cu iteratori este preferat.

```rust
let numere = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// IMPERATIV (for clasic) — spui CUM sa faca
let mut patrate_pare = Vec::new();
for &n in &numere {
    if n % 2 == 0 {
        patrate_pare.push(n * n);
    }
}

// IDIOMATIC (iteratori) — spui CE vrei
let patrate_pare: Vec<i32> = numere
    .iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * x)
    .collect();
```

Ambele produc `[4, 16, 36, 64, 100]`. De ce e mai bun stilul idiomatic?

1. **Fara `mut`** — variabila rezultat nu e mutabila. Cu cat mai putine variabile mutabile, cu atat mai putine bug-uri posibile.
2. **Se citeste ca o propozitie** — "ia numerele → filtreaza parele → transforma in patrate → colecteaza". Logica e vizibila direct in cod.
3. **Performanta** — compilatorul Rust optimizeaza lanturile de iteratori, eliminand overhead-ul buclei manuale.

---

### Range-uri ca iteratori

`1..=10` este deja un iterator — nu ai nevoie de Vec:

```rust
let suma: i32      = (1..=100).sum();                           // 5050
let patrate: Vec<i32> = (1..=5).map(|x| x * x).collect();     // [1, 4, 9, 16, 25]
let count          = (1..=1000).filter(|x| x % 7 == 0).count(); // 142
```

---

## Rezumat

### Closures

| Concept | Sintaxa |
|---------|---------|
| Closure simplu | `\|x\| x + 1` |
| Closure cu bloc | `\|x\| { let y = x + 1; y }` |
| Captureaza mediul | `\|x\| x + variabila_externa` |
| Closure mutabil | `let mut f = \|\| { contor += 1; }` |
| Closure ca argument | `.map(\|n\| n * 2)` |

### Iteratori — Intrare

| Metoda | Elementul primit |
|--------|-----------------|
| `.iter()` | `&T` — imprumut |
| `.iter_mut()` | `&mut T` — imprumut mutabil |
| `.into_iter()` | `T` — consuma colectia |

### Adaptoare (lazy)

| Metoda | Efect |
|--------|-------|
| `.map(\|x\| ...)` | Transforma fiecare element |
| `.filter(\|x\| ...)` | Pastreaza elementele care trec conditia |
| `.take(n)` / `.skip(n)` | Limiteaza / sare elemente |
| `.enumerate()` | Adauga index `(usize, &T)` |
| `.zip(iter)` | Combina in perechi `(T, U)` |
| `.chain(iter)` | Concateneaza doi iteratori |
| `.inspect(\|x\| ...)` | Debug — vizualizeaza elementul fara a-l modifica |

### Consumatori (eager)

| Metoda | Returneaza |
|--------|------------|
| `.collect()` | `Vec<T>`, etc. |
| `.count()` | `usize` |
| `.sum()` | `T` |
| `.any()` / `.all()` | `bool` |
| `.find()` | `Option<&T>` |
| `.for_each()` | `()` |
| `.max()` / `.min()` | `Option<&T>` |

---

## Exercitii

Fisier de lucru: `exercitii/src/bin/ex_08_closures.rs`
Rulare: `cargo run --bin ex_08_closures`
