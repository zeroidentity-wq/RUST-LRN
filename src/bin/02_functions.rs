// Lectia 02 — Functii, parametri si return values
// Rulare: cargo run --bin 02_functions

fn main() {
    // --- 1. Apel de baza ---
    salut();

    // --- 2. Functie cu parametru ---
    println!("{}", patrat(4)); // 16

    // --- 3. Return implicit vs explicit ---
    println!("{}", dublu_implicit(5));  // 10
    println!("{}", dublu_explicit(5));  // 10

    // --- 4. Expresie vs instructiune ---
    println!("{}", demonstreaza_expresie(3)); // 6, nu 7

    // --- 5. if/else ca expresie ---
    println!("{}", max_doua(8, 3));  // 8
    println!("{}", max_doua(2, 9));  // 9

    // --- 6. Return timpuriu ---
    println!("{}", descrie_numar(0));    // "zero"
    println!("{}", descrie_numar(5));    // "pozitiv"
    println!("{}", descrie_numar(-3));   // "negativ"

    // --- 7. Functii si ownership ---
    let s = String::from("Rust");
    afiseaza_imprumut(&s);   // s ramine valid
    println!("{}", s);        // OK

    let s2 = String::from("hello");
    let s2_nou = adauga_sufix(s2);   // s2 mutat in functie
    println!("{}", s2_nou);           // "hello!"
    // println!("{}", s2);            // EROARE — s2 a fost mutat
}

// ---------------------------------------------------------------------------
// 1. Functie simpla fara parametri si fara return
// ---------------------------------------------------------------------------
// Cand o functie nu returneaza nimic, tipul return este `()` (unit).
// In practica, il omitem complet din semnatura.

fn salut() {
    println!("Salut din functie!");
}

// ---------------------------------------------------------------------------
// 2. Parametri
// ---------------------------------------------------------------------------
// Fiecare parametru TREBUIE sa aiba tipul declarat explicit.
// Rust nu face inferenta de tip la semnatura functiei.

fn patrat(n: i32) -> i32 {
    n * n
}

// Parametri multipli — fiecare cu tipul sau
fn suma(a: i32, b: i32) -> i32 {
    a + b
}

// ---------------------------------------------------------------------------
// 3. Return implicit vs explicit
// ---------------------------------------------------------------------------
// In Rust, ultima EXPRESIE dintr-o functie este automat valoarea returnata.
// Expresie = linie FARA `;` la final.

fn dublu_implicit(n: i32) -> i32 {
    n * 2        // fara ; — expresie, valoarea e returnata
}

fn dublu_explicit(n: i32) -> i32 {
    return n * 2; // return explicit — echivalent, dar mai putin idiomaic
}

// ---------------------------------------------------------------------------
// 4. Expresie vs Instructiune — IMPORTANT
// ---------------------------------------------------------------------------
// Expresie:    produce o valoare        →  n * 2
// Instructiune: executa ceva, arunca   →  n * 2;  (cu ;)
//
// `;` transforma o expresie in instructiune — valoarea e pierduta.

fn demonstreaza_expresie(x: i32) -> i32 {
    let y = x * 2;
    y + 1;    // instructiune: calculeaza 7 (daca x=3), dar o ARUNCA
    y         // expresie: returneaza 6
}

// ---------------------------------------------------------------------------
// 5. if/else ca expresie
// ---------------------------------------------------------------------------
// In Rust, if/else poate produce o valoare — e o expresie, nu doar o structura de control.
// Conditia: ambele ramuri trebuie sa returneze ACELASI tip.

fn max_doua(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }  // if/else ca expresie — returneaza direct
}

// Acelasi lucru, mai explicit:
fn _max_doua_explicit(a: i32, b: i32) -> i32 {
    let rezultat = if a > b { a } else { b };
    rezultat
}

// ---------------------------------------------------------------------------
// 6. Return timpuriu cu `return`
// ---------------------------------------------------------------------------
// Uneori vrei sa iesi din functie inainte de final.
// `return` explicit este util in aceste cazuri.

fn descrie_numar(n: i32) -> &'static str {
    if n == 0 {
        return "zero";   // iesim imediat
    }
    if n > 0 {
        return "pozitiv";
    }
    "negativ"   // return implicit pentru cazul ramas
}

// ---------------------------------------------------------------------------
// 7. Functii si ownership — recapitulare
// ---------------------------------------------------------------------------
// Tipul parametrului determina ce se intampla cu ownership-ul.

// &String  → imprumut imutabil, ownership ramine la apelant
fn afiseaza_imprumut(s: &String) {
    println!("Valoare: {}", s);
}

// String (fara &) → ownership mutat in functie
//                   returnarea e singura cale de a-l primi inapoi
fn adauga_sufix(mut s: String) -> String {
    s.push_str("!");
    s   // returnam ownership inapoi
}
