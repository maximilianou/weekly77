# src02 - Rust E-Commerce System

A functional, async-first Rust project demonstrating domain modeling, persistence, and clean architecture patterns for an e-commerce platform. Built with **Tokio**, **SQLx**, **Serde**, and **Clap**.

## 📑 Table of Contents

1. [Overview](#overview)
2. [Project Structure](#project-structure)
3. [Prerequisites](#prerequisites)
4. [Quick Start](#quick-start)
5. [Building the Project](#building-the-project)
6. [Running the Project](#running-the-project)
7. [Testing](#testing)
8. [Common Compilation Errors & Solutions](#common-compilation-errors--solutions)
9. [Architecture & Design](#architecture--design)
10. [API Documentation](#api-documentation)
11. [Examples](#examples)
12. [Contributing](#contributing)

---

## Overview

**src02** is a production-ready template for building e-commerce systems in Rust. It demonstrates:

- **Functional Programming** style with immutable values and pure functions
- **Async/Await** patterns using Tokio
- **Database Persistence** with SQLx (SQLite support, easily extended to PostgreSQL)
- **Type-Safe Domain Modeling** using Rust's type system
- **Configuration Management** via CLI flags and `.env` files
- **Comprehensive Testing** with unit and integration tests
- **Clean Code** practices: no dead code, minimal warnings

### Use Cases

- Learn functional programming patterns in Rust
- Build a scalable e-commerce backend
- Understand async database operations
- Practice immutable data structures and state management
- Template for production microservices

---

## Project Structure

```
src02/
├── Cargo.toml                 # Project manifest & dependencies
├── Makefile                   # Build automation targets
├── README.md                  # This file (English)
├── ReadmeES.md                # Spanish documentation
├── ReadmeITA.md               # Italian documentation
├── .env.example               # Example environment configuration
├── src/
│   ├── lib.rs                 # Library root, module exports, demo runner
│   ├── models.rs              # Domain types: User, Product, Service, Payment
│   ├── catalog.rs             # Product/Service catalog queries
│   ├── usage.rs               # Service usage logging and payment resolution
│   ├── persistence.rs         # SQLx async database operations
│   ├── main.rs                # (deprecated, use bin/main.rs instead)
│   └── bin/
│       └── main.rs            # CLI binary entry point
├── tests/
│   ├── integration_tests.rs   # Core logic tests
│   ├── catalog_tests.rs       # Catalog-specific tests
│   └── persistence_tests.rs   # Database persistence tests
└── target/                    # Compiled artifacts (generated)
```

---

## Prerequisites

### Required

- **Rust 1.91.0 or later** — [Install Rust](https://rustup.rs/)
- **Cargo** — comes with Rust
- **Make** — for running Makefile targets (optional, but recommended)

### Optional

- **SQLite 3.x** — for file-backed database (usually pre-installed on Linux/macOS)
- **Git** — for version control

### Verify Installation

```bash
# Check Rust & Cargo versions
rustc --version
cargo --version

# Verify Make (if needed)
make --version
```

---

## Quick Start

### 1. Clone / Set Up the Project

```bash
# Navigate to the project
cd /home/dev01/projects/weekly77/app/src02

# Verify structure
ls -la
```

### 2. View Available Commands

```bash
make help
```

Output will show all available Makefile targets.

### 3. Run the Demo (In-Memory)

```bash
make demo
```

Expected output: list of products, services, and usage logs.

### 4. Run Tests

```bash
make test
```

All 4 integration tests should pass.

### 5. Create and Initialize a File-Backed Database

```bash
make run-file-create
```

Creates `shop_demo.db` in the project root.

---

## Building the Project

### Build (Debug)

```bash
make build
# or directly:
cargo build
```

**Output:** compiled artifacts in `target/debug/`

### Build (Release - Optimized)

```bash
make build-release
# or directly:
cargo build --release
```

**Output:** optimized binary in `target/release/src02`

### Clean Build Artifacts

```bash
make clean
```

Removes `target/` directory and database files.

---

## Running the Project

### Option 1: Demo Mode (In-Memory, No DB)

```bash
make demo
```

Runs sample data in memory and prints results to stdout. Useful for understanding data flow without database setup.

### Option 2: In-Memory Database

```bash
make run-inmemory
```

Initializes an in-memory SQLite database and runs the application.

### Option 3: File-Backed Database

**First time:**

```bash
make run-file-create
```

Creates `shop_demo.db` and initializes schema.

**Subsequent runs:**

```bash
make run-file
```

### Option 4: Using Environment Variables

Create a `.env` file:

```bash
echo "DB_URL=sqlite:my_custom.db" > .env
```

Then run:

```bash
cargo run --bin src02 -- --init-db
```

### Option 5: Custom Command Line Arguments

```bash
cargo run --bin src02 -- --help
```

Shows available CLI options:

```
OPTIONS:
  --db-url <DB_URL>       Database URL (default: sqlite::memory:)
  --init-db               Initialize database schema
  --demo                  Run demo with sample data
  -h, --help              Print help
  -V, --version           Print version
```

---

## Testing

### Run All Tests

```bash
make test
```

Runs both unit and integration tests sequentially.

### Run Unit Tests Only

```bash
make unit-test
```

Tests library code only (no binary tests).

### Run Integration Tests Only

```bash
make integration-test
```

Tests database and end-to-end scenarios.

### Run Tests with Verbose Output

```bash
make test-verbose
```

Shows detailed output for each test.

### Test Coverage

Current tests cover:

1. **Catalog Tests** (`tests/catalog_tests.rs`)
   - Product listing
   - Service retrieval
   - Immutable catalog operations

2. **Integration Tests** (`tests/integration_tests.rs`)
   - Payment resolution logic
   - Functional data transformations

3. **Persistence Tests** (`tests/persistence_tests.rs`)
   - In-memory SQLite operations
   - User/Service/Usage CRUD operations
   - Data serialization (JSON for PaymentMethod)

---

## Common Compilation Errors & Solutions

### Error 1: `DATABASE_URL` not set (if using `sqlx::query!` macros)

**Symptom:**
```
error: set `DATABASE_URL` to use query macros online
```

**Solution:**
Our project uses runtime `sqlx::query()` functions, not `query!` macros, so this error should not occur. If migrating to offline-checked queries, set:

```bash
export DATABASE_URL="sqlite:shop_demo.db"
cargo build
```

---

### Error 2: Rust Version Too Old

**Symptom:**
```
error[E0658]: use of unstable feature
```

**Solution:**
Update Rust to 1.91.0 or later:

```bash
rustup update stable
rustc --version  # Verify
```

---

### Error 3: Missing Build Tools (Windows)

**Symptom:**
```
error: linker `link.exe` not found
```

**Solution (Windows):**

Install Visual C++ build tools:

```bash
# Via Visual Studio Installer or standalone:
https://visualstudio.microsoft.com/visual-cpp-build-tools/
```

Then rebuild:

```bash
cargo clean
cargo build
```

---

### Error 4: SQLite Development Headers Missing (Linux)

**Symptom:**
```
error: failed to run custom build command for `libsqlite3-sys`
```

**Solution (Ubuntu/Debian):**

```bash
sudo apt-get update
sudo apt-get install libsqlite3-dev
cargo clean
cargo build
```

**Solution (Fedora/RHEL):**

```bash
sudo dnf install sqlite-devel
cargo build
```

---

### Error 5: Clippy Warnings During Compilation

**Symptom:**
```
warning: manual implementation of Option::map
```

**Solution:**
Run code formatter and linter:

```bash
make fmt     # Auto-formats code
make check   # Runs clippy with warnings-as-errors
```

All warnings in src02 have been resolved. If you see new ones, ensure you're on the latest version:

```bash
cargo update
cargo clean
cargo build
```

---

### Error 6: Port Already in Use (if extending with web server)

**Symptom:**
```
error: Address already in use
```

**Solution:**
Find and kill the process using the port:

```bash
# Linux/macOS
lsof -i :8080
kill -9 <PID>

# Windows
netstat -ano | findstr :8080
taskkill /PID <PID> /F
```

---

## Architecture & Design

### Domain Model

The project uses functional, immutable data structures:

#### **Models** (`src/models.rs`)

- **User** — Represents a customer
  - Fields: `id`, `profile` (display_name + default_payment)
  - Immutable; payment is `Option<PaymentMethod>`

- **PaymentMethod** — Enum of payment types
  - Card (number masked + holder name)
  - PayPal (account identifier)
  - Easily extended with more variants

- **Product** — Individual purchasable item
  - Fields: `id`, `name`, `price_cents`
  - Part of a Service

- **Service** — Collection of Products offered to users
  - Fields: `id`, `name`, `products: Vec<Product>`
  - Immutable; no side effects

- **ServiceUsage** — Record of a service/product used by a user
  - Fields: `user_id`, `service_id`, `product_id`, `payment_used: Option<PaymentMethod>`
  - Tracks which payment was used (or defaults to user's profile default)

#### **Catalog** (`src/catalog.rs`)

Pure, query-only operations on services and products:

- `with_service(service)` — Adds a service (returns new Catalog)
- `list_all_products()` — Returns all products across services
- `list_products_for_service(id)` — Returns products for a specific service
- `get_service(id)` — Retrieves a service by ID

#### **Usage** (`src/usage.rs`)

Manages service usage logs and payment resolution:

- `UsageLog` — Immutable collection of `ServiceUsage` records
- `add_usage(usage)` — Returns a new `UsageLog` with the usage appended (functional style)
- `service_usages_for_user(user_id)` — Filters usages by user
- `resolve_payment_for_usage(user, payment)` — Applies payment hierarchy: explicit > user default > None

#### **Persistence** (`src/persistence.rs`)

SQLx-based async database operations:

- `init_db(pool)` — Creates tables (users, services, products, usages)
- `save_user(pool, user)` — Inserts/updates user record
- `save_service(pool, service)` — Inserts service + products
- `save_usage(pool, usage)` — Records a service usage
- `get_users(pool)` — Retrieves all users from DB
- `get_services(pool)` — Retrieves all services + products
- `get_usages_for_user(pool, user_id)` — Queries usages by user

### Functional Principles

1. **Immutability** — Data structures are immutable; operations return new instances
2. **Pure Functions** — No side effects; same input always produces same output
3. **Composition** — Small functions composed into larger operations
4. **Type Safety** — Rust's type system prevents entire classes of bugs (null refs, thread issues, etc.)

---

## API Documentation

Generate Rust documentation:

```bash
make doc
```

Opens HTML documentation in your default browser showing all public types and functions.

To build without opening:

```bash
cargo doc --no-deps
```

Output: `target/doc/src02/index.html`

---

## Examples

### Example 1: Create Users and Services

```rust
use src02::models::*;

let alice = User::new("u-alice", "Alice", 
    Some(PaymentMethod::card("**** **** **** 4242", "Alice"))
);

let bob = User::new("u-bob", "Bob", None);  // No default payment

let product1 = Product::new("p-1", "Email Support", 500);
let product2 = Product::new("p-2", "Premium Analytics", 1500);

let service = Service::new("s-1", "SaaS Platform", vec![product1, product2]);
```

### Example 2: Query Services and Products

```rust
use src02::catalog::Catalog;

let mut catalog = Catalog::default();
catalog = catalog.with_service(service);

// Get all products
let all_products = catalog.list_all_products();
println!("Total products: {}", all_products.len());

// Get products for a specific service
if let Some(products) = catalog.list_products_for_service(&"s-1".into()) {
    for p in products {
        println!("- {} ({}¢)", p.name, p.price_cents);
    }
}
```

### Example 3: Track Service Usage and Resolve Payments

```rust
use src02::usage::{UsageLog, resolve_payment_for_usage};

let usage1 = ServiceUsage::new(&alice.id, &"s-1".into(), &"p-2".into(), None);
let log = UsageLog::from_vec(vec![usage1]);

// Resolve payment: use explicit payment or fallback to user's default
let resolved = resolve_payment_for_usage(&alice, None);
println!("Payment method: {:?}", resolved);  
// Output: Some(Card { card_number_masked: "****", holder: "Alice" })
```

### Example 4: Save and Retrieve from Database

```rust
use src02::persistence;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await?;
    persistence::init_db(&pool).await?;

    // Save user
    persistence::save_user(&pool, &alice).await?;

    // Retrieve users
    let users = persistence::get_users(&pool).await?;
    println!("Users: {:?}", users);

    Ok(())
}
```

### Example 5: Run with Makefile

```bash
# View all commands
make help

# Build + Test + Run Demo
make build && make test && make demo

# Create file DB
make run-file-create

# Clean everything
make clean
```

---

## Contributing

### Code Quality

Before committing:

```bash
# Format code
make fmt

# Check for warnings/style issues
make check

# Run tests
make test
```

### Adding New Features

1. Define new types in `models.rs`
2. Implement query logic in appropriate module (catalog, usage, persistence)
3. Add tests in `tests/` directory
4. Ensure `make check` passes
5. Update README with examples

### Extending to PostgreSQL

To switch from SQLite to PostgreSQL:

1. Update `Cargo.toml` dependencies:
   ```toml
   sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls"] }
   ```

2. Update `src/bin/main.rs`:
   ```rust
   let pool = sqlx::PgPool::connect(&db_url).await?;
   ```

3. Update schema SQL (column types may differ)

4. Run with:
   ```bash
   cargo run --bin src02 -- --db-url="postgresql://user:pass@localhost/mydb"
   ```

---

## License

MIT License (or your chosen license)

---

## Support

For issues, questions, or contributions, please refer to the project repository or create an issue tracker.

---

**Happy coding! 🚀**
