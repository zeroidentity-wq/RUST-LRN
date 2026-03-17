# Proiecte Practice: De la Zero la Backend Service în Rust
## Drum progresiv — fiecare proiect construiește pe cel anterior

---

# FAZA 1 — Fundamente (fără async, fără threaduri)
> Obiectiv: să devii confortabil cu Rust, ownership, și I/O de bază

---

## Proiect 1.1: CLI Todo App

**Ce construiești:** O aplicație de terminal care gestionează o listă de task-uri.
Salvează și citește dintr-un fișier JSON.

**Ce înveți:**
- Structs, enums, Vec, HashMap
- Citire/scriere fișiere
- Serializare cu serde (primul crate extern!)
- Pattern matching, error handling cu Result
- Parsare argumente din linie de comandă

**Funcționalități:**
```
todo add "Învață Rust"           → Adaugă task
todo list                        → Listează toate task-urile
todo done 1                      → Marchează task-ul #1 ca terminat
todo remove 1                    → Șterge task-ul #1
todo list --done                 → Arată doar cele terminate
todo list --pending              → Arată doar cele în așteptare
todo search "Rust"               → Caută după text
```

**Structura sugerată:**
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
enum Status {
    Pending,
    Done,
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    text: String,
    status: Status,
    created_at: String,
}

struct TodoApp {
    tasks: Vec<Task>,
    file_path: String,
}

impl TodoApp {
    fn load(path: &str) -> Result<Self, ...> { }
    fn save(&self) -> Result<(), ...> { }
    fn add(&mut self, text: String) { }
    fn complete(&mut self, id: u32) -> Result<(), ...> { }
    fn remove(&mut self, id: u32) -> Result<(), ...> { }
    fn list(&self, filter: Option<Status>) { }
    fn search(&self, query: &str) -> Vec<&Task> { }
}
```

**Crate-uri:** `serde`, `serde_json`

---

## Proiect 1.2: Analizor de log-uri

**Ce construiești:** Un tool CLI care citește fișiere de log (nginx, Apache, sau custom),
le parsează, și generează statistici.

**Ce înveți:**
- Procesare text avansată (split, parse, regex)
- HashMap pentru agregare date
- Sortare, filtrare cu iteratori
- Lucru cu fișiere mari (citire linie cu linie)
- Formatare output frumos în terminal

**Funcționalități:**
```
logparser analyze access.log
  → Top 10 URL-uri accesate
  → Top 10 IP-uri (cine a accesat cel mai mult)
  → Distribuție coduri de status (200, 404, 500...)
  → Cereri pe oră (grafic text simplu)
  → Detectare IP-uri suspecte (prea multe cereri într-un interval scurt)

logparser filter access.log --status 404
  → Arată doar erorile 404

logparser top access.log --by ip --limit 20
  → Top 20 IP-uri
```

**Format log Apache/nginx (o linie arată cam așa):**
```
192.168.1.1 - - [17/Mar/2026:10:15:32 +0000] "GET /api/users HTTP/1.1" 200 1234
```

**Structura sugerată:**
```rust
struct LogEntry {
    ip: String,
    timestamp: String,
    method: String,        // GET, POST, etc.
    path: String,          // /api/users
    status: u16,           // 200, 404, 500
    size: u64,             // bytes
}

impl LogEntry {
    fn parse(line: &str) -> Option<LogEntry> {
        // Parsează manual sau cu regex
    }
}

struct LogAnalyzer {
    entries: Vec<LogEntry>,
}

impl LogAnalyzer {
    fn top_paths(&self, limit: usize) -> Vec<(&str, usize)> { }
    fn top_ips(&self, limit: usize) -> Vec<(&str, usize)> { }
    fn status_distribution(&self) -> HashMap<u16, usize> { }
    fn requests_per_hour(&self) -> HashMap<String, usize> { }
    fn suspicious_ips(&self, threshold: usize) -> Vec<String> { }
}
```

**De ce e relevant pentru backend:** Orice backend service generează log-uri.
A ști să le analizezi e o superputere operațională. Plus, detectarea IP-urilor
suspecte e un prim pas spre securitate.

**Crate-uri:** `regex` (opțional — poți și manual)

---

## Proiect 1.3: Port Scanner simplu (sincron)

**Ce construiești:** Un tool care scanează porturile deschise pe un host.

**Ce înveți:**
- `std::net::TcpStream` — prima ta interacțiune cu rețeaua!
- Timeout-uri pe conexiuni
- Iterare prin range-uri de porturi
- Formatare output

**Funcționalități:**
```
portscan 192.168.1.1                    → Scanează porturile comune (top 100)
portscan 192.168.1.1 -p 1-1024         → Scanează range specific
portscan 192.168.1.1 -p 80,443,8080    → Scanează porturi specifice
portscan 192.168.1.1 --timeout 500     → Timeout de 500ms per port
```

**Structura sugerată:**
```rust
use std::net::TcpStream;
use std::time::Duration;

struct ScanResult {
    port: u16,
    open: bool,
    service: String,  // "HTTP", "HTTPS", "SSH" — lookup dintr-un HashMap
}

fn scan_port(host: &str, port: u16, timeout: Duration) -> bool {
    let addr = format!("{host}:{port}");
    TcpStream::connect_timeout(
        &addr.parse().unwrap(),
        timeout
    ).is_ok()
}

// HashMap cu porturi comune:
// 22 → "SSH", 80 → "HTTP", 443 → "HTTPS", 3306 → "MySQL", etc.
```

**Limitare importantă:** Versiunea asta e LENTĂ (scanează un port la un moment dat).
Asta e intenționat — o vom face rapidă cu threaduri în Faza 2.

---

# FAZA 2 — Concurență (threaduri, locks, channels)
> Obiectiv: să înțelegi cum rulezi cod în paralel, safe

---

## Proiect 2.1: Port Scanner multi-threaded

**Ce construiești:** Versiune rapidă a port scanner-ului, cu mai multe threaduri.

**Ce înveți:**
- `std::thread::spawn` — creare threaduri
- `Arc<Mutex<T>>` — partajare date safe între threaduri
- **Channels** (`mpsc`) — comunicare între threaduri
- Conceptul de thread pool
- De ce versiunea sincronă era lentă (I/O blocking)

**Arhitectura:**
```
Main thread
    │
    ├── spawn Thread 1  ──→  scanează porturile 1-200    ──→  trimite rezultate pe channel
    ├── spawn Thread 2  ──→  scanează porturile 201-400  ──→  trimite rezultate pe channel
    ├── spawn Thread 3  ──→  scanează porturile 401-600  ──→  trimite rezultate pe channel
    ...
    │
    └── Receiver  ←──  primește toate rezultatele, le sortează, le afișează
```

**Cod schelet:**
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let num_threads = 10;
    let ports_per_thread = 6553; // 65535 / 10

    for i in 0..num_threads {
        let tx = tx.clone();
        let start = i * ports_per_thread + 1;
        let end = (i + 1) * ports_per_thread;

        thread::spawn(move || {
            for port in start..=end {
                if scan_port("target", port, timeout) {
                    tx.send(port).unwrap();
                }
            }
        });
    }

    drop(tx); // Închide transmitter-ul original

    let mut open_ports: Vec<u16> = rx.iter().collect();
    open_ports.sort();

    for port in open_ports {
        println!("Port {} OPEN", port);
    }
}
```

**Componente din imagine acoperite:** ✅ Locks and atomics, ✅ Message passing via channels

---

## Proiect 2.2: Chat server TCP (multi-client)

**Ce construiești:** Un server de chat unde mai mulți clienți se conectează simultan
și pot trimite mesaje care apar la toți ceilalți.

**Ce înveți:**
- `TcpListener` + `TcpStream` — server TCP de la zero
- Un thread per client (modelul clasic)
- `Arc<Mutex<Vec<TcpStream>>>` — lista de clienți partajată
- Broadcast: când un client trimite un mesaj, îl trimiți la toți ceilalții
- Protocol simplu text (fiecare linie = un mesaj)

**Arhitectura:**
```
Server
  │
  ├── Main thread: TcpListener::bind("0.0.0.0:7878")
  │     │
  │     ├── Accept Client 1 → spawn thread → citește mesaje → broadcast la toți
  │     ├── Accept Client 2 → spawn thread → citește mesaje → broadcast la toți
  │     └── Accept Client 3 → spawn thread → citește mesaje → broadcast la toți
  │
  └── Shared state: Arc<Mutex<HashMap<String, TcpStream>>>
        (username → stream)
```

**Protocol sugertat:**
```
Client → Server:  /nick Dragon        → Setează nickname
Client → Server:  /msg Salut tuturor! → Trimite mesaj
Client → Server:  /list               → Listează userii online
Client → Server:  /quit               → Deconectare

Server → Client:  [Dragon] Salut tuturor!    → Mesaj broadcast
Server → Client:  ** Dragon a intrat **       → Notificare join
Server → Client:  ** Dragon a ieșit **        → Notificare leave
```

**Funcționalități bonus:**
- Mesaje private: `/whisper Dragon Salut doar ție`
- Camere (rooms): `/join #general`, `/join #rust`
- Istoric: ultimele 50 de mesaje sunt trimise noilor clienți

**Client:** Pentru început, folosești `telnet` sau `netcat` ca client:
```bash
nc localhost 7878
```
Apoi poți scrie un client Rust separat cu interfață în terminal.

**Componente:** ✅ Locks and atomics, ✅ Message passing via channels

---

## Proiect 2.3: Task Queue (Job Processor)

**Ce construiești:** Un sistem unde "producătorii" adaugă job-uri într-o coadă,
iar "consumatorii" (worker threads) le procesează în paralel.

**Ce înveți:**
- Pattern-ul Producer-Consumer
- Thread pool (număr fix de workeri)
- Channels pentru distribuirea job-urilor
- `Arc<Mutex<>>` pentru starea partajată
- Graceful shutdown (oprire curată)

**Arhitectura:**
```
Producător(i)
    │
    ▼
[ Channel / Coadă de job-uri ]
    │
    ├── Worker 1: preia job → procesează → raportează rezultat
    ├── Worker 2: preia job → procesează → raportează rezultat
    ├── Worker 3: preia job → procesează → raportează rezultat
    └── Worker 4: preia job → procesează → raportează rezultat
```

**Exemplu concret — Image Resizer:**
```rust
enum Job {
    Resize { path: String, width: u32, height: u32 },
    Convert { path: String, format: String },
    Compress { path: String, quality: u8 },
    Shutdown, // Semnal special de oprire
}

struct JobResult {
    job_id: u32,
    success: bool,
    duration_ms: u64,
    error: Option<String>,
}

struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new(size: usize) -> Self { }
    fn submit(&self, job: Job) { }
    fn shutdown(self) { }  // Consumă pool-ul — nu mai poți trimite job-uri
}
```

**Componente:** ✅ Locks and atomics, ✅ Message passing via channels,
✅ Error handling and reporting

---

# FAZA 3 — Async și networking avansat
> Obiectiv: înveți async/await cu Tokio și construiești servicii de rețea reale

---

## Proiect 3.1: Chat server async (cu Tokio)

**Ce construiești:** Rescrii chat server-ul din 2.2, dar cu async în loc de threaduri.

**Ce înveți:**
- **Async executors** (Tokio runtime) — piesa #1 din imagine!
- `async fn`, `.await`, `tokio::spawn`
- `tokio::net::TcpListener` / `TcpStream`
- `tokio::sync::broadcast` channel — perfect pentru chat
- Diferența thread per client vs async per client (eficiență)
- De ce async e mai bun pentru I/O-bound workloads

**Diferența cheie față de versiunea sincronă:**
```rust
// SINCRON (Faza 2) — un thread per client
thread::spawn(move || {
    handle_client(stream);  // Blocant — thread-ul stă aici
});

// ASYNC (Faza 3) — un task per client, același thread!
tokio::spawn(async move {
    handle_client(stream).await;  // Non-blocant — executorul face switch
});
```

Versiunea async poate gestiona 10.000+ clienți cu doar câteva threaduri OS,
în timp ce versiunea sincronă ar avea nevoie de 10.000 threaduri OS (imposibil).

**Crate-uri:** `tokio` (cu features: full)

**Componente:** ✅ Async executors, ✅ Message passing via channels

---

## Proiect 3.2: HTTP Server de la zero

**Ce construiești:** Un server HTTP/1.1 fără framework — parsezi request-uri raw
și construiești response-uri manual.

**Ce înveți:**
- Protocolul HTTP la nivel de bytes (request line, headers, body)
- Parsare text structurat
- Routing simplu (match pe path și metodă)
- Servire fișiere statice
- Content-Type, Status Codes
- Cum funcționează Actix/Axum "sub capotă"

**Ce trebuie parsat:**
```
GET /index.html HTTP/1.1\r\n
Host: localhost:8080\r\n
User-Agent: Mozilla/5.0\r\n
Accept: text/html\r\n
\r\n
```

**Structuri:**
```rust
enum HttpMethod { GET, POST, PUT, DELETE }

struct HttpRequest {
    method: HttpMethod,
    path: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

struct HttpResponse {
    status_code: u16,
    status_text: String,
    headers: HashMap<String, String>,
    body: String,
}

impl HttpResponse {
    fn to_bytes(&self) -> Vec<u8> {
        // Formatează: "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<body>"
    }
}
```

**Funcționalități progresive:**
1. Servește "Hello World" la orice request
2. Routing: `/` → pagina principală, `/about` → despre, `404` pentru rest
3. Servire fișiere statice din folder `./public/`
4. Parsare query parameters: `/search?q=rust`
5. Suport POST cu body JSON
6. Logging: afișează fiecare request cu timestamp

**Componente:** ✅ Async executors, ✅ Logging and tracing, ✅ Error handling

---

## Proiect 3.3: REST API cu Actix-web

**Ce construiești:** Un API REST complet pentru un sistem de bookmarks/linkuri.

**Ce înveți:**
- **Actix-web** framework — piesa din imagine!
- Routing, extractors, middleware
- Persistență cu SQLite (via `sqlx` sau `rusqlite`)
- Validare input
- **Logging și tracing** — cu `tracing` crate
- **Error handling** structurat cu tipuri custom
- Autentificare simplă (API key sau JWT)
- CORS, rate limiting

**API Endpoints:**
```
POST   /api/auth/register          → Înregistrare user
POST   /api/auth/login             → Login → primești token

GET    /api/bookmarks              → Lista bookmark-uri (cu paginare)
POST   /api/bookmarks              → Adaugă bookmark
GET    /api/bookmarks/:id          → Un bookmark specific
PUT    /api/bookmarks/:id          → Modifică bookmark
DELETE /api/bookmarks/:id          → Șterge bookmark
GET    /api/bookmarks/search?q=    → Căutare

GET    /api/tags                   → Lista tag-uri
GET    /api/tags/:tag/bookmarks    → Bookmark-urile cu un anumit tag

GET    /api/stats                  → Statistici (total, pe tag, recent)
```

**Structura proiectului:**
```
bookmark-api/
├── src/
│   ├── main.rs              → Setup server, DB, logging
│   ├── config.rs            → Configurare din env vars
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── auth.rs          → Endpoints autentificare
│   │   ├── bookmarks.rs     → CRUD bookmarks
│   │   └── tags.rs          → Endpoints tag-uri
│   ├── models/
│   │   ├── mod.rs
│   │   ├── bookmark.rs      → Struct Bookmark + DB queries
│   │   └── user.rs          → Struct User + autentificare
│   ├── middleware/
│   │   ├── auth.rs          → Verificare token
│   │   └── logging.rs       → Request logging
│   └── errors.rs            → Tipuri de erori custom
├── migrations/              → SQL migrations
├── Cargo.toml
└── .env                     → DATABASE_URL, JWT_SECRET, etc.
```

**Crate-uri:** `actix-web`, `sqlx`, `serde`, `tracing`, `tracing-subscriber`,
`jsonwebtoken`, `argon2` (hashing parole), `dotenv`

**Componente:** ✅ Async executors, ✅ Actix, ✅ Logging and tracing,
✅ Error handling and reporting

---

# FAZA 4 — Proiecte capstone (toate piesele împreună)
> Obiectiv: construiești ceva complex care folosește TOATE componentele din imagine

---

## Proiect 4.1: URL Shortener (complet, production-ready)

**Ce construiești:** Un serviciu de scurtare URL-uri (gen bit.ly) cu analytics.

**Toate piesele din imagine:**
- ✅ **Async executors** — Tokio runtime pentru servirea cererilor
- ✅ **Locks and atomics** — Cache în memorie cu `DashMap` sau `Arc<RwLock>`
- ✅ **Channels** — Background worker care procesează analytics async
- ✅ **Actix/Axum** — Web framework
- ✅ **Logging and tracing** — Request tracing cu correlation IDs
- ✅ **Error handling** — Erori structurate, răspunsuri consistente

**Arhitectura:**
```
Client → Actix Web Server
              │
              ├── POST /shorten  → Generează short URL → Salvează în DB
              ├── GET /:code     → Lookup în cache/DB → Redirect 301
              │                       │
              │                       └── Trimite event pe channel ──→ Analytics Worker
              │                                                          │
              ├── GET /api/stats/:code → Returnează statistici            ▼
              │                                                    Procesează async:
              └── Middleware pipeline:                              - Incrementează contor
                  1. Request logging (tracing)                     - Salvează IP, timestamp
                  2. Rate limiting (atomics)                       - Geolocație (opțional)
                  3. Auth check (pentru API)                       - Salvează în DB batch
```

**Funcționalități:**
```
POST /api/shorten
  Body: { "url": "https://very-long-url.com/path", "custom_code": "my-link" }
  Response: { "short_url": "http://localhost:8080/a1b2c3", "code": "a1b2c3" }

GET /a1b2c3
  → 301 Redirect la URL-ul original
  → Înregistrează vizita async (nu încetinește redirect-ul!)

GET /api/stats/a1b2c3
  → { "total_clicks": 1523, "clicks_today": 47,
      "top_referrers": [...], "clicks_per_day": [...] }

DELETE /api/links/a1b2c3  (autentificat)
  → Șterge link-ul
```

---

## Proiect 4.2: Monitoring Agent + Dashboard

**Ce construiești:** Un agent care monitorizează servere/servicii și un dashboard web.

**Toate piesele:**
- ✅ **Async executors** — Verificări periodice async (ping, HTTP check, port check)
- ✅ **Locks and atomics** — Stare partajată între checker threads și API
- ✅ **Channels** — Alerting pipeline: checker → evaluator → notifier
- ✅ **Actix** — API pentru dashboard + servire pagini web
- ✅ **Logging and tracing** — Logging structurat al fiecărui check
- ✅ **Error handling** — Clasificare erori (timeout, connection refused, DNS failure)

**Funcționalități:**
```
AGENT (rulează continuu):
  - Ping check: verifică dacă host-ul răspunde la ICMP
  - HTTP check: GET pe un URL, verifică status code și timp de răspuns
  - Port check: verifică dacă un port e deschis
  - Certificate check: verifică expirarea certificatului SSL
  - Interval configurabil per check (ex: la fiecare 30 secunde)

ALERTING:
  - Dacă un serviciu e down de 3 verificări consecutive → alertă
  - Dacă timpul de răspuns depășește threshold → warning
  - Notificări: log, webhook, email (opțional)

DASHBOARD (web):
  - Lista serviciilor monitorizate cu status live
  - Istoric uptime per serviciu (grafic)
  - Response time over time (grafic)
  - Log cu toate incidentele
```

**Configurare (TOML):**
```toml
[[checks]]
name = "Blog personal"
type = "http"
url = "https://blog.exemplu.ro"
interval_seconds = 60
expected_status = 200
timeout_ms = 5000

[[checks]]
name = "Server SSH"
type = "port"
host = "192.168.1.100"
port = 22
interval_seconds = 30

[[checks]]
name = "DNS Google"
type = "ping"
host = "8.8.8.8"
interval_seconds = 120
```

---

## Proiect 4.3: Reverse Proxy cu load balancing

**Ce construiești:** Un proxy care primește cereri și le distribuie la mai multe
servere backend, cu health checking și logging.

**Toate piesele:**
- ✅ **Async executors** — Forward async al request-urilor
- ✅ **Locks and atomics** — Lista de backend-uri sănătoase (actualizată atomic)
- ✅ **Channels** — Health check results → state updater
- ✅ **Actix/Hyper** — Primire și forward cereri HTTP
- ✅ **Logging and tracing** — Request tracing end-to-end cu timing
- ✅ **Error handling** — Backend down, timeout, retry logic

**Arhitectura:**
```
Client Request
      │
      ▼
┌─────────────────────┐
│   Reverse Proxy     │
│                     │
│  1. Log request     │
│  2. Select backend  │──→ Round-robin / Least connections / Random
│  3. Forward request │
│  4. Return response │
│  5. Log response    │
└────────┬────────────┘
         │
    ┌────┼────┐
    ▼    ▼    ▼
 Backend Backend Backend     ← Health checker verifică periodic
  :3001   :3002   :3003        dacă sunt alive
```

**Algoritmi de load balancing de implementat:**
1. **Round Robin** — pe rând: 1, 2, 3, 1, 2, 3...
2. **Weighted Round Robin** — server-ul mai puternic primește mai multe cereri
3. **Least Connections** — trimite la server-ul cu cele mai puține conexiuni active
4. **Random** — alege aleator

**Super relevant pentru rețelistică!** Proxy-urile și load balancer-ele sunt
componente fundamentale în orice infrastructură de rețea.

---

# ORDINEA RECOMANDATĂ ȘI TIMELINE

```
Luna 1-2:   Proiect 1.1 (Todo CLI)
            → Consolidezi bazele Rust

Luna 2-3:   Proiect 1.2 (Log Analyzer)
            → Procesare date, iteratori, HashMap

Luna 3-4:   Proiect 1.3 (Port Scanner sincron)
            → Prima atingere cu networking

Luna 4-5:   Proiect 2.1 (Port Scanner multi-threaded)
            → Threaduri, channels, locks

Luna 5-6:   Proiect 2.2 (Chat Server TCP)
            → Server real, protocol custom

Luna 6-7:   Proiect 2.3 (Task Queue)
            → Producer-consumer, thread pool

Luna 7-8:   Proiect 3.1 (Chat Server async)
            → Tokio, async/await

Luna 8-9:   Proiect 3.2 (HTTP Server de la zero)
            → Înțelegi HTTP la nivel de bytes

Luna 9-10:  Proiect 3.3 (REST API cu Actix)
            → Framework real, DB, auth, logging

Luna 10-12: Proiect 4.x (alege unul sau mai multe capstone)
            → Toate piesele împreună!
```

---

# CE PUI ÎN CV / PORTOFOLIU

Când termini aceste proiecte, ai un portofoliu solid:

**Junior Backend Developer:**
- REST API cu autentificare și persistență (3.3)
- Înțelegere profundă a HTTP (3.2 — ai construit un server de la zero!)

**Systems / Network Engineer:**
- Port scanner (1.3 + 2.1)
- Reverse proxy cu load balancing (4.3)
- Monitoring agent (4.2)

**DevOps / SRE:**
- Log analyzer (1.2)
- Monitoring cu alerting (4.2)
- URL shortener production-ready (4.1)

**Security:**
- Port scanner (network reconnaissance)
- Log analyzer (detectare anomalii)
- Reverse proxy (inspection, filtering)

Fiecare proiect se pune pe GitHub cu README, documentație, și teste.
Asta arată angajatorilor nu doar că știi să scrii cod, ci că știi să
construiești și să operezi software real.

---

*Începe cu Proiect 1.1 (Todo CLI) — e destul de simplu să-l termini în 1-2
săptămâni, dar destul de complex să te forțeze să folosești struct-uri, enums,
fișiere, și error handling. Când ești gata, adu-mi codul și îl analizăm împreună!*