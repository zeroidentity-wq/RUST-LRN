// Lectia 02 — Functii, parametri si return values
// Rulare: cargo run --bin 02_functions

fn main() {
    // --- 1. Apel de baza ---
    anunta_meci();

    // --- 2. Functie cu parametru ---
    println!("{}", patrat_damage(4)); // 16

    // --- 3. Return implicit vs explicit ---
    println!("{}", dublu_implicit(5));  // 10
    println!("{}", dublu_explicit(5));  // 10

    // --- 4. Expresie vs instructiune ---
    println!("{}", demo_loot_pierdut(3)); // 6, nu 7 — loot-ul de 7 e aruncat

    // --- 5. if/else ca expresie ---
    println!("{}", cel_mai_puternic(8, 3));  // 8
    println!("{}", cel_mai_puternic(2, 9));  // 9

    // --- 6. Return timpuriu ---
    println!("{}", stare_erou(0));    // "mort"
    println!("{}", stare_erou(15));   // "critic"
    println!("{}", stare_erou(80));   // "in viata"

    // --- 7. Functii si ownership ---
    let sabie = String::from("Excalibur");
    inspecteaza(&sabie);         // imprumut — sabia ramane la noi
    println!("{}", sabie);       // OK

    let sabie2 = String::from("Durandal");
    let sabie2_upgraded = upgradeaza(sabie2);  // sabie2 mutata in functie
    println!("{}", sabie2_upgraded);            // "Durandal!"
    // println!("{}", sabie2);                  // EROARE — sabie2 a fost mutata
}

// ---------------------------------------------------------------------------
// 1. Functie simpla fara parametri si fara return
// ---------------------------------------------------------------------------
// Cand o functie nu returneaza nimic, tipul return este `()` (unit).
// In practica, il omitem complet din semnatura.

fn anunta_meci() {
    println!("Meciul incepe! Pregatiti-va eroi!");
}

// ---------------------------------------------------------------------------
// 2. Parametri
// ---------------------------------------------------------------------------
// Fiecare parametru TREBUIE sa aiba tipul declarat explicit.
// Rust nu face inferenta de tip la semnatura functiei.

fn patrat_damage(n: i32) -> i32 {
    n * n // damage la patrat (ex: critical hit)
}

// Parametri multipli — fiecare cu tipul sau
fn damage_net(atac: i32, armor: i32) -> i32 {
    atac - armor
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
    return n * 2; // return explicit — echivalent, dar mai putin idiomatic
}

// ---------------------------------------------------------------------------
// 4. Expresie vs Instructiune — IMPORTANT
// ---------------------------------------------------------------------------
// Expresie:     produce o valoare (da loot)     →  net + 5
// Instructiune: executa ceva, arunca rezultatul →  net + 5;  (cu ;)
//
// `;` transforma o expresie in instructiune — loot-ul e pierdut.

fn demo_loot_pierdut(x: i32) -> i32 {
    let y = x * 2;
    y + 1;   // instructiune: calculeaza 7 (daca x=3), dar ARUNCA loot-ul
    y        // expresie: returneaza 6 — acesta e loot-ul real
}

// ---------------------------------------------------------------------------
// 5. if/else ca expresie
// ---------------------------------------------------------------------------
// In Rust, if/else poate produce o valoare — e o expresie, nu doar control flow.
// Conditia: ambele ramuri trebuie sa returneze ACELASI tip.

fn cel_mai_puternic(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }  // returneaza direct atacantul mai puternic
}

// Acelasi lucru, mai explicit:
fn _cel_mai_puternic_explicit(a: i32, b: i32) -> i32 {
    let castigator = if a > b { a } else { b };
    castigator
}

// ---------------------------------------------------------------------------
// 6. Return timpuriu cu `return`
// ---------------------------------------------------------------------------
// Iesi din functie imediat daca o conditie e indeplinita.
// Util cand nu are sens sa continui (ex: eroul e deja mort).

fn stare_erou(hp: i32) -> &'static str {
    if hp <= 0 {
        return "mort";    // iesim imediat — eroul e eliminat
    }
    if hp < 20 {
        return "critic";  // HP critic — retrage-te!
    }
    "in viata"   // return implicit pentru cazul normal
}

// ---------------------------------------------------------------------------
// 7. Functii si ownership — recapitulare
// ---------------------------------------------------------------------------
// Tipul parametrului determina ce se intampla cu ownership-ul item-ului.

// &String  → imprumut imutabil, sabia ramane la proprietar
fn inspecteaza(item: &String) {
    println!("Inspectezi: {}", item);
}

// String (fara &) → ownership mutat in functie
//                   returnarea e singura cale de a-l primi inapoi
fn upgradeaza(mut item: String) -> String {
    item.push_str("!"); // item upgradat
    item                // returnam ownership inapoi
}
