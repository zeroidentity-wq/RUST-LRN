fn cripteaza(text: &str, n:u8) -> String {
    let mut criptat = String::new();
    for caracter in text.chars(){
        let i_original = caracter as u8 - b'a';
        let index_nou = (i_original  + n) % 26;
        let litera = (b'a' + index_nou) as char;
        criptat.push(litera);
    }
    println!("Criptat: {}", criptat);
    criptat
}




fn main() {
    cripteaza("xyz", 3);

}