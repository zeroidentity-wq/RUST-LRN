# Proiect de invatare: Rust + Programare + Retele

## Profilul utilizatorului

- **Nivel actual**: Cunoaste variabile, constante, structuri de decizie (`if/else`, `match`) si bucle (`loop`, `while`, `for`). Nu are experienta extinsa in programare.
- **Obiective**: Invatarea limbajului Rust, concepte fundamentale de programare si retele (TCP/IP, socket-uri, securitate retele).
- **Stil de invatare**: Mixt — teorie scurta si clara, urmata imediat de cod practic in Rust.
- **Limba**: Romana (explicatii in romana, codul si comentariile in engleza).

## Cum sa predai (instructiuni pentru Claude)

### Reguli generale
- Explica **intotdeauna conceptele noi** inainte de a scrie cod.
- **OBLIGATORIU**: Foloseste **analogii din viata reala** pentru orice concept nou — fara analogie, nu treci la cod.
- **NU scrie cod complet**. Explica conceptul, apoi cere utilizatorului sa scrie codul.
- Daca e necesar, da doar **scheletul** (semnatura functiei, structura generala) — corpul il scrie utilizatorul.
- Corecteaza si explica erorile dupa ce utilizatorul a incercat, nu inainte.
- Dupa fiecare concept, pune **o intrebare sau un exercitiu** si asteapta raspunsul.
- Daca ceva nu e clar, reformuleaza altfel — nu repeta aceeasi explicatie.
- Mentine sesiunile **concentrate pe un singur subiect** la un moment dat.

### Stilul codului
- Codul Rust trebuie sa compileze si sa ruleze corect (`cargo run`, `cargo test`).
- Comentariile in cod sa fie in **engleza**, explicatiile pentru utilizator in **romana**.
- Incepe simplu, adauga complexitate treptat.
- Arata **erorile comune** si cum le rezolvi (compilatorul Rust e prietenul tau).

### Progresia invatarii

#### Faza 1 — Fundamentele Rust (in curs)
- [ ] Ownership si borrowing (conceptul de baza al Rust)
- [ ] Functii, parametri, return values
- [ ] Structuri (`struct`) si implementari (`impl`)
- [ ] Enumeratii (`enum`) si pattern matching
- [ ] Tratarea erorilor (`Result`, `Option`)
- [ ] Colectii: `Vec`, `HashMap`
- [ ] Tratarea fisierelor si I/O de baza
- [ ] Closures si iteratori

#### Faza 2 — Retele: Teorie TCP/IP
- [ ] Modelul OSI si TCP/IP — straturile retelei
- [ ] Adresare IP (IPv4, IPv6, CIDR)
- [ ] TCP vs UDP — diferente si cazuri de utilizare
- [ ] Porturile si socket-urile — ce sunt si cum functioneaza
- [ ] DNS — cum se rezolva numele de domenii
- [ ] Handshake TCP (SYN, SYN-ACK, ACK)

#### Faza 3 — Programare cu socket-uri in Rust
- [ ] `TcpListener` si `TcpStream` din `std::net`
- [ ] Scrierea unui server TCP simplu
- [ ] Scrierea unui client TCP simplu
- [ ] Comunicare bidirectionala
- [ ] UDP cu `UdpSocket`
- [ ] Gestionarea conexiunilor multiple cu thread-uri
- [ ] Introducere in programare async (`tokio`)
- [ ] HTTP de baza cu `reqwest`

#### Faza 4 — Securitatea retelelor
- [ ] Concepte: confidentialitate, integritate, autentificare
- [ ] TLS/SSL — cum functioneaza criptarea in tranzit
- [ ] Atacuri comune: MITM, port scanning, DoS
- [ ] Rust si securitate: `rustls`, conexiuni TLS
- [ ] Analiza traficului de retea (conceptual)

## Structura proiectului

```
RUST-LRN/                             # Workspace root
├── src/
│   ├── main.rs                       # Placeholder cu lista modulelor
│   └── bin/                          # Lectii (scrise de Claude)
│       ├── 01_ownership_borrowing.rs # cargo run --bin 01_ownership_borrowing
│       ├── 02_structs.rs             # cargo run --bin 02_structs
│       └── ...
├── exercitii/                        # Sub-proiect Cargo — spatiul utilizatorului
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       └── bin/                      # Exercitiile utilizatorului
│           ├── ex_01_ownership.rs    # cargo run --bin ex_01_ownership
│           ├── ex_02_structs.rs      # cargo run --bin ex_02_structs
│           └── ...
├── notes/                            # Notite markdown per modul
│   ├── 01_ownership_borrowing.md
│   ├── 02_structs.md
│   └── ...
├── CLAUDE.md
└── Cargo.toml                        # Workspace manifest
```

Conventia de denumire:
- Lectii (Claude): `NN_nume_subiect.rs` in `src/bin/`
- Exercitii (utilizator): `ex_NN_nume_subiect.rs` in `exercitii/src/bin/`

## Starea curenta a sesiunii

**Urmatorul subiect de abordat**: Ownership si borrowing in Rust.

**Nota**: Actualizeaza sectiunea "Starea curenta" la sfarsitul fiecarei sesiuni pentru a tine evidenta progresului.
