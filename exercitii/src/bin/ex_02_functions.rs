// Exercise 02 — Functions, parameters, return values
// Rulare: cargo run --bin ex_02_functions

fn main() {
    println!("{}",patrat(3));
    let nume = String::from("Tony");
    saluta(&nume);
    let max = max_doua(2,6);
    println!("Maxim: {}",max);

    // Ex 4 — asteptat: 15
    println!("Suma: {}", suma_trei(4, 5, 6));

    // Ex 5 — asteptat: true, false
    println!("{}", este_pozitiv(7));
    println!("{}", este_pozitiv(-2));

    // Ex 6 — asteptat: negativ, zero, mic, mare
    println!("{}", clasa_numar(-5));
    println!("{}", clasa_numar(0));
    println!("{}", clasa_numar(3));
    println!("{}", clasa_numar(42));

    // Ex 7 — asteptat: 4 (s ramane valid dupa apel)
    let s = String::from("Rust");
    println!("Lungime: {}", lungime_imprumut(&s));
    println!("Inca valid: {}", s);

    // Ex 8 — asteptat: >> salut (s2 e consumat in functie)
    let s2 = String::from("salut");
    let s2_nou = adauga_prefix(s2);
    println!("{}", s2_nou);

    // Ex 9 — asteptat: 15
    println!("Suma: {}", suma_pana_la(5));

    // Ex 10 — asteptat: 7, 3
    println!("{}", abs_valoare(-7));
    println!("{}", abs_valoare(3));

    // Ex 11 — asteptat: 120, 1
    println!("{}", factorial(5));
    println!("{}", factorial(0));
}

// 1. Scrie o functie `patrat(n: i32) -> i32` care returneaza n * n.
//    Foloseste return implicit (fara `return` si fara `;` pe ultima linie).
fn patrat(n:i32) -> i32 {
    n * n
}

// 2. Scrie o functie `saluta(nume: &String)` care printeaza "Salut, <nume>!".
//    Nu returneaza nimic.

fn saluta(nume: &String) {
    println!("Salutare,{}!", nume);
}

// 3. Scrie o functie `max_doua(a: i32, b: i32) -> i32` care returneaza
//    cel mai mare dintre cele doua numere.
//    Indiciu: poti folosi if/else ca expresie — if/else returneaza o valoare in Rust.
fn max_doua(a:i32, b:i32) -> i32 {
    if a > b {a} else {b}
}

// ---------------------------------------------------------------------------
// Exercitii suplimentare
// ---------------------------------------------------------------------------

// 4. Scrie o functie `suma_trei(a: i32, b: i32, c: i32) -> i32`
//    care returneaza suma celor trei numere.
//    Foloseste return implicit.
fn suma_trei(a: i32, b: i32, c: i32) -> i32 {
    todo!()
}

// 5. Scrie o functie `este_pozitiv(n: i32) -> bool`
//    care returneaza true daca n > 0, false altfel.
//    Foloseste if/else ca expresie — fara `return` explicit.
fn este_pozitiv(n: i32) -> bool {
    todo!()
}

// 6. Scrie o functie `clasa_numar(n: i32) -> &'static str`
//    care returneaza:
//      - "negativ"  daca n < 0
//      - "zero"     daca n == 0
//      - "mic"      daca n este intre 1 si 9 (inclusiv)
//      - "mare"     daca n >= 10
//    Foloseste `return` timpuriu pentru primele doua cazuri.
fn clasa_numar(n: i32) -> &'static str {
    todo!()
}

// 7. Scrie o functie `lungime_imprumut(s: &String) -> usize`
//    care returneaza lungimea string-ului FARA a prelua ownership.
//    Indiciu: metoda .len() returneaza numarul de bytes dintr-un String.
fn lungime_imprumut(s: &String) -> usize {
    todo!()
}

// 8. Scrie o functie `adauga_prefix(s: String) -> String`
//    care primeste ownership-ul unui String, adauga ">> " la inceput
//    si returneaza String-ul modificat.
//    Indiciu: format!(">> {}", s) construieste un String nou.
fn adauga_prefix(s: String) -> String {
    todo!()
}

// ---------------------------------------------------------------------------
// Exercitii suplimentare — Functii (nivel mai ridicat)
// ---------------------------------------------------------------------------

// 9. Scrie o functie `suma_pana_la(n: u32) -> u32`
//    care returneaza suma 1 + 2 + 3 + ... + n.
//    Exemplu: suma_pana_la(5) => 15
//    Foloseste un `for` si o variabila acumulatoare.
fn suma_pana_la(n: u32) -> u32 {
    todo!()
}

// 10. Scrie o functie `abs_valoare(n: i32) -> i32`
//     care returneaza valoarea absoluta a lui n.
//     Exemplu: abs_valoare(-7) => 7, abs_valoare(3) => 3
//     Conditie: foloseste if/else ca expresie, fara `return` explicit.
fn abs_valoare(n: i32) -> i32 {
    todo!()
}

// 11. Scrie o functie `factorial(n: u64) -> u64`
//     care calculeaza n! (n factorial).
//     Exemplu: factorial(5) => 120  (5 * 4 * 3 * 2 * 1)
//     Conditie: foloseste return timpuriu pentru cazul n == 0.
fn factorial(n: u64) -> u64 {
    todo!()
}

// ---------------------------------------------------------------------------
// Quiz — Ownership & Borrowing (capitol anterior)
// ---------------------------------------------------------------------------
// Pentru fiecare bloc: citeste codul, raspunde la intrebare IN COMENTARIU,
// apoi decommenteaza codul ca sa verifici daca ai dreptate.

// QUIZ 1: Compileaza sau nu? De ce?
//         Scrie raspunsul tau ca un comentariu deasupra blocului.
//
// fn quiz1() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("{}", s1);
// }

// QUIZ 2: Compileaza sau nu? De ce?
//         Scrie raspunsul tau ca un comentariu deasupra blocului.
//
// fn quiz2() {
//     let mut s = String::from("hello");
//     let r1 = &mut s;
//     let r2 = &mut s;
//     println!("{} {}", r1, r2);
// }

// QUIZ 3: Codul de mai jos compileaza, dar afiseaza ceva neasteptat.
//         Ce va afisa si de ce? Scrie raspunsul in comentariu.
//
// fn quiz3() {
//     let x = 5;
//     let y = x;
//     println!("x={}, y={}", x, y);
//     // Hint: x este i32. Ce stii despre tipurile simple si ownership?
// }
