// Exercise 01 — Ownership & Borrowing
// Rulare: cargo run --bin ex_01_ownership

fn main() {
    let mut s1 = String::from("Rust!"); // devine Rust!Rust!
    let s2 = &s1;
    println!("{}, {}", s1, s2);
    println!("{}", prima_litera(&s1));
    adauga_exclamare(&mut s1);
    println!("{}", s1);
    main_ex4();
    dublura(&mut s1);
    let s2 = String::from("Luung");
    let exercitiu6 = prima_si_ultima(&s2);
    println!("{:?}", exercitiu6);

    let mut s3 = String::from("DeInversat");
    inverseaza(&mut s3);
    println!("s3 inversat: {}", s3);

    let s4 = String::from("De procesat: ");
    let s4_nou = proceseaza(s4);
    println!("s4 procesat: {}", s4_nou);

    main_ex7();
}

// 2. Scrie `prima_litera(s: &String) -> char`
//    Returneaza prima litera din numele eroului, fara sa muti ownership-ul.
//    Exemplu: "Arthur" -> 'A'
fn prima_litera(s: &String) -> char {
    s.chars().nth(0).unwrap()
}

// 3. Scrie `adauga_exclamare(s: &mut String)`
//    Blacksmith-ul adauga "!" la sfarsitul numelui eroului (imprumut mutabil).
//    Exemplu: "Rust" -> "Rust!"
fn adauga_exclamare(s: &mut String) {
    s.push_str("!");
}

// 4. Codul de mai jos nu compileaza — nu poti avea doua &mut simultan,
//    nici &mut si & in acelasi timp.
//    Repara-l astfel incat sa afiseze: "hello, hello"
fn main_ex4() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    r1.push_str(", hello");
    // let r2 = &s;  ← scrie &s (imutabil)
    // Eroarea originala era let r2 = &mut s (al doilea &mut), nu &s. Sunt doua reguli diferite:
    // - Nu poti avea doua &mut simultan
    // - Nu poti avea &mut si & simultan
    // pasam lui r1 o referinta mutabila catre s, si facem un push_str cu textul lipsa.
    println!("{}", s);
}

// 5. Scrie `dublura(s: &mut String)`
//    Dubleaza continutul inventarului eroului.
//    Exemplu: "Potion" -> "PotionPotion"
//    Indiciu: salveaza o copie inainte sa modifici cu s.clone()
fn dublura(s: &mut String) {
    let original = s.clone();
    s.push_str(&original);
}

// 6. Scrie `prima_si_ultima(s: &String) -> (char, char)`
//    Returneaza prima si ultima litera din numele eroului, fara sa muti ownership-ul.
//    Exemplu: "Rust" -> ('R', 't')
//    Indiciu: .chars().last() pentru ultimul caracter
fn prima_si_ultima(s: &String) -> (char, char) {
    let ultima = s.chars().last().unwrap();
    let prima = s.chars().next().unwrap();
    (prima, ultima)
}

// 7. Codul de mai jos nu compileaza — referinta dangling (suspendata).
//    `s` este sters cand iese din scope, iar `referinta` ar pointa la memorie invalida.
//    Repara-l mutand `s` in acelasi scope cu `referinta`.
fn main_ex7() {
    let referinta;
    let s = String::from("hello");
    referinta = &s;

    println!("{}", referinta);
}

// s iese din scope si face drop, si nu poate fi folosit mai departe de catre referinta
// ma gandesc sa scot scope si sa fie vizibil in main

// 8. Scrie `inverseaza(s: &mut String)`
//    Inverseaza numele eroului in loc (blacksmith-ul il rescrie).
//    Exemplu: "Arthur" -> "ruhtrA"
//    Indiciu: s.chars().rev().collect::<String>() iti da String-ul inversat
//    *s inseamna "scrie la adresa la care pointeaza s"
fn inverseaza(s: &mut String) {
    *s = s.chars().rev().collect::<String>();
}

// 9. Scrie `proceseaza(s: String) -> String`
//    Quest log-ul preia ownership-ul unui mesaj, adauga " [procesat]" si returneaza ownership-ul.
//    Exemplu: "Misiunea" -> "Misiunea [procesat]"
//    Dupa apel, variabila originala din main() mai poate fi folosita?
fn proceseaza(s: String) -> String {
    let mut de_procesat = s;
    de_procesat.push_str(" [procesat]");
    de_procesat
}