// Exercise 01 — Ownership & Borrowing
// Write your solution here

fn main() {
    let mut s1 = String::from("Rust!"); // devine Rust!Rust!
    let s2 = &s1;
    println!("{}, {}", s1,s2);
    println!("{}", prima_litera(&s1));
    adauga_exclamare(&mut s1);
    println!("{}", s1); // aici vedem rezultatul la dubla()
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

//  2. Scrie o funcție prima_litera(s: &String) -> char care returnează primul caracter dintr-un String, fara sa mute proprietatea.
fn prima_litera(s: &String) -> char {
    s.chars().nth(0).unwrap()
}

// 3. Scrie o funcție adauga_exclamare(s: &mut String) care adauga "!" la sfârșitul unui String.
fn adauga_exclamare(s: &mut String) {
    s.push_str("!");
}


// 4. Codul de mai jos nu compilează. Găsește TOATE erorile, explica-le
//    in comentarii si repara codul ca sa afișeze: "hello hello"
//
fn main_ex4() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    r1.push_str(", hello");
    // let r2 = &s;  ← scrie &s (imutabil)
    // Eroarea originala era let r2 = &mut s (al doilea &mut), nu &s. Sunt doua reguli diferite:
    // - Nu poti avea doua &mut simultan
    //    - Nu poti avea &mut si & simultan
    // pasam lui r1 o referința mutabila către s, si facem un pus_str cu textul lipsa.
    println!("{}", s);
}

// 5. Scrie o funcție dubla(s: &mut String) care dublează conținutul
//    string-ului. Exemplu: "Rust" devine "RustRust".
//    Indiciu: poti citi valoarea curenta cu s.clone() înainte sa modifici.
fn dublura(s: &mut String) {
    let original = s.clone();
    s.push_str(&original);
}

// 6. Scrie o funcție prima_si_ultima(s: &String) -> (char, char) care
//    returnează primul si ultimul caracter dintr-un String, fara sa mute
//    proprietatea. Exemplu: "Rust" -> ('R', 't')
//    Indiciu: pentru ultimul caracter poti folosi s.chars().last()
fn prima_si_ultima(s: &String) -> (char, char) {
    let ultima = s.chars().last().unwrap();
    let prima = s.chars().next().unwrap();
    (prima, ultima)
}

// 7. Codul de mai jos nu compileaza. Explica in comentarii DE CE apare
//    eroarea, apoi repara-l.
//
fn main_ex7() {
    let referinta;
    let s = String::from("hello");
    referinta = &s;

    println!("{}", referinta);
}

// s iese din scope si face drop, si nu poate fi folosit mai departe de catre referinta
// ma gandesc sa scot scope si sa fie vizibil in main

// 8. Scrie o functie inverseaza(s: &mut String) care inverseaza sirul
//    in loc. Exemplu: "Rust" devine "tsuR".
//    Indiciu: s.chars().rev().collect::<String>() iti da un String inversat.
//    *s inseamna "scrie la adresa la care pointeaza s".
fn inverseaza(s: &mut String) {
    *s=s.chars().rev().collect::<String>();
}

// 9. Scrie o functie proceseaza(s: String) -> String care:
//    - PREIA proprietatea unui String (nu imprumut!)
//    - adauga " [procesat]" la sfarsit
//    - returneaza proprietatea inapoi
//    Testeaza: s = "date" => "date [procesat]"
//    Dupa apel, variabila originala din main() mai poate fi folosita?
fn proceseaza(s: String) -> String {
    // fac o var mutabila care sa preia pe s si întorc rezultatul
    let mut de_procesat = s;
    de_procesat.push_str(" [procesat]");
    de_procesat

}