fn main() {
    inverseaza("salut");
    inverseaza("AtaĂÎțî");

    // Caracterele normale ocupa: 1 Byte
    // Caracterele speciale ocupa: 2 Byte
    let cafe = String::from("café");
    println!("Bytes: {}", cafe.len()); // 5 bytes
    println!("Caractere: {}", cafe.chars().count()); // 4 caractere

    // Fara Vec, String pur
    inverseaza_string("Cevaa");
}

// Exercițiul 1.1: Inversează un string — manual, byte cu byte
// Scrie o funcție inverseaza(s: &str) -> String care inversează un string.
// "salut" → "tulas"
// Restricții: Nu ai voie să folosești `.reverse()`, `.rev()`, sau `.chars().rev()`
fn inverseaza(s: &str) -> String {
    let mut v_char: Vec<char> = Vec::new(); // normal
    let mut reverse: Vec<char> = Vec::new(); // inversat
    for c in s.chars() {
        v_char.push(c);
    }
    let mut i = v_char.len() - 1; // index coada
    println!("Normal: {:?}, len:{}, last:{}", v_char, i, v_char[i]);

    loop {
        reverse.push(v_char[i]);
        if i == 0 {
            break;
        }
        i -= 1;
    }

    // Construiesc String
    let mut s: String = String::new();
    for c in reverse {
        s.push(c);
    }

    println!("String inversat: {s}");
    s
}

// Poți rezolva FĂRĂ Vec, folosind doar un String gol pe care adaugi caractere?

fn inverseaza_string(s: &str) -> String {
    let mut s_string = String::new(); // string gol
    let mut i = s.chars().count() - 1;
    loop {
        s_string.push(s.chars().nth(i).unwrap());
        if i == 0 {
            break;
        }
        i -= 1;
    }

    println!("String inversat: {s_string}");
    s_string
}
