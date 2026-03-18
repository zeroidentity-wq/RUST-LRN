// Lectia 04 — Enumeratii (enum) si Pattern Matching
// Rulare: cargo run --bin 04_enums

// ============================================================
// 1. ENUM DE BAZA
// ============================================================
// Un enum defineste un set fix de variante posibile.
// O valoare de tip Clasa poate fi EXACT una dintre acestea.

#[derive(Debug)]
enum Clasa {
    Warrior,
    Mage,
    Archer,
}

// Variantele se acceseaza cu ::
// let c = Clasa::Warrior;

// ============================================================
// 2. match CU ENUM
// ============================================================
// match trebuie sa acopere TOATE variantele — compilatorul verifica.

fn descrie_clasa(clasa: &Clasa) {
    match clasa {
        Clasa::Warrior => println!("Warrior: tanc si damage corp la corp"),
        Clasa::Mage => println!("Mage: damage magic, fragil"),
        Clasa::Archer => println!("Archer: damage de la distanta"),
    }
}

// ============================================================
// 3. impl PE ENUM — metode ca la struct
// ============================================================

impl Clasa {
    fn bonus_damage(&self) -> i32 {
        match self {
            Clasa::Warrior => 10,
            Clasa::Mage => 25,
            Clasa::Archer => 15,
        }
    }
}

// ============================================================
// 4. ENUM CU DATE ASOCIATE
// ============================================================
// Fiecare varianta poate contine date de tip diferit.

#[derive(Debug)]
enum Item {
    Sabie(i32),        // damage
    Armura(i32),       // defense
    Potion(i32),       // heal
    QuestItem(String), // nume quest
}

fn descrie_item(item: &Item) {
    match item {
        Item::Sabie(dmg) => println!("Sabie — {} damage", dmg),
        Item::Armura(def) => println!("Armura — {} defense", def),
        Item::Potion(heal) => println!("Potion — vindeca {} HP", heal),
        Item::QuestItem(nume) => println!("Quest item: {}", nume),
    }
}

// ============================================================
// 5. Option<T> — enum built-in
// ============================================================
// Option este deja definit in Rust:
//   enum Option<T> {
//       Some(T),
//       None,
//   }
//
// Il folosesti cand o valoare POATE sa lipseasca.

fn gaseste_sabie(inventar: &Vec<Item>) -> Option<i32> {
    for item in inventar {
        if let Item::Sabie(dmg) = item {
            return Some(*dmg);
        }
    }
    None
}

// ============================================================
// 6. ENUM IN STRUCT
// ============================================================

#[derive(Debug)]
struct Erou {
    nume: String,
    clasa: Clasa, // enum in loc de String
    hp: i32,
    inventar: Vec<Item>,
}

impl Erou {
    fn new(nume: &str, clasa: Clasa) -> Erou {
        Erou {
            nume: String::from(nume),
            clasa, // shorthand: clasa: clasa
            hp: 100,
            inventar: Vec::new(),
        }
    }

    fn info(&self) {
        println!(
            "[{} | {:?}] HP: {} | Inventar: {:?}",
            self.nume, self.clasa, self.hp, self.inventar
        );
    }

    fn pick_up(&mut self, item: Item) {
        self.inventar.push(item);
    }

    fn use_potion(&mut self) {
        // Cautam primul Potion in inventar
        let index = self
            .inventar
            .iter()
            .position(|item| matches!(item, Item::Potion(_)));

        match index {
            Some(i) => {
                // Scoatem itemul si extragem heal-ul
                let item = self.inventar.remove(i);
                if let Item::Potion(heal) = item {
                    self.hp += heal;
                    println!(
                        "{} foloseste o potion si recupereaza {} HP!",
                        self.nume, heal
                    );
                }
            }
            None => println!("Nu ai nicio potion in inventar!"),
        }
    }

    fn damage_total(&self) -> i32 {
        // Bonus din clasa + damage din sabie (daca exista)
        let bonus_clasa = self.clasa.bonus_damage();
        let bonus_sabie: i32 = self
            .inventar
            .iter()
            .filter_map(|item| {
                if let Item::Sabie(dmg) = item {
                    Some(*dmg)
                } else {
                    None
                }
            })
            .sum();
        bonus_clasa + bonus_sabie
    }
}

// ============================================================
// MAIN — demonstratie
// ============================================================

fn main() {
    println!("=== Lectia 04: enum ===\n");

    // 1. Enum de baza + match
    let clasa = Clasa::Mage;
    descrie_clasa(&clasa);
    println!("Bonus damage Mage: {}\n", clasa.bonus_damage());

    // 2. Enum cu date asociate
    let items = vec![
        Item::Sabie(30),
        Item::Armura(15),
        Item::Potion(50),
        Item::QuestItem(String::from("Fragmentul Regalitatii")),
    ];

    for item in &items {
        descrie_item(item);
    }

    // 3. Option<T>
    println!();
    match gaseste_sabie(&items) {
        Some(dmg) => println!("Ai o sabie cu {} damage in inventar!", dmg),
        None => println!("Nu ai sabie."),
    }

    // 4. Struct cu enum
    println!();
    let mut erou = Erou::new("Zara", Clasa::Archer);
    erou.pick_up(Item::Sabie(20));
    erou.pick_up(Item::Potion(40));
    erou.info();

    println!("Damage total: {}", erou.damage_total());

    erou.use_potion();
    erou.info();
}
