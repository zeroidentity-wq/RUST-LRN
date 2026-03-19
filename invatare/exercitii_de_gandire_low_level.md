# Exerciții de Gândire Low-Level în Rust
## "Nu folosi metoda gata făcută — construiește-o tu."

---

# FILOSOFIA ACESTOR EXERCIȚII

Fiecare exercițiu are o regulă de aur: **nu folosi metoda din standard library care face
exact lucrul cerut**. Dacă trebuie să inversezi un string, nu ai voie să chemi `.reverse()`.
Dacă trebuie să sortezi, nu ai voie să chemi `.sort()`. Construiești totul de la zero,
byte cu byte, caracter cu caracter.

De ce? Pentru că atunci când înțelegi CUM funcționează lucrurile la nivel fundamental,
poți rezolva ORICE problemă — nu doar pe cele pentru care există o metodă gata făcută.

Fiecare exercițiu are:
- 🎯 **Cerința** — ce trebuie să faci
- 🚫 **Restricții** — ce NU ai voie să folosești
- 💡 **Indicii de gândire** — întrebări care te ghidează spre soluție
- 🔬 **Întrebări de aprofundare** — te fac să gândești mai departe

Încearcă să rezolvi FĂRĂ să te uiți la indicii. Dacă te blochezi, citește doar primul
indiciu. Dacă tot nu merge, citește al doilea. Nu sări direct la toate.

---

# NIVEL 1 — Gândire cu bytes și caractere

---

## Exercițiul 1.1: Inversează un string — manual, byte cu byte

🎯 **Cerința:**
Scrie o funcție `inverseaza(s: &str) -> String` care inversează un string.
"salut" → "tulas"

🚫 **Restricții:** Nu ai voie să folosești `.reverse()`, `.rev()`, sau `.chars().rev()`.

💡 **Indicii de gândire (citește pe rând, doar dacă te blochezi):**
1. Un string e o secvență de caractere. Cum ai inversa o secvență de cărți de joc
   așezate pe masă? Te-ai uita la ultima carte, apoi la penultima, etc.
2. Poți colecta toate caracterele într-un `Vec<char>`, apoi parcurgi vectorul
   de la final spre început.
3. Funcția `.len()` pe un Vec îți dă câte elemente are. Ultimul element e la
   index `len() - 1`. Penultimul la `len() - 2`. Poți folosi un `while` care
   pornește de la final.

🔬 **Întrebări de aprofundare (după ce rezolvi):**
- Ce se întâmplă dacă string-ul conține emoji sau caractere românești (ă, ș, ț)?
  Funcționează corect? De ce da sau de ce nu?
- Care e diferența dintre `.len()` pe un String (bytes) și `.chars().count()`
  (caractere)? Testează cu "café" — câți bytes are vs câte caractere?
- Poți rezolva FĂRĂ Vec, folosind doar un String gol pe care adaugi caractere?

✅ **Soluție exercițiul 1.1:**

```rust
// Varianta 1: cu Vec<char>
fn inverseaza(s: &str) -> String {
    let mut v_char: Vec<char> = Vec::new();
    for c in s.chars() {
        v_char.push(c);
    }
    let mut i = v_char.len() - 1;
    let mut reverse: Vec<char> = Vec::new();
    loop {
        reverse.push(v_char[i]);
        if i == 0 { break; }
        i -= 1;
    }
    let mut result = String::new();
    for c in reverse {
        result.push(c);
    }
    result
}

// Varianta 2: fără Vec, doar String
// ATENȚIE: folosește s.chars().count() - 1, NU s.len() - 1
// s.len() returnează bytes, nu caractere — pentru "AtaĂÎțî" ar da index greșit!
fn inverseaza_string(s: &str) -> String {
    let mut result = String::new();
    let mut i = s.chars().count() - 1;
    loop {
        result.push(s.chars().nth(i).unwrap());
        if i == 0 { break; }
        i -= 1;
    }
    result
}
```

🐛 **Bug comun**: `s.len() - 1` returnează lungimea în **bytes**, nu caractere.
Pentru "AtaĂÎțî": `len()` = 11 bytes, dar `chars().count()` = 7 caractere.
Folosind `len()` ca index în `.chars().nth()` → panic cu `.unwrap()` pe `None`.

---

🔬 **Răspunsuri la întrebările de aprofundare:**

**`café` — bytes vs caractere:**
- `.len()` = **5 bytes** (`c`=1, `a`=1, `f`=1, `é`=2 bytes în UTF-8)
- `.chars().count()` = **4 caractere**

Caracterele ASCII (a-z, 0-9) = 1 byte. Caracterele speciale (é, ă, ș, î) = 2 bytes.
Rust stochează `String` intern ca bytes UTF-8, deci `.len()` numără bytes, nu caractere vizibile.

**De ce varianta fără Vec e mai lentă?**

`.chars().nth(i)` **nu sare direct** la poziția `i` — parcurge caracterele de la început de fiecare dată:
- iterația 1: parcurge `n-1` pași
- iterația 2: parcurge `n-2` pași
- ...

Total: `n + (n-1) + ... + 1` = **O(n²)**

Varianta cu `Vec<char>` face `v_char[i]` — acces direct la memorie = **O(1)** per iterație = **O(n)** total.

---

## Exercițiul 1.2: Verifică dacă un string e palindrom

🎯 **Cerința:**
Scrie o funcție `este_palindrom(s: &str) -> bool`.
"aba" → true, "abba" → true, "abc" → false.
Bonus: ignoră spațiile și majusculele: "Ana are era na" → true (sorta... vezi mai jos).

🚫 **Restricții:** Nu ai voie să inversezi string-ul și să compari cu originalul.

💡 **Indicii de gândire:**
1. Un palindrom se citește la fel de la stânga și de la dreapta. Deci compară
   primul caracter cu ultimul, al doilea cu penultimul, etc.
2. Ai nevoie de doi "pointeri" — unul care pornește de la început, unul de la final.
   Se mișcă unul spre celălalt până se întâlnesc.
3. Colectează caracterele într-un Vec<char>. Folosește două variabile index:
   `stanga = 0` și `dreapta = lungime - 1`. Compară `vec[stanga]` cu `vec[dreapta]`.
   Dacă sunt egale, mută ambii pointeri (stanga +1, dreapta -1). Dacă nu — nu e palindrom.

🔬 **Întrebări de aprofundare:**
- Tehnica cu doi pointeri (two pointers) e un pattern FUNDAMENTAL în algoritmi.
  Unde altundeva ai putea-o folosi?
- Cum ai modifica funcția să ignore spațiile? Dar semnele de punctuație?
- Ce complexitate are soluția ta? Câte comparații faci pentru un string de N caractere?

✅ **Soluție exercițiul 1.2:**

```rust
fn este_palindrom(s: &str) -> bool {
    let trim = s.trim().to_lowercase();
    let trim_v2 = trim.replace(" ", ""); // elimină spațiile din mijloc
    let vec_chars: Vec<char> = trim_v2.chars().collect();

    if vec_chars.is_empty() {
        return true;
    }

    let mut stanga: usize = 0;
    let mut dreapta = vec_chars.len() - 1;

    while stanga < dreapta {
        if vec_chars[stanga] != vec_chars[dreapta] {
            return false;
        }
        stanga += 1;
        dreapta -= 1;
    }

    true
}
```

🐛 **Bug comun**: guard-ul `is_empty()` trebuie pus **înainte** de `vec_chars.len() - 1`,
altfel pe string gol `""` → `0 - 1` pe `usize` → panic (underflow).

🔬 **Răspunsuri la întrebările de aprofundare:**

**Complexitate:** N/2 comparații pentru un string de N caractere = **O(n)**.
Fiecare caracter e comparat o singură dată (cei doi pointeri se întâlnesc la mijloc).

**Ignorare spații:** `.replace(" ", "")` elimină toate spațiile înainte de comparație.
`"Ana are era na"` → `"anaareerna"` → palindrom ✅

---

## Exercițiul 1.3: Numără fiecare caracter (fără HashMap!)

🎯 **Cerința:**
Scrie o funcție care primește un string și returnează frecvența fiecărei litere mici (a-z).
"banana" → a:3, b:1, n:2

🚫 **Restricții:** Nu ai voie să folosești HashMap sau BTreeMap.

💡 **Indicii de gândire:**
1. Alfabetul englez are 26 de litere. Poți folosi un array de 26 de elemente: `[u32; 26]`.
   Indexul 0 = 'a', indexul 1 = 'b', ..., indexul 25 = 'z'.
2. Cum transformi un caracter în index? În Rust, poți face `c as u32 - 'a' as u32`.
   Asta îți dă 0 pentru 'a', 1 pentru 'b', etc.
3. Parcurgi string-ul caracter cu caracter, calculezi indexul, și incrementezi
   contorul la acea poziție.

🔬 **Întrebări de aprofundare:**
- Tocmai ai implementat un concept care se numește "array as a hash map".
  E mai rapid decât un HashMap real — de ce?
- Cum ai extinde soluția pentru a suporta și litere mari? Dar cifre?
- Această tehnică (mapping caracter → index) e baza a multor algoritmi. Unde
  altundeva ai putea-o folosi? (Hint: anagrame, criptografie simplă)

✅ **Soluție exercițiul 1.3:**

```rust
fn calculeaza_string(s: String) {
    let s_mic = s.to_lowercase();
    let mut frecventa: [u32; 26] = [0; 26];

    for caracter in s_mic.chars() {
        if !caracter.is_ascii_lowercase() {
            continue; // sări peste spații, cifre, caractere non-ASCII
        }
        let index = caracter as u32 - 'a' as u32;
        frecventa[index as usize] += 1;
    }

    for i in 0..26 {
        if frecventa[i] > 0 {
            let litera = (b'a' + i as u8) as char;
            println!("{}: {}", litera, frecventa[i]);
        }
    }
}
```

🐛 **Bug comun**: indexarea cu `u32` în loc de `usize` → eroare de compilare.
Array-urile în Rust se indexează **doar** cu `usize` — cast obligatoriu: `frecventa[index as usize]`.

🐛 **Bug comun 2**: fără guard `is_ascii_lowercase()`, un spațiu sau caracter românesc
cauzează **underflow** pe `u32` (`' ' as u32 - 'a' as u32` = `32 - 97` pe tip fără semn → panic).

🔬 **Răspunsuri la întrebările de aprofundare:**

**De ce e mai rapid decât HashMap?**
Array-ul are elementele unul lângă altul în memorie — acces direct la index = O(1) fără
overhead. HashMap calculează un hash, gestionează coliziuni, alocă pe heap. Pentru un
alfabet fix (26 litere), array-ul e întotdeauna mai rapid.

**Cum obții litera din index:**
Operația inversă față de `caracter as u32 - 'a' as u32`:
```rust
let litera = (b'a' + i as u8) as char;
// b'a' = 97u8, adaugi indexul, convertești la char
```

---

## Exercițiul 1.4: Caesar Cipher — criptare prin deplasare

🎯 **Cerința:**
Implementează criptarea Caesar: fiecare literă e deplasată cu N poziții în alfabet.
Cu deplasare 3: "abc" → "def", "xyz" → "abc" (se întoarce la început).

Scrie `cripteaza(text: &str, deplasare: u8) -> String` și `decripteaza(text: &str, deplasare: u8) -> String`.

🚫 **Restricții:** Nu ai voie să folosești un tabel de lookup pre-construit. Calculează
matematic poziția nouă a fiecărui caracter.

💡 **Indicii de gândire:**
1. Fiecare caracter are un cod numeric. 'a' = 97, 'b' = 98, ..., 'z' = 122.
   Poți converti cu `c as u8` și înapoi cu `rezultat as char`.
2. "Se întoarce la început" = operația modulo (%). Dacă ajungi la 26, te întorci la 0.
3. Formula: `nou_index = (index_vechi + deplasare) % 26`. Unde `index_vechi = c as u8 - b'a'`.
   `b'a'` e syntax sugar pentru `'a' as u8` (byte literal).

🔬 **Întrebări de aprofundare:**
- Ce se întâmplă cu literele mari? Dar cu spațiile și semnele de punctuație?
  (Ar trebui lăsate neschimbate.)
- Poți sparge un mesaj criptat cu Caesar fără să știi deplasarea? Cum? (Hint: în limba
  română, cele mai frecvente litere sunt e, a, i — asta se numește frequency analysis.)
- Dacă aplici criptarea de 26 de ori cu deplasare 1, ce obții?

---

# NIVEL 2 — Gândire cu algoritmi fundamentali

---

## Exercițiul 2.1: Sortare manuală — Bubble Sort

🎯 **Cerința:**
Scrie o funcție `bubble_sort(lista: &mut Vec<i32>)` care sortează vectorul crescător.

🚫 **Restricții:** Nu ai voie să folosești `.sort()`, `.sort_by()`, sau orice metodă
de sortare existentă.

💡 **Indicii de gândire:**
1. Bubble sort funcționează așa: parcurgi lista de la stânga la dreapta, și compari
   fiecare element cu vecinul din dreapta. Dacă sunt în ordine greșită, le interschimbi.
2. După o parcurgere completă, cel mai mare element "plutește" (bubble) la final.
   Dar restul s-ar putea să nu fie sortat. Deci repeți.
3. Când faci o parcurgere completă fără nicio interschimbare, lista e sortată — te oprești.

🔬 **Întrebări de aprofundare:**
- Câte parcurgeri face bubble sort în cel mai rău caz pentru N elemente?
  Câte comparații totale? (Asta se numește complexitate — O(n²) în cazul ăsta.)
- Poți optimiza: dacă la parcurgerea K ultimele K elemente sunt deja la loc,
  nu mai trebuie verificate. Implementează asta.
- Există sortări mai rapide (merge sort, quick sort). De ce ar conta viteza
  dacă listele sunt mici?

✅ **Soluție exercițiul 2.1:**

```rust
fn main() {
    let mut v = vec![3, 2, 1, 4, 5];
    bubble_sort(&mut v);
    println!("{:?}", v); // acum funcționează — v nu e consumat
}

fn bubble_sort(vec: &mut Vec<i32>) {
    let n = vec.len();
    let mut swapped: bool;
    // loop exterior — câte treceri complete
    for i in 0..n-1 {
        swapped = false;
        // loop interior — nr de elemente - iteratia - 1
        // (ultimele i elemente sunt deja sortate)
        for j in 0..n-i-1 {
            if vec[j] > vec[j+1] {
                // swap tradițional cu variabilă temp
                let temp = vec[j];
                vec[j] = vec[j+1];
                vec[j+1] = temp;
                swapped = true;
            }
        }
        // dacă nu s-a făcut niciun swap, lista e sortată
        if !swapped {
            break;
        }
    }
    println!("{:?}", vec);
}
```

🐛 **Bug comun — ownership**: `fn bubble_sort(mut vec: Vec<i32>)` consumă vectorul.
Funcția primește ownership și `main` nu mai poate folosi `v` după apel.
Soluția: `&mut Vec<i32>` — împrumut mutabil, funcția modifică vectorul original.

🐛 **Bug comun — swap greșit**:
```rust
// GREȘIT — pierde datele!
max = i;        // salvează INDEX-ul, nu valoarea
vec[i+1] = max; // suprascrie vec[i+1]
vec[i] = vec[i+1]; // acum vec[i+1] e deja modificat!

// CORECT — swap cu temp
let temp = vec[j];
vec[j] = vec[j+1];
vec[j+1] = temp;
```

🔬 **Răspunsuri la întrebările de aprofundare:**

**Complexitate:** N-1 + N-2 + ... + 1 = N(N-1)/2 comparații = **O(n²)**.
Optimizarea `n-i-1` în loop-ul interior e deja implementată — ultimele `i` elemente
sunt garantat sortate după fiecare trecere, deci nu le mai comparăm.

**De ce contează viteza chiar și pentru liste mici?** Dacă bubble sort rulează
în interiorul altui algoritm (ex: pentru fiecare nod dintr-un graf), O(n²) se
înmulțește cu dimensiunea grafului și devine rapid o problemă.

---

## Exercițiul 2.2: Căutare binară — manual

🎯 **Cerința:**
Scrie o funcție `cauta_binar(lista: &[i32], tinta: i32) -> Option<usize>` care
găsește indexul unui element într-o listă SORTATĂ.

🚫 **Restricții:** Nu ai voie să folosești `.binary_search()`, `.contains()`,
`.position()`, sau `.find()`.

💡 **Indicii de gândire:**
1. Gândește-te la jocul de ghicit un număr între 1 și 100. Întrebi "e 50?" —
   dacă e mai mare, elimini toată jumătatea inferioară. Dacă e mai mic, elimini
   jumătatea superioară. La fiecare pas, înjumătățești spațiul de căutare.
2. Ai nevoie de trei variabile: `stanga`, `dreapta`, și `mijloc`.
   La fiecare pas: `mijloc = (stanga + dreapta) / 2`. Compari `lista[mijloc]` cu ținta.
3. Dacă `lista[mijloc] == tinta` → l-ai găsit! Dacă `lista[mijloc] < tinta` →
   caută în dreapta (`stanga = mijloc + 1`). Dacă `lista[mijloc] > tinta` →
   caută în stânga (`dreapta = mijloc - 1`).

🔬 **Întrebări de aprofundare:**
- Câți pași face căutarea binară pentru o listă de 1.000.000 de elemente?
  Dar căutarea liniară (element cu element)? (Hint: log2(1.000.000) ≈ 20)
- Ce se întâmplă dacă lista NU e sortată? Funcționează căutarea binară? De ce?
- Există un bug subtil: `(stanga + dreapta) / 2` poate cauza overflow pe numere
  mari. Cum l-ai repara? (Hint: `stanga + (dreapta - stanga) / 2`)

---

## Exercițiul 2.3: Implementează propriul Stack (stivă)

🎯 **Cerința:**
Creează un struct `Stack<i32>` cu metodele:
- `new()` → creează o stivă goală
- `push(valoare)` → adaugă pe vârf
- `pop() -> Option<i32>` → scoate de pe vârf
- `peek() -> Option<&i32>` → se uită la vârf fără a scoate
- `is_empty() -> bool`
- `size() -> usize`

🚫 **Restricții:** Intern, folosește un `Vec<i32>` — dar expune DOAR interfața de stack.
Nimeni din exterior nu trebuie să poată accesa elementele din mijloc.

💡 **Indicii de gândire:**
1. Un stack e "ultimul intrat, primul ieșit" (LIFO). Ca un teanc de farfurii:
   pui farfurii deasupra, și iei tot de deasupra.
2. Vec-ul tău e privat (nu pub). Push = `vec.push()`, Pop = `vec.pop()`.
   Peek = uită-te la ultimul element fără a-l scoate.
3. Peek e mai tricky: trebuie să returnezi o referință. `self.data.last()` returnează
   `Option<&i32>`, exact ce ai nevoie.

🔬 **Întrebări de aprofundare:**
- De ce am folosit `Option<i32>` pentru pop, nu doar `i32`? Ce s-ar întâmpla
  dacă faci pop pe o stivă goală?
- Poți face Stack-ul generic? Adică `Stack<T>` care funcționează cu orice tip,
  nu doar i32? (Hint: `struct Stack<T> { data: Vec<T> }`)
- Unde se folosesc stivele în viața reală? (Hint: butonul Back din browser,
  Ctrl+Z, evaluarea expresiilor matematice, call stack-ul programului tău)

---

## Exercițiul 2.4: Implementează propriul Queue (coadă)

🎯 **Cerința:**
Creează un struct `Queue<i32>` cu metodele:
- `new()` → creează o coadă goală
- `enqueue(valoare)` → adaugă la final
- `dequeue() -> Option<i32>` → scoate de la început
- `peek() -> Option<&i32>` → se uită la primul element
- `is_empty() -> bool`
- `size() -> usize`

🚫 **Restricții:** Intern, folosește un `Vec<i32>`. Gândește-te ce se întâmplă când
scoți de la început — cum afectează asta performanța?

💡 **Indicii de gândire:**
1. O coadă e "primul intrat, primul ieșit" (FIFO). Ca o coadă la magazin.
2. Enqueue = push la final (ușor cu Vec). Dequeue = scoate primul element.
   Dar `vec.remove(0)` mută TOATE elementele cu o poziție — e lent!
3. Prima implementare: folosește `vec.remove(0)`. Funcționează, dar gândește-te
   de ce e ineficient.

🔬 **Întrebări de aprofundare:**
- `vec.remove(0)` are complexitate O(n) — mută fiecare element. Pentru 1 milion
  de elemente, fiecare dequeue mută 999.999 elemente. Cum ai putea face mai bine?
- Soluție avansată: "ring buffer" — folosești un array fix și doi pointeri
  (head și tail) care "se învârt". Poți implementa asta?
- Rust are `VecDeque` în standard library — exact un ring buffer. Dar ideea e
  să înțelegi DE CE există, nu doar să-l folosești.

---

# NIVEL 3 — Gândire cu biți și reprezentări

---

## Exercițiul 3.1: Conversie binar ↔ decimal — manual

🎯 **Cerința:**
Scrie două funcții:
- `la_binar(n: u32) -> String` — convertește un număr în reprezentarea lui binară
  ca string. Exemplu: 13 → "1101"
- `la_decimal(binar: &str) -> u32` — convertește un string binar în număr.
  Exemplu: "1101" → 13

🚫 **Restricții:** Nu ai voie să folosești `format!("{:b}", n)` sau `u32::from_str_radix`.

💡 **Indicii de gândire (la_binar):**
1. Cum convertești manual din decimal în binar? Împarți la 2 repetat.
   13 / 2 = 6 rest 1, 6 / 2 = 3 rest 0, 3 / 2 = 1 rest 1, 1 / 2 = 0 rest 1.
   Citit invers: 1101.
2. Restul la împărțire cu 2 = `n % 2`. Câtul = `n / 2`. Repetă până n = 0.
3. Resturile citite INVERS dau numărul binar. Adaugă fiecare rest la un Vec,
   apoi inversează-l (sau adaugă la începutul unui String).

💡 **Indicii de gândire (la_decimal):**
1. "1101" binar = 1×8 + 1×4 + 0×2 + 1×1 = 13. Fiecare poziție e o putere a lui 2.
2. Parcurge string-ul de la stânga la dreapta. Pentru fiecare caracter:
   `rezultat = rezultat * 2 + cifra_curentă`.

🔬 **Întrebări de aprofundare:**
- Extinde la hexadecimal (baza 16). Cifrele sunt 0-9 apoi A-F.
  255 → "FF", "1A" → 26.
- De ce computerele folosesc binar? (Hint: tranzistori — on/off, 1/0)
- Un u8 are 8 biți. Care e cel mai mare număr pe care îl poți stoca? De ce?

---

## Exercițiul 3.2: Operații pe biți — fără operatori aritmetici

🎯 **Cerința:**
Implementează adunarea a două numere FĂRĂ operatorii + sau -. Folosește doar
operații pe biți: & (AND), | (OR), ^ (XOR), << (shift left), >> (shift right).

`fn aduna(a: u32, b: u32) -> u32`

Exemplu: aduna(5, 3) → 8

🚫 **Restricții:** Nu ai voie să folosești +, -, *, /. Doar &, |, ^, <<, >>.

💡 **Indicii de gândire:**
1. Gândește-te cum aduni manual în binar:
   ```
     101   (5)
   + 011   (3)
   -----
    1000   (8)
   ```
2. XOR (^) face adunarea fără carry: 1^1=0, 1^0=1, 0^1=1, 0^0=0.
   AND (&) găsește unde apare carry: 1&1=1 (ambii biți sunt 1 = carry).
   Carry-ul trebuie deplasat la stânga cu 1 (<<1).
3. Algoritmul: `suma = a ^ b`, `carry = (a & b) << 1`.
   Repetă cu `a = suma`, `b = carry` până carry e 0.

🔬 **Întrebări de aprofundare:**
- Poți implementa și scăderea doar cu operații pe biți?
  (Hint: a - b = a + (~b + 1), unde ~b e NOT pe biți, și +1 folosește funcția ta de adunare)
- Poți implementa și înmulțirea? (Hint: înmulțirea e adunare repetată,
  dar cu biți se poate face mai eficient cu shift și add)
- Procesorul tău CHIAR face adunarea așa la nivel hardware — cu porți logice!

---

## Exercițiul 3.3: Detectează puteri ale lui 2 — cu un singur bit

🎯 **Cerința:**
Scrie o funcție `este_putere_de_2(n: u32) -> bool` care verifică dacă un număr
e putere a lui 2 (1, 2, 4, 8, 16, 32, ...).

🚫 **Restricții:** Nu ai voie să folosești buclă, nu ai voie să împarți repetat la 2,
nu ai voie să folosești logaritmi. Rezolvă-o cu O SINGURĂ operație pe biți.

💡 **Indicii de gândire:**
1. Scrie câteva puteri ale lui 2 în binar:
   1 = 0001, 2 = 0010, 4 = 0100, 8 = 1000, 16 = 10000.
   Ce au în comun? Exact UN SINGUR bit setat pe 1.
2. Acum scrie n-1 pentru fiecare: 0 = 0000, 1 = 0001, 3 = 0011, 7 = 0111, 15 = 01111.
   Compară n cu n-1. Ce observi?
3. Dacă n e putere de 2: `n & (n-1) == 0`. Asta e tot. O singură operație.

🔬 **Întrebări de aprofundare:**
- De ce funcționează `n & (n-1) == 0`? Demonstrează-ți ție în scris.
- Acest "truc" (n & (n-1)) mai face ceva util: elimină cel mai puțin semnificativ
  bit setat. Poți folosi asta pentru a număra câți biți de 1 are un număr?
  (Asta se numește "popcount" sau "Hamming weight")
- Multe structuri de date (hash maps, buffere) au dimensiuni puteri de 2. De ce?

---

## Exercițiul 3.4: Swap fără variabilă temporară

🎯 **Cerința:**
Scrie o funcție care interschimbă două numere FĂRĂ a folosi o variabilă temporară.

```rust
fn swap(a: &mut i32, b: &mut i32) {
    // ... fără "let temp = ..."
}
```

🚫 **Restricții:** Nu ai voie să declari nicio variabilă nouă. Niciun `let`.

💡 **Indicii de gândire:**
1. XOR are o proprietate magică: `a ^ a = 0` și `a ^ 0 = a`.
2. Dacă faci: `a = a ^ b`, apoi `b = a ^ b`, apoi `a = a ^ b`... ce se întâmplă?
   Ia hârtie și testează cu a=5 (101) și b=3 (011).
3. Pasul 1: a devine 110. Pasul 2: b devine 110^011 = 101 = 5 (vechiul a!).
   Pasul 3: a devine 110^101 = 011 = 3 (vechiul b!).

🔬 **Întrebări de aprofundare:**
- Ce se întâmplă dacă a și b pointează la ACEEAȘI locație de memorie? (a == b).
  Funcționează? De ce nu? (Asta e un bug real în programe C.)
- Există și varianta cu adunare/scădere: a=a+b, b=a-b, a=a-b. Ce dezavantaj are
  față de XOR? (Hint: overflow)

---

# NIVEL 4 — Gândire cu memorie și structuri

---

## Exercițiul 4.1: Implementează un String dinamic de la (aproape) zero

🎯 **Cerința:**
Creează un struct `MiniString` care stochează text, cu aceste metode:
- `new() -> MiniString`
- `push_char(c: char)` — adaugă un caracter
- `lungime() -> usize` — returnează lungimea
- `ca_str() -> &str` — returnează conținutul ca &str
- `contine(sub: &str) -> bool` — verifică dacă conține un substring

🚫 **Restricții:** Intern poți folosi `Vec<u8>`, dar implementează `contine` manual
(fără `.contains()`).

💡 **Indicii de gândire:**
1. Un string în Rust e intern un `Vec<u8>` — un vector de bytes. Fiecare caracter
   ASCII e un singur byte.
2. Pentru `push_char`: convertește char la bytes cu `c as u8` (funcționează pentru ASCII).
   Pentru UTF-8 complet, ar trebui `.encode_utf8()`.
3. Pentru `contine`: parcurge byte-ii. La fiecare poziție, verifică dacă
   următorii N bytes (unde N = lungimea substring-ului) se potrivesc.
4. Pentru `ca_str`: poți converti `&[u8]` la `&str` cu `std::str::from_utf8(&self.data).unwrap()`.

🔬 **Întrebări de aprofundare:**
- Cum gestionează Vec-ul memoria? Ce se întâmplă când adaugi mai multe caractere
  decât capacitatea inițială? (Hint: realocare — Vec-ul alocă un buffer mai mare
  și copiază tot. De aceea `.capacity()` ≠ `.len()`)
- Algoritmul tău de căutare substring e O(n×m) (naiv). Există algoritmi mai
  rapizi: Knuth-Morris-Pratt, Boyer-Moore. Caută cum funcționează KMP.

---

## Exercițiul 4.2: Linked List — de la zero

🎯 **Cerința:**
Implementează o linked list (listă înlănțuită) simplu înlănțuită, cu:
- `new()` → listă goală
- `push_front(valoare)` → adaugă la început
- `pop_front() -> Option<i32>` → scoate de la început
- `lungime() -> usize`
- `afiseaza()` → printează toate elementele

🚫 **Restricții:** Nu ai voie să folosești Vec. Construiești cu Box și Option.

💡 **Indicii de gândire:**
1. Fiecare nod al listei conține: o valoare și un pointer la nodul următor
   (sau nimic, dacă e ultimul).
   ```
   [5 | ->] -> [3 | ->] -> [8 | None]
   ```
2. Structura:
   ```rust
   struct Nod {
       valoare: i32,
       urmator: Option<Box<Nod>>,
   }
   
   struct Lista {
       cap: Option<Box<Nod>>,
   }
   ```
3. `Box<Nod>` e necesar pentru că Rust trebuie să știe dimensiunea fiecărui tip
   la compilare. Un Nod care conține un alt Nod ar avea dimensiune infinită.
   Box-ul pune nodul pe heap și stochează doar un pointer (dimensiune fixă).

🔬 **Întrebări de aprofundare:**
- De ce nu poți face pur și simplu `urmator: Option<Nod>` fără Box?
  Ce eroare primești? Ce spune ea despre cum Rust gestionează memoria?
- Linked list vs Vec: Vec-ul are elementele unul lângă altul în memorie (contiguous).
  Linked list-ul are nodurile împrăștiate prin heap. Care e mai rapid pentru acces
  la elementul 500? De ce? (Hint: cache locality)
- Poți implementa o linked list DUBLU înlănțuită? (Hint: e surprinzător de greu
  în Rust din cauza ownership — fiecare nod ar avea doi proprietari!)

---

## Exercițiul 4.3: Mini-allocator — înțelege memoria

🎯 **Cerința:**
Simulează un allocator de memorie simplu. Ai un array de 1024 bytes:
`let mut memorie: [u8; 1024] = [0; 1024];`

Implementează:
- `aloca(memorie: &mut [u8], dimensiune: usize) -> Option<usize>` — găsește
  loc liber, marchează-l ca ocupat, returnează adresa (indexul)
- `elibereaza(memorie: &mut [u8], adresa: usize, dimensiune: usize)` — marchează
  zona ca liberă

🚫 **Restricții:** Lucrezi doar cu array-ul de bytes. Nicio alocare reală pe heap.

💡 **Indicii de gândire:**
1. Cel mai simplu allocator: parcurge array-ul de la stânga la dreapta,
   găsește prima zonă de bytes liberi (valoare 0) suficient de mare.
2. Când aloci, marchează bytes-ii ca ocupați (pune 1). Când eliberezi, pune 0.
3. Ăsta se numește "first-fit allocator". E simplu dar are o problemă:
   fragmentare — după mai multe alocări și eliberări, ai "găuri" mici
   inutilizabile.

🔬 **Întrebări de aprofundare:**
- Ce e fragmentarea memoriei? Dă un exemplu concret cu allocatorul tău.
- Cum face Rust alocare pe heap cu Box::new()? Ce se întâmplă "sub capotă"?
- Allocatorul real al sistemului (malloc/free în C, sau jemalloc în Rust)
  e mult mai complex. Ce probleme adiționale trebuie să rezolve?

---

# NIVEL 5 — Gândire algoritmică avansată

---

## Exercițiul 5.1: Evaluator de expresii matematice

🎯 **Cerința:**
Scrie un program care primește un string cu o expresie matematică simplă
și returnează rezultatul.

Input: "3 + 5 * 2" → Output: 13 (respectând precedența operatorilor!)
Input: "( 3 + 5 ) * 2" → Output: 16 (respectând parantezele!)

🚫 **Restricții:** Nu ai voie să folosești nicio bibliotecă de parsare sau eval.

💡 **Indicii de gândire:**
1. Asta e o problemă clasică rezolvată cu DOI stacks: unul pentru numere,
   unul pentru operatori.
2. Algoritmul Shunting-Yard (Dijkstra): parcurgi expresia token cu token.
   Numerele merg pe stack-ul de numere. Operatorii merg pe stack-ul de operatori,
   dar ÎNAINTE verifici dacă operatorul de pe vârf are precedență mai mare sau
   egală — dacă da, evaluezi întâi pe ăla.
3. Precedența: * și / > + și -.
   '(' se pune pe stack. Când întâlnești ')', evaluezi tot până la '('.

🔬 **Întrebări de aprofundare:**
- Tocmai ai implementat un parser simplu! Compilatoarele (inclusiv rustc)
  fac asta la o scară mult mai mare.
- Poți extinde cu funcții? sin(45), sqrt(16)?
- Algoritmul Shunting-Yard convertește la "Reverse Polish Notation". Ce e asta?

---

## Exercițiul 5.2: Maze solver — găsește drumul prin labirint

🎯 **Cerința:**
Dată o matrice 2D unde '#' e perete, '.' e drum liber, 'S' e start și 'E' e ieșire,
găsește un drum de la S la E (dacă există).

```
#S###
#...#
###.#
#E..#
#####
```

Afișează traseul marcat cu '*'.

🚫 **Restricții:** Implementează manual — fără biblioteci de grafuri.

💡 **Indicii de gândire:**
1. Asta se rezolvă cu BFS (Breadth-First Search) sau DFS (Depth-First Search).
2. BFS folosește o coadă (Queue). Pornești de la S, adaugi vecinii (sus, jos, stânga,
   dreapta) care sunt '.' în coadă. Marcați celulele vizitate. Când ajungi la 'E' → gata.
3. DFS folosește un stack (sau recursie). E similar, dar explorează în adâncime.
4. BFS garantează cel mai scurt drum. DFS nu.

🔬 **Întrebări de aprofundare:**
- Care e diferența fundamentală între BFS și DFS? Când ai folosi fiecare?
- BFS și DFS sunt baza a MULTOR algoritmi: pathfinding în jocuri, crawling web,
  rute GPS, rețele de calculatoare. Poți vedea conexiunea cu rețelistica?
- Algoritmul A* e o versiune mai inteligentă — folosește o estimare a distanței
  până la destinație. Caută cum funcționează.

---

## Exercițiul 5.3: Compresie simplă — Run-Length Encoding

🎯 **Cerința:**
Implementează RLE compression:
- `comprima("aaabbbcccc")` → `"3a3b4c"`
- `decomprima("3a3b4c")` → `"aaabbbcccc"`

🚫 **Restricții:** Fără biblioteci externe. Fără regex.

💡 **Indicii de gândire (comprima):**
1. Parcurgi string-ul caracter cu caracter. Ții un contor al caracterului curent.
2. Când întâlnești un caracter DIFERIT de cel curent (sau ajungi la final):
   adaugă contorul + caracterul la rezultat, apoi resetezi.
3. Grijă: dacă un caracter apare o singură dată, scrii "1a" sau doar "a"?
   Ambele variante sunt valide — alege una și fii consistent.

🔬 **Întrebări de aprofundare:**
- RLE funcționează bine pentru "aaaaaaa" dar rău pentru "abcdef" (devine mai LUNG).
  Ce tip de date e potrivit pentru RLE? (Hint: imagini cu zone de culoare uniformă)
- Formatele BMP și PCX folosesc RLE. PNG folosește DEFLATE (LZ77 + Huffman).
  Care e diferența conceptuală?
- Poți face RLE-ul să funcționeze cu bytes în loc de caractere? Asta l-ar face
  util pentru compresie de fișiere binare.

---

# ORDINEA RECOMANDATĂ

Dacă ești la început, mergi așa:

1. **1.1** (inversare string) — te învață iterare și construcție
2. **1.2** (palindrom) — te învață two pointers
3. **1.3** (frecvență caractere) — te învață mapping
4. **1.4** (Caesar cipher) — te învață lucrul cu bytes
5. **3.1** (conversie binar) — te învață cum gândește computerul
6. **2.1** (bubble sort) — te învață sortare de la zero
7. **2.2** (căutare binară) — te învață eficiența
8. **2.3** (Stack) — te învață structuri de date
9. **2.4** (Queue) — te învață trade-off-uri de performanță
10. **3.2** (adunare cu biți) — te învață cum funcționează hardware-ul
11. **3.3** (puteri de 2) — te învață bit tricks
12. **4.1** (MiniString) — te învață cum funcționează String-ul real
13. **4.2** (Linked List) — te învață pointeri și heap
14. **5.3** (compresie RLE) — te învață procesare de date
15. **5.2** (labirint) — te învață BFS/DFS
16. **5.1** (evaluator expresii) — te învață parsare
17. **4.3** (allocator) — te învață cum funcționează memoria

Succes! Și adu-mi codul tău — îl analizăm împreună. 🦀