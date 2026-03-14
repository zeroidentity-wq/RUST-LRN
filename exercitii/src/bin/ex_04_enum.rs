#[derive(Debug)]
// 1. Defineste un enum Clasa cu cel putin 3 variante (alege tu clasele)
enum Clasa {
    Archer, Mage, Warrior }

// 2. Defineste un enum Rasa cu cel putin 3 variante (ex: Elf, Human, Orc...)
enum Rasa { Elf, Human, Orc }
// 3. Creeaza un struct Erou care foloseste Clasa si Rasa in loc de String
pub struct Erou {
    name: String,
    clasa: Clasa,
    rasa: Rasa,
    hp: i32,
    inventar: Vec<Item>,
}

impl Erou {
    fn new(nume: &str, clasa: Clasa, rasa: Rasa) -> Erou {
        Erou {
            name : String::from(nume),
            clasa,
            rasa,
            hp:100,
            inventar:Vec::new(),
        }
    }
}

#[derive(Debug)]
enum Item {
    Sabie(i32),         // damage
    Armura(i32),        // defense
    Potion(i32),        // heal
    QuestItem(String),  // nume quest
}

// 4. Scrie o functie bonus_nivel(clasa: &Clasa) -> i32 care returneaza un bonus diferit per clasa (ex: Mage +5, Warrior +3, Archer +4)
fn bonus_nivel(clasa: &Clasa) -> i32 {
    match clasa {
        Clasa::Mage => { 5 },
        Clasa::Warrior => { 3 },
        Clasa::Archer => { 4 },
    }
}

// 5. In main(), creeaza un erou si afiseaza bonusul lui de nivel


fn main() {
    let erou = Erou::new("Behemud", Clasa::Archer, Rasa::Elf);
    println!("Bonus nivel erou:{:?} {}", erou.clasa ,bonus_nivel(&erou.clasa));
}
