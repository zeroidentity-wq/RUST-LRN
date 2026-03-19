// Scrie o funcție care primește un string și returnează frecvența fiecărei litere mici (a-z).
// "banana" → a:3, b:1, n:2
fn main() {
    calculeaza_string(String::from("avadacadabra"));
}

fn calculeaza_string(s: String) {
    // array cu 26 0
    // fiecare caracter -> c as u32 - 'a' as u32'
    let s_mic = s.to_lowercase();
    let mut frecventa: [u32; 26] = [0; 26]; // 26 de 0
    for caracter in s_mic.chars() {
        if !caracter.is_ascii_lowercase() {
            continue;
        }
        let index = caracter as u32 - 'a' as u32;
        //merg la index si +1 unde? la frecventa
        frecventa[index as usize] += 1;
    }
    // println!("{:?}", frecventa);
    for i in 0..26 {
        if frecventa[i] > 0 {
            let litera = (b'a' + i as u8) as char; // 'a' as u8 + index as u8
            println!("{}: {}", litera, frecventa[i]);
        }
    }
}
