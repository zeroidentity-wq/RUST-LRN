// Exercitii 08 — Closures si Iteratori
// Rulare: cargo run --bin ex_08_closures

// ============================================================
// EXERCITIUL 1 — Closure cu capturarea mediului
// ============================================================
// Ai un pret de baza si o taxa de transport definite ca variabile.
// Scrie un closure `calculeaza_total` care le captureaza pe amandoua
// si returneaza pretul final (pret + taxa).
//
// Aplica-l pe urmatoarele preturi si afiseaza rezultatele:
//   preturi: [30, 75, 120, 5, 200]
// Output asteptat (cu taxa = 12):
//   30  -> 42
//   75  -> 87
//   ...
// ============================================================

fn ex1() {
    let taxa_transport = 12;
    let preturi = vec![30, 75, 120, 5, 200];

    // scrie closure-ul si aplica-l pe lista de preturi
    let pret_final: Vec<i32> = preturi.iter()
        .map(|x| {
            println!("{} ->> {}", x, x + taxa_transport);
            x + taxa_transport
        })
        .collect();
}

// ============================================================
// EXERCITIUL 2 — Lant de iteratori: filter + map + sum
// ============================================================
// Ai urmatoarea lista de note: [3, 7, 5, 9, 2, 8, 4, 6, 10, 1]
//
// Folosind UN SINGUR lant de iteratori (fara `for`):
// 1. Pastreaza doar notele de trecere (>= 5)
// 2. Ridica fiecare la patrat
// 3. Calculeaza suma
//
// Afiseaza rezultatul.
// Hint: .filter() -> .map() -> .sum()
// ============================================================

fn ex2() {
    let note = vec![3, 7, 5, 9, 2, 8, 4, 6, 10, 1];

    // scrie lantul de iteratori
}

// ============================================================
// EXERCITIUL 3 — zip: asociaza doua liste
// ============================================================
// Ai doi vectori:
//   nume:   ["Ion", "Ana", "Petre", "Maria"]
//   scoruri: [85, 42, 91, 67]
//
// Folosind .zip() si .filter(), colecteaza intr-un Vec
// doar perechile (nume, scor) unde scorul este >= 70.
//
// Afiseaza fiecare pereche filtrata.
// Output asteptat:
//   Ion: 85
//   Petre: 91
// ============================================================

fn ex3() {
    let nume    = vec!["Ion", "Ana", "Petre", "Maria"];
    let scoruri = vec![85, 42, 91, 67];

    // scrie zip + filter + collect, apoi afiseaza
}

// ============================================================
// EXERCITIUL 4 — enumerate + take: top N
// ============================================================
// Ai un Vec cu numele jucatorilor, deja sortati dupa scor
// (primul este cel mai bun):
//   ["Albastros", "Kepolis", "Hoham", "Murish", "Gavlos"]
//
// Afiseaza primii 3 cu locul lor (incepand de la 1):
//   Locul 1: Albastros
//   Locul 2: Kepolis
//   Locul 3: Hoham
//
// Hint: .enumerate() adauga indexul 0, 1, 2... — aduna 1 la afisare.
// ============================================================

fn ex4() {
    let jucatori = vec!["Albastros", "Kepolis", "Hoham", "Murish", "Gavlos"];

    // scrie .iter().enumerate().take(3) si afiseaza
}

// ============================================================
// EXERCITIUL 5 — Combinat: closure + iteratori pe range
// ============================================================
// Scrie un closure `este_prim` care primeste un numar u32
// si returneaza true daca este numar prim, false altfel.
// (Un numar prim e divizibil doar cu 1 si cu el insusi.)
//
// Hint pentru logica: un numar n este prim daca niciun numar
// din 2..n nu il divide. Poti folosi .all() pe un range.
//
// Folosind closure-ul, afiseaza toate numerele prime din 2..=50
// si numarul lor total.
// ============================================================

fn ex5() {
    // defineste closure-ul este_prim, apoi aplica-l pe (2..=50)
}

// ============================================================
// EXERCITIUL 6 — Closure mutabil + any / all + chain
// ============================================================
// Ai doua liste de scoruri din doua runde ale aceluiasi turneu:
//   runda1: [45, 80, 30, 90, 55]
//   runda2: [60, 70, 85, 20, 95]
//
// Partea A — Closure mutabil:
//   Scrie un closure `adauga_la_total` care captureaza o variabila
//   `total: i32 = 0` si, la fiecare apel cu un scor, il adauga la total.
//   Aplica-l pe toate scorurile din runda1 si afiseaza totalul final.
//
// Partea B — chain + any / all:
//   Combina cele doua liste cu .chain() intr-un singur iterator.
//   Verifica si afiseaza:
//     - Exista vreun scor mai mic de 25? (.any())
//     - Toate scorurile sunt peste 10? (.all())
//     - Care este scorul maxim din ambele runde? (.max())
// ============================================================

fn ex6() {
    let runda1 = vec![45, 80, 30, 90, 55];
    let runda2 = vec![60, 70, 85, 20, 95];

    // Partea A: closure mutabil
    let mut total = 0;
    // scrie closure-ul si aplica-l pe runda1

    // Partea B: chain + any / all / max
    // combina runda1 si runda2 cu .chain(), apoi aplica verificarile
}

fn main() {
    println!("=== Exercitii 08: Closures si Iteratori ===\n");

    println!("--- Exercitiul 1: Closure cu capturare ---");
    ex1();

    println!("\n--- Exercitiul 2: filter + map + sum ---");
    ex2();

    println!("\n--- Exercitiul 3: zip + filter ---");
    ex3();

    println!("\n--- Exercitiul 4: enumerate + take ---");
    ex4();

    println!("\n--- Exercitiul 5: Closure + range ---");
    ex5();

    println!("\n--- Exercitiul 6: Closure mutabil + chain + any/all ---");
    ex6();
}