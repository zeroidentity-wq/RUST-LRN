// Scrie o funcție `este_palindrom(s: &str) -> bool`.
// "aba" → true, "abba" → true, "abc" → false.
// Bonus: ignoră spațiile și majusculele: "Ana are era na" → true (sorta... vezi mai jos).

fn este_palindrom(s: &str) -> bool {
    let trim = s.trim().to_lowercase(); // trimmed si lowercase
    let trim_v2 = trim.replace(" ", "");
    let vec_chars: Vec<char> = trim_v2.chars().collect();
    println!("Trimed: {:?}", vec_chars);
    if vec_chars.is_empty() {
        return true;
    }
    let mut stanga: usize = 0;
    let mut dreapta = vec_chars.len() - 1;

    while stanga < dreapta {
        if vec_chars[stanga] != vec_chars[dreapta] {
            return false;
        }
        stanga += 1;
        dreapta -= 1;
    }

    true
}

fn main() {
    let input = "Ana are era na";
    match este_palindrom(input) {
        true => {
            println!("Este palindrom!");
        }
        false => {
            println!("NU este palindrom!");
        }
    }
}
