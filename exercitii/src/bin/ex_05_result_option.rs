// Exercitii 05 — Option si Result
// Rulare: cargo run --bin ex_05_result_option

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
            mana: 100,
            inventar: Vec::new(),
        }
    }

    fn add_item(&mut self, item: Item) {
        self.inventar.push(item);
    }

    fn afiseaza_inventar(&self) {
        println!(
            "afiseaza inventar erou: {} = {:?}",
            self.nume, self.inventar
        );
    }

    fn afiseaza_hp(&self) {
        println!("{} are {} HP!", self.nume, self.hp);
    }
}

// ============================================================
// EXERCITIUL 1 — Result
// ============================================================
// Scrie functia `divide(a: i32, b: i32) -> Result<i32, String>`
// - daca b == 0, returneaza Err cu un mesaj
// - altfel, returneaza Ok cu rezultatul impartirii
//
// In `main`, apeleaza:
//   divide(10, 2)  -> printeaza rezultatul
//   divide(10, 0)  -> printeaza eroarea
// Foloseste `match` pentru ambele cazuri.
// ============================================================

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// ============================================================
// EXERCITIUL 2 — Option
// ============================================================
// Scrie functia `gaseste_potion(erou: &Erou) -> Option<i32>`
// - cauta in inventarul eroului primul `Item::Potion`
// - daca gaseste, returneaza `Some(valoarea_potiunii)`
// - daca nu gaseste, returneaza `None`
//
// In `main`, creeaza doi eroi:
//   - primul ARE o Potion in inventar
//   - al doilea NU are nicio Potion
// Apeleaza functia pentru amandoi si printeaza rezultatul cu `match`.
// ============================================================

fn gaseste_potion(erou: &Erou) -> Option<i32> {
    for item in &erou.inventar {
        match item {
            Item::Potion(v) => return Some(*v),
            _ => {}
        }
    }
    None
}

// ============================================================
// EXERCITIUL 3 — Result cu struct
// ============================================================
// Scrie functia `aplica_dmg(erou: &mut Erou, dmg: i32) -> Result<i32, String>`
// - daca dmg < 0, returneaza Err("Damage-ul nu poate fi negativ")
// - daca dmg > erou.hp, returneaza Err("Eroul a murit")
// - altfel, scade dmg din erou.hp si returneaza Ok(erou.hp ramas)
//
// In `main`, testeaza toate cele 3 cazuri (dmg valid, dmg negativ, dmg fatal).
// ============================================================

fn aplica_dmg(erou: &mut Erou, dmg: i32) -> Result<i32, String> {
    if dmg < 0 {
        return Err("Dmg must be greater than 0".to_string());
    } else if dmg > erou.hp {
        return Err("Eroul a murit!".to_string());
    } else {
        erou.hp -= dmg;
    }
    Ok(erou.hp)
}

// 4. Scrie executa_quest — apeleaza aplica_dmg si propaga eroarea cu ?
fn vindeca_erou(erou: &mut Erou) -> Result<i32, String> {
    erou.hp += 10;
    Ok(gaseste_potion(erou).ok_or("Nu exista nicio potion in inventar".to_string())?)
}

fn main() {
    match divide(5, 3) {
        Ok(rezulat) => {
            println!("Rezulat:{}", rezulat);
        }
        Err(eroare) => {
            println!("{}", eroare);
        }
    }

    match divide(10, 0) {
        Ok(rezulat) => {
            println!("Rezulat: {}", rezulat);
        }
        Err(eroare) => {
            println!("{}", eroare);
        }
    }

    let mut erou1 = Erou::new("Behemudo");
    let mut erou2 = Erou::new("Barbos");

    erou1.add_item(Item::Potion(1));
    erou1.add_item(Item::Potion(2));
    erou1.add_item(Item::Potion(3));
    erou1.afiseaza_inventar();
    erou2.afiseaza_inventar();

    match gaseste_potion(&erou1) {
        Some(rezulat) => println!("Erou: {} are {} poțiuni!", erou1.nume, rezulat),
        None => println!("Erou: {} nu are poțiuni!", erou1.nume),
    }

    match gaseste_potion(&erou2) {
        Some(rezulat) => println!("Erou: {} are {} poțiuni!", erou2.nume, rezulat),
        None => println!("Erou: {} nu are poțiuni!", erou2.nume),
    }

    // dmg valid, dmg negativ, dmg fatal
    match aplica_dmg(&mut erou1, 30) {
        Ok(hp_ramas) => {
            println!("Atack reusit! Erou: {} HP Ramas: {}", erou1.nume, hp_ramas);
        }
        Err(error) => {
            println!("Eraore: {}", error);
        }
    }
    match aplica_dmg(&mut erou1, -5) {
        Ok(hp_ramas) => {
            println!("Atack reusit! Erou: {} HP Ramas: {}", erou1.nume, hp_ramas);
        }
        Err(error) => {
            println!("Eraore: {}", error);
        }
    }

    erou1.afiseaza_hp();
    match vindeca_erou(&mut erou1) {
        Ok(valoare) => {
            println!("Vindeca erou:{} {}", erou1.nume, valoare);
        }
        Err(error) => {
            println!("{}", error);
        }
    }
    erou1.afiseaza_hp();

    match aplica_dmg(&mut erou1, 105) {
        Ok(hp_ramas) => {
            println!("Atack reusit! Erou: {} HP Ramas: {}", erou1.nume, hp_ramas);
        }
        Err(error) => {
            println!("Eraore: {}", error);
        }
    }
}
