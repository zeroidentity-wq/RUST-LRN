// Lectia 08 — Closures si Iteratori
// Rulare: cargo run --bin 08_closures

// ============================================================
// CLOSURES
// ============================================================
// Un closure este o functie anonima (fara fn si fara nume)
// pe care o poti salva intr-o variabila sau trimite ca argument.
//
// Superputere vs fn clasica: closure-ul poate CAPTURA variabile
// din contextul in care a fost creat.
// ============================================================

fn demo_closures() {
    println!("========== CLOSURES ==========\n");

    // --- 1. Sintaxa: closure vs functie normala ---
    println!("--- 1. Sintaxa ---");

    fn aduna_cinci_fn(x: i32) -> i32 { x + 5 }   // functie clasica
    let aduna_cinci_cl = |x| x + 5;               // closure echivalent

    println!("  fn:      {}", aduna_cinci_fn(10)); // 15
    println!("  closure: {}", aduna_cinci_cl(10)); // 15
    // Apelul e identic. Closure-ul e mai scurt si nu necesita tipuri explicite.

    // --- 2. Capturarea mediului ---
    println!("\n--- 2. Capturarea mediului ---");

    let taxa_livrare = 15;

    // Closure-ul "captureaza" taxa_livrare prin imprumut.
    // O functie fn nu poate face asta — ar da eroare de compilare.
    let calculeaza_total = |pret| pret + taxa_livrare;

    println!("  Total pt 50:  {}", calculeaza_total(50));  // 65
    println!("  Total pt 100: {}", calculeaza_total(100)); // 115

    // --- 3. Closure mutabil ---
    println!("\n--- 3. Closure mutabil ---");

    let mut contor = 0;

    // mut pe closure deoarece modifica variabila `contor` din exterior
    let mut incrementeaza = || {
        contor += 1;
        println!("  Contor: {}", contor);
    };

    incrementeaza(); // Contor: 1
    incrementeaza(); // Contor: 2
    incrementeaza(); // Contor: 3

    // --- 4. Closure ca argument ---
    println!("\n--- 4. Closure ca argument ---");

    let v = vec![1, 2, 3, 4, 5];

    // Closure-ul |n| n * 2 este trimis direct ca argument lui .map()
    // Nu trebuie sa definim o functie separata
    let duble: Vec<i32> = v.iter().map(|n| n * 2).collect();
    println!("  Duble: {:?}", duble);

    // Closure cu bloc mai lung (cand ai mai mult de o expresie)
    let descrie: Vec<String> = v.iter().map(|&n| {
        if n % 2 == 0 {
            format!("{} este par", n)
        } else {
            format!("{} este impar", n)
        }
    }).collect();

    for s in &descrie {
        println!("  {}", s);
    }
}

// ============================================================
// ITERATORI
// ============================================================
// Un iterator produce valori una cate una, la cerere.
// Closures si iteratori merg mana in mana:
//   - metodele de iterare primesc closure-uri
//   - closure-ul descrie CE sa faci cu fiecare element
//
// EVALUARE LENESA: adaptoarele (.map, .filter) nu fac nimic
// singure. Banda porneste abia la consumator (.collect, .sum).
// ============================================================

fn demo_iter_intrare() {
    println!("\n========== INTRARE IN ITERATOR ==========\n");

    let v = vec![10, 20, 30];

    // .iter() — imprumuta elementele ca &T, v ramane valid
    let suma_ref: i32 = v.iter().map(|x| x).sum();
    println!("  .iter() suma: {} | v inca exista: {:?}", suma_ref, v);

    // .iter_mut() — imprumuta mutabil (&mut T), modifica in-place
    let mut v2 = vec![1, 2, 3];
    v2.iter_mut().for_each(|x| *x *= 100);
    println!("  .iter_mut() dupa x100: {:?}", v2);

    // .into_iter() — consuma Vec-ul (v3 NU mai exista dupa)
    let v3 = vec![1, 2, 3];
    let v4: Vec<i32> = v3.into_iter().map(|x| x + 1).collect();
    println!("  .into_iter() rezultat nou: {:?}", v4);
    // println!("{:?}", v3); // EROARE: v3 a fost consumat (mutat in v4)
}

fn demo_adaptoare() {
    println!("\n========== ADAPTOARE (lazy) ==========\n");

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];

    // .map() — transforma fiecare element
    let patrate: Vec<i32> = v.iter().map(|x| x * x).collect();
    println!("  .map() patrate:    {:?}", patrate);

    // .filter() — pastreaza elementele care trec conditia
    // Nota: &&x in pattern deoarece .iter() da &i32, filter primeste &&i32
    let pare: Vec<&i32> = v.iter().filter(|&&x| x % 2 == 0).collect();
    println!("  .filter() pare:    {:?}", pare);

    // .take(n) si .skip(n)
    let primele_3: Vec<&i32> = v.iter().take(3).collect();
    let fara_5:    Vec<&i32> = v.iter().skip(5).collect();
    println!("  .take(3):          {:?}", primele_3);
    println!("  .skip(5):          {:?}", fara_5);

    // .enumerate() — adauga index automat
    println!("  .enumerate():");
    for (i, val) in v.iter().enumerate().take(4) {
        println!("    [{}] = {}", i, val);
    }

    // .zip() — combina doi iteratori in perechi
    let litere = vec!['a', 'b', 'c'];
    let numere  = vec![1, 2, 3];
    let perechi: Vec<(&char, &i32)> = litere.iter().zip(numere.iter()).collect();
    println!("  .zip():            {:?}", perechi);

    // .chain() — concateneaza doi iteratori
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let tot: Vec<&i32> = a.iter().chain(b.iter()).collect();
    println!("  .chain():          {:?}", tot);
}

fn demo_consumatori() {
    println!("\n========== CONSUMATORI (eager) ==========\n");

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // .count() — numara elementele
    let nr_pare = v.iter().filter(|&&x| x % 2 == 0).count();
    println!("  .count() pare:          {}", nr_pare);

    // .sum() — aduna valorile
    let suma: i32 = v.iter().sum();
    println!("  .sum():                 {}", suma);

    // .any() — true daca CEL PUTIN un element trece conditia
    let are_mare = v.iter().any(|&x| x > 8);
    println!("  .any(x > 8):            {}", are_mare);

    // .all() — true daca TOATE elementele trec conditia
    let toate_poz = v.iter().all(|&x| x > 0);
    println!("  .all(x > 0):            {}", toate_poz);

    // .find() — primul element care satisface conditia → Option<&T>
    let primul_par = v.iter().find(|&&x| x % 2 == 0);
    println!("  .find(par):             {:?}", primul_par);

    // .max() si .min()
    println!("  .max():                 {:?}", v.iter().max());
    println!("  .min():                 {:?}", v.iter().min());

    // .for_each() — executa o actiune fara a returna ceva util
    print!("  .for_each() primele 4:  ");
    v.iter().take(4).for_each(|x| print!("{} ", x));
    println!();
}

fn demo_inlantuire() {
    println!("\n========== INLANTUIRE ==========\n");

    let note = vec![3, 7, 5, 9, 2, 8, 4, 6, 10, 1];

    // Vrem suma notelor de trecere (>= 5), luand doar primele 4
    let rezultat: i32 = note.iter()
        .filter(|&&n| n >= 5)  // pastreaza notele de trecere
        .take(4)               // primele 4 dintre ele
        .map(|&n| n * n)       // ridica la patrat
        .sum();                // aduna

    println!("  Note: {:?}", note);
    println!("  Suma patratelor primelor 4 note de trecere (>=5): {}", rezultat);

    // Acelasi lucru cu un for clasic — compara verbozitatea
    let mut suma = 0;
    let mut gasite = 0;
    for &n in &note {
        if n >= 5 {
            suma += n * n;
            gasite += 1;
            if gasite == 4 { break; }
        }
    }
    println!("  Acelasi rezultat cu for clasic: {}", suma);
}

fn demo_range() {
    println!("\n========== RANGE-URI CA ITERATORI ==========\n");

    // Range-urile sunt deja iteratori — nu ai nevoie de Vec
    let suma: i32           = (1..=100).sum();
    let patrate: Vec<i32>   = (1..=5).map(|x| x * x).collect();
    let div7                = (1..=100).filter(|x| x % 7 == 0).count();

    println!("  Suma 1..=100:              {}", suma);
    println!("  Patratele 1..=5:           {:?}", patrate);
    println!("  Divizibile cu 7 in 1..100: {}", div7);
}

fn main() {
    demo_closures();
    demo_iter_intrare();
    demo_adaptoare();
    demo_consumatori();
    demo_inlantuire();
    demo_range();
}
