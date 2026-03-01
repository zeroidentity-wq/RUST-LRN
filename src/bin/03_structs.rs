// Lectia 03 — Struct-uri si implementari
// Rulare: cargo run --bin 03_structs

fn main() {
    // --- 1. Creare struct manual (fara constructor) ---
    let erou = Personaj {
        nume: String::from("Arthas"),
        hp: 100,
        mana: 80,
        nivel: 5,
    };
    println!("{} are {} HP si nivel {}", erou.nume, erou.hp, erou.nivel);

    // --- 2. Struct mutabil — campurile pot fi modificate ---
    let mut razboinic = Personaj {
        nume: String::from("Uther"),
        hp: 150,
        mana: 40,
        nivel: 8,
    };
    razboinic.hp -= 30; // a primit damage
    println!("{} are acum {} HP", razboinic.nume, razboinic.hp);

    // --- 3. Constructor ::new() ---
    let mag = Personaj::new("Jaina", 12);
    println!("{} invocat la nivel {}", mag.nume, mag.nivel);

    // --- 4. Metoda cu &self (citire) ---
    mag.descrie();

    // --- 5. Metoda cu &self care returneaza o valoare ---
    let dmg = mag.calculeaza_damage(50);
    println!("{} face {} damage", mag.nume, dmg);

    // --- 6. Metoda cu &mut self (modificare) ---
    let mut tank = Personaj::new("Thrall", 20);
    println!("Inainte de potion: {} HP", tank.hp);
    tank.bea_potion();
    println!("Dupa potion: {} HP", tank.hp);

    // --- 7. Metoda cu &mut self si parametru ---
    tank.primeste_damage(30);

    // --- 8. Struct in struct ---
    let echipa = Echipa {
        lider: Personaj::new("Sylvanas", 30),
        dimensiune: 5,
    };
    println!(
        "Echipa de {} membri, lider: {} (nivel {})",
        echipa.dimensiune, echipa.lider.nume, echipa.lider.nivel
    );
}

// ---------------------------------------------------------------------------
// 1. Definirea unui struct
// ---------------------------------------------------------------------------
// Un struct este un tip de date custom care grupeaza mai multe valori.
// Fiecare valoare se numeste "camp" (field).
//
// Analogie: Fisa de personaj dintr-un RPG.
// In loc de variabile separate (hp, mana, nivel...), le grupezi intr-un
// singur tip — ai mereu "pachetul complet" al personajului.

struct Personaj {
    nume: String,
    hp: i32,
    mana: i32,
    nivel: u32,
}

// ---------------------------------------------------------------------------
// 2. impl — Abilitatile personajului
// ---------------------------------------------------------------------------
// `impl` adauga comportament (metode) unui struct.
// E ca pagina de abilitati atasata fisei de personaj.
//
// Tipuri de metode:
//   fn f(&self)      — citeste campurile, nu le modifica
//   fn f(&mut self)  — poate modifica campurile
//   fn f(...)        — functie asociata (nu primeste self) — ex: constructor

impl Personaj {
    // --- Constructor ---
    // Nu primeste `self` — creeaza o instanta noua de la zero.
    // Apelat cu :: (Personaj::new), nu cu . (erou.new).
    fn new(nume: &str, nivel: u32) -> Personaj {
        Personaj {
            nume: String::from(nume),
            hp: 100,
            mana: 50,
            nivel, // shorthand: echivalent cu `nivel: nivel`
        }
    }

    // --- Metoda cu &self — doar citire ---
    // `self` e imprumut imutabil — metoda nu poate schimba nimic.
    fn descrie(&self) {
        println!(
            "[{}] Nivel {} | HP: {} | Mana: {}",
            self.nume, self.nivel, self.hp, self.mana
        );
    }

    // --- Metoda cu &self care returneaza o valoare ---
    fn calculeaza_damage(&self, baza: i32) -> i32 {
        baza + self.nivel as i32 * 10
    }

    // --- Metoda cu &mut self — modifica campurile ---
    // Instanta trebuie declarata cu `mut` ca sa poti apela aceasta metoda.
    fn bea_potion(&mut self) {
        self.hp += 50;
        println!("{} a baut o potion! +50 HP", self.nume);
    }

    // --- &mut self cu parametru ---
    fn primeste_damage(&mut self, dmg: i32) {
        self.hp -= dmg;
        if self.hp <= 0 {
            println!("{} a cazut in lupta!", self.nume);
        } else {
            println!("{} are {} HP ramas", self.nume, self.hp);
        }
    }
}

// ---------------------------------------------------------------------------
// 3. Struct in struct
// ---------------------------------------------------------------------------
// Campurile unui struct pot fi ele insele struct-uri.

struct Echipa {
    lider: Personaj,
    dimensiune: u32,
}
