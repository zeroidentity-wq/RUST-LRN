// ============================================================
// Ownership & Borrowing — cod demonstrativ
// Citeste notitele in: notes/01_ownership_borrowing.md
// ============================================================

fn main() {
    demo_move();
    demo_copy();
    demo_borrowing();
    demo_mutable_borrow();
}

// --- MOVE ---
// Proprietatea se muta cand atribui sau trimiti la o functie
fn demo_move() {
    println!("=== MOVE ===");

    let s1 = String::from("hello");
    let s2 = s1; // s2 acum detine OWNER
    // println!("s1={}",s1); EROARE: s1 nu mai este proprietar
    println!("s={}",s2); // s2 detine hello
}

// --- COPY ---
// Tipurile simple (i32, bool, f64, char) se copiaza automat
fn demo_copy() {
    println!("\n=== COPY ===");

    let x = 55;
    let y = x; // Tipurile simple implementeaza COPY
    println!("x: {}, y: {}", x,y);
}

// --- BORROWING IMUTABIL ---
// Imprumutam cu &, proprietarul ramane valid
fn demo_borrowing() {
    println!("\n=== BORROWING (&) ===");
    // calculeaza lungimea unui string imprumutat
    let s = String::from("Rust");
    let nr_caractere = calculeaz_lungime(&s);
    println!("nr de caractere: {}", nr_caractere);
}

fn calculeaz_lungime(st: &String) -> i32 {
    let lungime:i32 = st.len() as i32;
    lungime
}

// --- BORROWING MUTABIL ---
// Imprumutam cu &mut ca sa modificam
fn demo_mutable_borrow() {
    println!("\n=== BORROWING MUTABIL (&mut) ===");

    let mut s:String = String::from("Adauga aici ");
    println!("s= {}", s);
    adauga_text(&mut s);
    println!("s= {}", s);
}
fn adauga_text(text: &mut String ){
    text.push_str(" am adaugat!");
}

