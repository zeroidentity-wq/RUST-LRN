fn cripteaza(text: &str, n:u8) -> String {
    let mut criptat = String::new();
    for caracter in text.chars(){
        let i_original = caracter as u8;
        let mask = (i_original + n) as char;
        criptat.push(mask);
    }
    println!("{}", criptat);
    criptat
}

fn main() {
    cripteaza("abc",3);
}