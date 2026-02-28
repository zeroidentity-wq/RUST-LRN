// Exercise 02 — Functions, parameters, return values
// Rulare: cargo run --bin ex_02_functions

fn main() {
    println!("{}", patrat(3));
    let nume = String::from("Tony");
    saluta(&nume);
    let max = max_doua(2, 6);
    println!("Maxim: {}", max);

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

// 1. Abilitatea "Critical Strike" — returneaza damage-ul la patrat.
//    Exemplu: patrat(3) -> 9
//    Conditie: foloseste return implicit (fara `return` si fara `;` pe ultima linie).
fn patrat(n: i32) -> i32 {
    n * n
}

// 2. Anunta eroul care a intrat in arena.
//    Exemplu: saluta(&String::from("Arthur")) -> afiseaza "Salutare, Arthur!"
//    Conditie: nu returneaza nimic, primeste imprumut imutabil.
fn saluta(nume: &String) {
    println!("Salutare,{}!", nume);
}

// 3. Returneaza atacantul mai puternic dintre doi eroi.
//    Exemplu: max_doua(2, 6) -> 6
//    Conditie: foloseste if/else ca expresie — if/else returneaza o valoare in Rust.
fn max_doua(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// ---------------------------------------------------------------------------
// Exercitii suplimentare
// ---------------------------------------------------------------------------

// 4. Calculeaza damage-ul total din trei atacuri simultane.
//    Exemplu: suma_trei(4, 5, 6) -> 15
//    Conditie: foloseste return implicit.
fn suma_trei(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}

// 5. Verifica daca eroul mai are HP pozitiv (e inca in viata).
//    Exemplu: este_pozitiv(7) -> true, este_pozitiv(-2) -> false
//    Conditie: foloseste if/else ca expresie — fara `return` explicit.
fn este_pozitiv(n: i32) -> bool {
    if n > 0 { true } else { false }
}

// 6. Clasifica inamicul dupa nivel de putere.
//    Returneaza:
//      - "negativ"  daca n < 0  (ghost — nivel invalid)
//      - "zero"     daca n == 0 (mort)
//      - "mic"      daca n este intre 1 si 9 inclusiv (mob slab)
//      - "mare"     daca n >= 10 (boss)
//    Conditie: foloseste `return` timpuriu pentru primele doua cazuri.
fn clasa_numar(n: i32) -> &'static str {
    if n < 0 {
        return "negativ";
    } else if n == 0 {
        return "zero";
    } else if n >= 1 && n <= 9 {
        return "mic";
    } else if n >= 10 {
        return "mare";
    } else {
        return "gresit"
    }
}

// 7. Returneaza lungimea numelui eroului FARA a prelua ownership-ul.
//    Exemplu: lungime_imprumut(&String::from("Rust")) -> 4
//    Conditie: primeste imprumut imutabil, proprietarul ramane valid dupa apel.
fn lungime_imprumut(s: &String) -> usize {
    let lungime: usize = s.len();
    lungime
}

// 8. Blacksmith-ul preia ownership-ul numelui item-ului, adauga prefixul ">> "
//    si returneaza ownership-ul item-ului modificat.
//    Exemplu: adauga_prefix(String::from("Excalibur")) -> ">> Excalibur"
//    Indiciu: format!(">> {}", s) construieste un String nou.
fn adauga_prefix(s: String) -> String {
    let prefixat: String = format!(">> {}", s);
    prefixat
}

// ---------------------------------------------------------------------------
// Exercitii suplimentare — Functii (nivel mai ridicat)
// ---------------------------------------------------------------------------

// 9. Calculeaza XP-ul total acumulat de la nivelul 1 pana la nivelul n.
//    Exemplu: suma_pana_la(5) -> 15  (1+2+3+4+5)
//    Conditie: foloseste un `for` cu range si o variabila acumulatoare.
fn suma_pana_la(n: u32) -> u32 {
    let mut suma = 0;
    for i in 1..=n {
        suma = suma + i;
    }
    suma
}

// 10. Calculeaza damage-ul absolut (fara semn negativ — nu exista damage negativ).
//     Exemplu: abs_valoare(-7) -> 7, abs_valoare(3) -> 3
//     Conditie: foloseste if/else ca expresie, fara `return` explicit.
fn abs_valoare(n: i32) -> i32 {
    if n < 0 { -n } else if n == 0 { 0 } else { n }
}

// 11. Calculeaza combo multiplier-ul: n! (n factorial).
//     Exemplu: factorial(5) -> 120  (5*4*3*2*1), factorial(0) -> 1
//     Conditie: foloseste return timpuriu pentru cazul n == 0.
fn factorial(n: u64) -> u64 {
    if n == 0 { return 1; }
    let mut factorial = 1;
    for i in 1..=n {
        factorial *= i;
    }
    factorial
}

// ---------------------------------------------------------------------------
// Quiz — Ownership & Borrowing (capitol anterior)
// ---------------------------------------------------------------------------
// Pentru fiecare bloc: citeste codul, raspunde la intrebare IN COMENTARIU,
// apoi decommenteaza codul ca sa verifici daca ai dreptate.

// QUIZ 1: Compileaza sau nu? De ce?
//         (Hint: jucatorul 2 preia sabia de la jucatorul 1)
//
// NU compileaza, s2 detine valoarea lui s1, iar in timpul apelului lui println! s1 nu mai detine valoarea
// fn quiz1() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("{}", s1);
// }

// QUIZ 2: Compileaza sau nu? De ce?
//         (Hint: doi blacksmith-i incearca sa upgradeze acelasi item simultan)
//
// NU compileaza, nu putem avea simultan mai mult de o referinta mutabila catre aceasi zona din memorie.
// fn quiz2() {
//     let mut s = String::from("hello");
//     let r1 = &mut s;
//     let r2 = &mut s;
//     println!("{} {}", r1, r2);
// }

// QUIZ 3: Codul de mai jos compileaza. Ce va afisa si de ce?
//         (Hint: gold-ul se copiaza, nu se muta)
//
// Compileaza pentru ca i32 implementeaza trait COPY, asa ca valoarea este copiata in y, nu este mutata.
// fn quiz3() {
//     let x = 5;
//     let y = x;
//     println!("x={}, y={}", x, y);
//     // Hint: x este i32. Ce stii despre tipurile simple si ownership?
// }