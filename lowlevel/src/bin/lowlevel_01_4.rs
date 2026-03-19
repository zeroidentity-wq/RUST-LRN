fn cripteaza(text: &str, n: u8) -> String {
    let mut criptat = String::new();
    for caracter in text.chars() {
        let i_original = caracter as u8 - b'a';
        let index_nou = (i_original + n) % 26;
        let litera = (b'a' + index_nou) as char;
        criptat.push(litera);
    }
    println!("Criptat: {}", criptat);
    criptat
}

fn decripteaza(text: &str, n: u8) -> String {
    let mut decriptat = String::new();
    for caracter in text.chars() {
        let i_curent = caracter as u8 - b'a';
        let index_vechi = (i_curent + 26 - n) % 26;
        let litera = (b'a' + index_vechi) as char;
        decriptat.push(litera);
    }
    println!("Decriptat: {}", decriptat);
    decriptat
}

fn main() {
    cripteaza("zab", 3);
    decripteaza("abc", 3);
}
