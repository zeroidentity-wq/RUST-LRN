use anyhow::{Context, Result};
use clap::Parser;
use log::{debug, error, info, log_enabled, warn, Level};

#[derive(Parser)]
// aici ii spui sa implementeze interfata Parser pentru structul Cli
// Compilator uita-te la campurile structurii mele si scrie tu automat implement
// Parser for Cli {} pentru mine folosind regulile definite de clap

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    warn!("Startup..[+]");
    /* patern: grrs text fisier.txt
    let pattern = std::env::args().nth(1).expect("Nu ai dat pattern"); // text-ul de cautat
    let path = std::env::args()
        .nth(2)
        .expect("Nu ai furnizat /path pentru fisier");

    let args = Cli {
        pattern,
        path: std::path::PathBuf::from(path),
    };
    */
    let args = Cli::parse();
    info!("Argumentele au fost parsate!");

    // let continut = std::fs::read_to_string(&args.path).expect("Nu am reusit sa citesc fisierul!");
    let continut = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Nu am putut citi fisierul {}", args.path.display()))?;

    grrs::find_matches(&continut, &args.pattern, &mut std::io::stdout());

    /*
    match continut {
        Ok(content) => {
            println!("Fisierul contine: {}", content)
        }
        Err(eroare) => {
            println!("Ohh nUU: {}", eroare);
        }
    }
    */

    // println!("{}", continut);

    for linie in continut.lines() {
        if linie.contains(&args.pattern) {
            println!("For: {}", linie);
        }
    }

    // println!("Patters: {:?}, Path: {:?}", args.pattern, args.path);
    Ok(())
}

/*
 *
 * */
