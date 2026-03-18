// Lectia 05 — Tratarea erorilor: Option si Result
// Rulare: cargo run --bin 05_result_option

// ============================================================
// SETUP — structuri folosite in lectie
// ============================================================

#[derive(Debug)]
enum Item {
    Sabie(i32),
    Armura(i32),
    Potion(i32),
    QuestItem(String),
}

#[derive(Debug)]
struct Erou {
    nume: String,
    hp: i32,
    mana: i32,
    inventar: Vec<Item>,
}

impl Erou {
    fn new(nume: &str) -> Erou {
        Erou {
            nume: String::from(nume),
            hp: 100,
            mana: 50,
            inventar: Vec::new(),
        }
    }

    fn pick_up(&mut self, item: Item) {
        self.inventar.push(item);
    }
}

// ============================================================
// 1. Option<T> — o valoare care poate lipsi
// ============================================================
// Option<T> este un enum built-in in Rust:
//
//   enum Option<T> {
//       Some(T),   // exista o valoare de tip T
//       None,      // nu exista nimic
//   }
//
// Rust nu are null. Option este modul Rust de a spune:
// "aceasta valoare poate sa existe sau nu — si ESTI FORTAT sa verifici."

fn cauta_sabie(inventar: &Vec<Item>) -> Option<i32> {
    for item in inventar {
        // if let = sintaxa scurta pentru match cu un singur caz (explicat in sectiunea 3)
        if let Item::Sabie(dmg) = item {
            return Some(*dmg); // am gasit — returnam valoarea invelita in Some
        }
    }
    None // nu am gasit nimic
}

fn cauta_potion(inventar: &Vec<Item>) -> Option<i32> {
    for item in inventar {
        if let Item::Potion(heal) = item {
            return Some(*heal);
        }
    }
    None
}

// ============================================================
// 2. match pe Option — cel mai explicit mod de tratare
// ============================================================

fn afiseaza_sabie(inventar: &Vec<Item>) {
    match cauta_sabie(inventar) {
        Some(dmg) => println!("  Ai o sabie cu {} damage.", dmg),
        None => println!("  Nu ai nicio sabie in inventar."),
    }
}

// ============================================================
// 3. if let — sintaxa scurta cand te intereseaza DOAR un caz
// ============================================================
// In loc de match cu doua brate, folosesti if let cand
// vrei sa actionezi doar daca valoarea este Some (sau Ok).

fn verifica_potion(inventar: &Vec<Item>) {
    if let Some(heal) = cauta_potion(inventar) {
        println!("  Ai o potion care vindeca {} HP.", heal);
    }
    // Daca e None, nu se intampla nimic — nu trebuie bratul else
}

// ============================================================
// 4. Metode utile pe Option
// ============================================================
//
//  .unwrap_or(val)      — returneaza valoarea sau un default
//  .unwrap_or_else(fn)  — ca unwrap_or dar default calculat dintr-o functie
//  .is_some()           — true daca e Some
//  .is_none()           — true daca e None
//
// ATENTIE: .unwrap() exista dar face panic daca e None — evita-l in productie!

fn damage_sabie(inventar: &Vec<Item>) -> i32 {
    cauta_sabie(inventar).unwrap_or(0) // daca nu ai sabie, damage = 0
}

// ============================================================
// 5. Result<T, E> — o operatie care poate esua
// ============================================================
// Result<T, E> este un enum built-in:
//
//   enum Result<T, E> {
//       Ok(T),    // succes — contine rezultatul de tip T
//       Err(E),   // esec   — contine eroarea de tip E
//   }
//
// Diferenta fata de Option:
//   Option = "valoarea poate sa lipseasca" (normal, nu e o eroare)
//   Result = "operatia a reusit sau a esuat" (esecul e exceptional)

fn ataca(atacator: &Erou, tinta: &Erou) -> Result<i32, String> {
    if atacator.hp <= 0 {
        return Err(format!("{} este mort si nu poate ataca!", atacator.nume));
    }
    if tinta.hp <= 0 {
        return Err(format!("{} este deja mort!", tinta.nume));
    }
    if atacator.mana < 10 {
        return Err(format!("{} nu are destula mana!", atacator.nume));
    }

    let dmg = damage_sabie(&atacator.inventar) + 10;
    Ok(dmg)
}

// ============================================================
// 6. match pe Result
// ============================================================

fn executa_atac(atacator: &Erou, tinta: &Erou) {
    match ataca(atacator, tinta) {
        Ok(dmg) => println!("  {} ataca si face {} damage!", atacator.nume, dmg),
        Err(msg) => println!("  Eroare: {}", msg),
    }
}

// ============================================================
// 7. Operatorul ? — propagarea erorii
// ============================================================
// In loc sa scrii match peste tot, ? face urmatorul lucru:
//   - daca Result e Ok(val), extrage val si continua
//   - daca Result e Err(e),  returneaza imediat Err(e) din functia curenta
//
// Functioneaza doar in functii care returneaza Result (sau Option).

fn quest_atac(atacator: &Erou, tinta: &Erou) -> Result<String, String> {
    let dmg = ataca(atacator, tinta)?; // daca e Err, iese automat din functie

    // Ajungem aici DOAR daca ataca() a returnat Ok
    let mesaj = format!(
        "{} completeaza quest-ul! Damage facut: {}",
        atacator.nume, dmg
    );
    Ok(mesaj)
}

// ============================================================
// MAIN — demonstratie
// ============================================================

fn main() {
    println!("=== Lectia 05: Option si Result ===\n");

    // ---------- Option ----------
    println!("--- Option ---");

    let mut erou = Erou::new("Arthas");
    erou.pick_up(Item::Sabie(35));
    erou.pick_up(Item::Potion(50));

    let inventar_gol: Vec<Item> = Vec::new();

    // match pe Option
    println!("Inventar complet:");
    afiseaza_sabie(&erou.inventar);
    println!("Inventar gol:");
    afiseaza_sabie(&inventar_gol);

    // if let
    println!("\nVerificare potion:");
    verifica_potion(&erou.inventar);
    verifica_potion(&inventar_gol);

    // unwrap_or
    println!("\nunwrap_or:");
    println!(
        "  Damage sabie (inventar complet): {}",
        damage_sabie(&erou.inventar)
    );
    println!(
        "  Damage sabie (inventar gol):     {}",
        damage_sabie(&inventar_gol)
    );

    // ---------- Result ----------
    println!("\n--- Result ---");

    let mut erou_mort = Erou::new("Zombie");
    erou_mort.hp = 0;

    let mut erou_fara_mana = Erou::new("Mage obosit");
    erou_fara_mana.mana = 0;

    let tinta = Erou::new("Goblin");

    println!("Erou viu vs Goblin:");
    executa_atac(&erou, &tinta);

    println!("Erou mort vs Goblin:");
    executa_atac(&erou_mort, &tinta);

    println!("Erou fara mana vs Goblin:");
    executa_atac(&erou_fara_mana, &tinta);

    // ---------- Operatorul ? ----------
    println!("\n--- Operatorul ? ---");

    match quest_atac(&erou, &tinta) {
        Ok(mesaj) => println!("  {}", mesaj),
        Err(e) => println!("  Quest esuat: {}", e),
    }

    match quest_atac(&erou_mort, &tinta) {
        Ok(mesaj) => println!("  {}", mesaj),
        Err(e) => println!("  Quest esuat: {}", e),
    }
}
