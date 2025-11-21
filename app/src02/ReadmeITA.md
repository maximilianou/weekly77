# src02 - Sistema di E-Commerce in Rust

Un progetto Rust funzionale e asincrono che dimostra la modellazione del dominio, la persistenza e i modelli di architettura pulita per una piattaforma di e-commerce. Costruito con **Tokio**, **SQLx**, **Serde** e **Clap**.

## 📑 Indice

1. [Descrizione Generale](#descrizione-generale)
2. [Struttura del Progetto](#struttura-del-progetto)
3. [Prerequisiti](#prerequisiti)
4. [Avvio Rapido](#avvio-rapido)
5. [Compilazione del Progetto](#compilazione-del-progetto)
6. [Esecuzione del Progetto](#esecuzione-del-progetto)
7. [Test](#test)
8. [Errori di Compilazione Comuni e Soluzioni](#errori-di-compilazione-comuni-e-soluzioni)
9. [Architettura e Progettazione](#architettura-e-progettazione)
10. [Documentazione API](#documentazione-api)
11. [Esempi](#esempi)
12. [Contribuire](#contribuire)

---

## Descrizione Generale

**src02** è un modello pronto per la produzione per costruire sistemi di e-commerce in Rust. Dimostra:

- Stile di **Programmazione Funzionale** con valori immutabili e funzioni pure
- Modelli **Async/Await** usando Tokio
- **Persistenza del Database** con SQLx (supporto SQLite, facilmente estendibile a PostgreSQL)
- **Modellazione del Dominio Type-Safe** usando il sistema di tipi di Rust
- **Gestione della Configurazione** tramite flag CLI e file `.env`
- **Test Esaustivi** con test unitari e di integrazione
- **Codice Pulito**: senza codice morto, avvisi minimi

### Casi d'Uso

- Imparare modelli di programmazione funzionale in Rust
- Costruire un backend di e-commerce scalabile
- Comprendere operazioni asincrone di database
- Praticare strutture dati immutabili e gestione dello stato
- Modello per microservizi in produzione

---

## Struttura del Progetto

```
src02/
├── Cargo.toml                 # Manifesto del progetto e dipendenze
├── Makefile                   # Automazione della compilazione
├── README.md                  # Documentazione in inglese
├── ReadmeES.md                # Documentazione in spagnolo
├── ReadmeITA.md               # Questo file (Italiano)
├── .env.example               # Esempio di configurazione
├── src/
│   ├── lib.rs                 # Radice della libreria, export dei moduli
│   ├── models.rs              # Tipi di dominio: User, Product, Service, Payment
│   ├── catalog.rs             # Query del catalogo
│   ├── usage.rs               # Registrazione dell'utilizzo e risoluzione pagamenti
│   ├── persistence.rs         # Operazioni database con SQLx
│   ├── main.rs                # (deprecato, usare bin/main.rs)
│   └── bin/
│       └── main.rs            # Punto di ingresso binario con CLI
├── tests/
│   ├── integration_tests.rs   # Test della logica principale
│   ├── catalog_tests.rs       # Test specifici del catalogo
│   └── persistence_tests.rs   # Test di persistenza
└── target/                    # Artefatti compilati (generati)
```

---

## Prerequisiti

### Richiesti

- **Rust 1.91.0 o successivo** — [Installa Rust](https://rustup.rs/)
- **Cargo** — viene con Rust
- **Make** — per eseguire target del Makefile (opzionale, ma consigliato)

### Opzionali

- **SQLite 3.x** — per database con file (normalmente preinstallato su Linux/macOS)
- **Git** — per controllo versione

### Verificare l'Installazione

```bash
# Verificare versioni di Rust e Cargo
rustc --version
cargo --version

# Verificare Make (se necessario)
make --version
```

---

## Avvio Rapido

### 1. Clonare / Configurare il Progetto

```bash
# Navigare al progetto
cd /home/dev01/projects/weekly77/app/src02

# Verificare la struttura
ls -la
```

### 2. Visualizzare i Comandi Disponibili

```bash
make help
```

Mostrerà tutti i target disponibili del Makefile.

### 3. Eseguire la Demo (In Memoria)

```bash
make demo
```

Output atteso: lista di prodotti, servizi e registri di utilizzo.

### 4. Eseguire i Test

```bash
make test
```

I 4 test di integrazione dovrebbero passare.

### 5. Creare e Inizializzare un Database con File

```bash
make run-file-create
```

Crea `shop_demo.db` nella radice del progetto.

---

## Compilazione del Progetto

### Compilare (Debug)

```bash
make build
# oppure direttamente:
cargo build
```

**Output:** artefatti compilati in `target/debug/`

### Compilare (Release - Ottimizzato)

```bash
make build-release
# oppure direttamente:
cargo build --release
```

**Output:** binario ottimizzato in `target/release/src02`

### Pulire gli Artefatti di Compilazione

```bash
make clean
```

Rimuove la directory `target/` e i file del database.

---

## Esecuzione del Progetto

### Opzione 1: Modalità Demo (In Memoria, Senza DB)

```bash
make demo
```

Esegue dati di esempio in memoria e stampa i risultati. Utile per comprendere il flusso dei dati senza configurare il database.

### Opzione 2: Database in Memoria

```bash
make run-inmemory
```

Inizializza un database SQLite in memoria ed esegue l'applicazione.

### Opzione 3: Database con File

**Prima volta:**

```bash
make run-file-create
```

Crea `shop_demo.db` e inizializza lo schema.

**Esecuzioni successive:**

```bash
make run-file
```

### Opzione 4: Usando Variabili d'Ambiente

Creare un file `.env`:

```bash
echo "DB_URL=sqlite:mio_custom.db" > .env
```

Quindi eseguire:

```bash
cargo run --bin src02 -- --init-db
```

### Opzione 5: Argomenti della Linea di Comando Personalizzati

```bash
cargo run --bin src02 -- --help
```

Mostra le opzioni CLI disponibili.

---

## Test

### Eseguire Tutti i Test

```bash
make test
```

Esegue test unitari e di integrazione in sequenza.

### Eseguire Solo i Test Unitari

```bash
make unit-test
```

Test solo del codice della libreria.

### Eseguire Solo i Test di Integrazione

```bash
make integration-test
```

Test del database e scenari end-to-end.

### Eseguire i Test con Output Dettagliato

```bash
make test-verbose
```

Mostra output dettagliato per ogni test.

---

## Errori di Compilazione Comuni e Soluzioni

### Errore 1: `DATABASE_URL` non impostato

**Sintomo:**
```
error: set `DATABASE_URL` to use query macros online
```

**Soluzione:**
Il nostro progetto usa funzioni `sqlx::query()` a runtime, non macro `query!`. Se si migra a query verificate offline, impostare:

```bash
export DATABASE_URL="sqlite:shop_demo.db"
cargo build
```

---

### Errore 2: Versione di Rust Troppo Vecchia

**Sintomo:**
```
error[E0658]: use of unstable feature
```

**Soluzione:**
Aggiornare Rust a 1.91.0 o successivo:

```bash
rustup update stable
rustc --version  # Verificare
```

---

### Errore 3: Strumenti di Compilazione Mancanti (Windows)

**Sintomo:**
```
error: linker `link.exe` not found
```

**Soluzione (Windows):**

Installare Visual C++ build tools:

```bash
# Via Visual Studio Installer o standalone:
https://visualstudio.microsoft.com/visual-cpp-build-tools/
```

Quindi ricompilare:

```bash
cargo clean
cargo build
```

---

### Errore 4: Header di Sviluppo SQLite Mancanti (Linux)

**Sintomo:**
```
error: failed to run custom build command for `libsqlite3-sys`
```

**Soluzione (Ubuntu/Debian):**

```bash
sudo apt-get update
sudo apt-get install libsqlite3-dev
cargo clean
cargo build
```

**Soluzione (Fedora/RHEL):**

```bash
sudo dnf install sqlite-devel
cargo build
```

---

### Errore 5: Avvisi di Clippy Durante la Compilazione

**Sintomo:**
```
warning: manual implementation of Option::map
```

**Soluzione:**
Eseguire il formattatore e il linter:

```bash
make fmt     # Auto-formatta il codice
make check   # Esegue clippy con warnings-as-errors
```

---

## Architettura e Progettazione

### Modello di Dominio

Il progetto utilizza strutture dati funzionali e immutabili.

#### **Modelli** (`src/models.rs`)

- **User** — Rappresenta un cliente
- **PaymentMethod** — Enum dei tipi di pagamento (Carta, PayPal, ecc.)
- **Product** — Articolo individuale acquistabile
- **Service** — Collezione di prodotti
- **ServiceUsage** — Registrazione dell'utilizzo di servizio/prodotto da parte dell'utente

#### **Catalogo** (`src/catalog.rs`)

Operazioni pure e solo di consultazione:

- `with_service(service)` — Aggiunge un servizio
- `list_all_products()` — Restituisce tutti i prodotti
- `list_products_for_service(id)` — Restituisce i prodotti di un servizio

#### **Utilizzo** (`src/usage.rs`)

Gestisce i registri di utilizzo e la risoluzione dei pagamenti:

- `UsageLog` — Collezione immutabile di registri
- `add_usage(usage)` — Restituisce un nuovo UsageLog con utilizzo aggiunto
- `resolve_payment_for_usage(user, payment)` — Applica la gerarchia dei pagamenti

#### **Persistenza** (`src/persistence.rs`)

Operazioni asincrone di database con SQLx:

- `init_db(pool)` — Crea le tabelle
- `save_user()`, `save_service()`, `save_usage()`
- `get_users()`, `get_services()`, `get_usages_for_user()`

---

## Documentazione API

Generare documentazione Rust:

```bash
make doc
```

---

## Esempi

Vedere la sezione "Esempi" in README.md per esempi di codice completi.

---

## Contribuire

### Qualità del Codice

Prima di fare il commit:

```bash
make fmt      # Formatta il codice
make check    # Verifica lo stile
make test     # Esegue i test
```

---

**Buona programmazione! 🚀**
