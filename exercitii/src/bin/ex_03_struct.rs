// Exercise 03 — Struct + impl | Functii | Ownership & Borrowing
// Rulare: cargo run --bin ex_03_struct
//
// Construieste un sistem RPG simplu folosind un struct `Erou`.
// Fiecare exercitiu combina concepte din capitolele 1, 2 si 3.
//
// ============================================================
// STRUCT: Erou
// ============================================================
// Campuri necesare:
//   - nume:     String       (numele eroului)
//   - clasa:    String       (ex: "Warrior", "Mage")
//   - hp:       i32          (viata curenta)
//   - nivel:    u32          (nivelul eroului)
//   - inventar: Vec<String>  (lista de iteme detinute)
//

use rand::Rng;

struct  Erou {
    nume: String,
    clasa: String,
    hp: i32,
    nivel: i32,
    inventar: Vec<String>,
}

impl Erou {
    // 1. new(nume: &str, clasa: &str) -> Erou
    //    Constructorul eroului. Valorile initiale:
    //    hp = 100, nivel = 1, inventar = gol.
    fn new(nume: &str, clasa: &str) -> Erou {
        Erou{
            nume: String::from(nume),
            clasa: String::from(clasa),
            hp: 100,
            nivel : 1,
            inventar: Vec::new(),
        }
    }
    // 2. info(&self)
    //    Afișează toate câmpurile eroului, inclusiv inventarul.
    //    Exemplu output:
    //    [Arthur | Warrior] HP: 100 | Nivel: 1 | Inventar: ["Excalibur", "Potion"]
    fn info(&self) {
        println!("[{} | {}] HP: {} | Nivel: {} | Inventar: {:?}, putere_totala: {}", self.nume, self.clasa, self.hp, self.nivel, self.inventar, putere_totala(self));
    }
    // 3. pick_up(&mut self, item: String)
    //    Eroul preia ownership-ul unui item si il adauga in inventar.
    //    Gandeste-te: de ce parametrul este `String` si nu `&String`?

    fn pick_up(&mut self, item: String) {
        self.inventar.push(item);
        // item va fi owner ai valorii in cadrul funcției pana va fi făcut push in vector
        // din acest moment vectorul deține item-ul in inventar
    }

    // 4. take_damage(&mut self, dmg: i32)
    //    Scade dmg din hp. Daca hp ajunge la 0 sau mai mic,
    //    afiseaza "<nume> a cazut in lupta!".
    fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
        if self.hp <= 0 {
            println!("Erou {} a cazut in lupta!", self.nume);
        }
    }

    // 5. is_alive(&self) -> bool
    //    Returnează true daca hp > 0.
    fn is_alive(&self) -> bool {
        if self.hp > 0 { true } else { false }
    }

    //
    // 6. use_potion(&mut self)
    //    Cauta primul item din inventar care contine cuvantul "Potion".
    //    Daca il gaseste: il scoate din inventar si adauga +30 hp.
    //    Daca nu gaseste: afiseaza "Nu ai nicio potiune in inventar!".
    //    Indiciu: Vec are metoda .iter(), .position() si .remove(index).

    fn use_potion(&mut self) {
        let index = self.inventar.iter().position(|item| item.contains("Potion"));
        match index {
            Some(potiune) => {
                self.inventar.remove(potiune);
                self.hp += 30;
            }
            None => {
                println!("Nu ai nici o poțiune in inventar!")
            }
        }

    }

}


// ============================================================
// FUNCTII LIBERE (in afara impl)
// ============================================================
//
// 7. putere_totala(erou: &Erou) -> i32
//    Calculeaza puterea eroului: nivel * 10 + nr_iteme * 5.
//    Primeste imprumut imutabil — nu preia ownership.
//    Gandeste-te: de ce folosim `&Erou` si nu `Erou`?

fn putere_totala(erou: &Erou) -> i32 {
    erou.nivel * 10 + erou.inventar.len() as i32 * 5
}

// 8. ataca(atacator: &Erou, tinta: &mut Erou)
//    Atacatorul loveste tinta cu damage = nivel_atacator * 5.
//    Afiseaza: "<atacator> ataca <tinta> pentru <dmg> damage!".
//    Apeleaza take_damage pe tinta.
//    Gandeste-te: de ce atacatorul e `&Erou` dar tinta e `&mut Erou`?
//    Pentru ca tintei va fi nevoie sa-i modificam campul de hp current in cazul in care damage este >= 0

fn ataca(atacator: &Erou, tinta: &mut Erou) {
    let damage = rand::thread_rng().gen_range(1..=20) * atacator.nivel; // intre 1 si 20 inclusiv
    println!("{} ataca {} => {} damage.",atacator.nume, tinta.nume, damage);
    tinta.take_damage(damage);

}



// ============================================================
// MAIN — testeaza tot ce ai scris
// ============================================================
// Creeaza cel putin 2 eroi, adauga iteme, simuleaza un atac,
// foloseste o potiune si afiseaza starea finala.

fn main() {
    let mut erou_1 = Erou::new("Arthemus", "Demi-Wolf");
    erou_1.info();
    let mut erou_2 = Erou::new("Davous", "Archer");
    erou_2.info();
    erou_1.pick_up(String::from("Potion"));
    erou_2.pick_up(String::from("Potion"));
    ataca(&erou_1, &mut erou_2);
    if erou_2.is_alive() {
        println!("Erou {} is alive!", erou_2.nume);
    }
    erou_2.info();
    erou_2.use_potion();
    erou_2.info();

}