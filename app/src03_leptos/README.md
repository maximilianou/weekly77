# src03_leptos — Rust full-stack (Leptos frontend + Axum backend)

Questo progetto è uno scaffold completo di e-commerce: frontend Leptos (CSS themes separati e switchabili), backend Axum, persistenza con approval workflow per immagini.

## Setup Locale (Sviluppo)

### 1. Prerequisiti Globali

```bash
# Install Rust (if not present)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk (WASM build tool)
cargo install trunk
```

### 2. Frontend — Dev Server (Leptos + Trunk)

```bash
cd frontend
trunk serve
```

- Accedi a `http://localhost:8080`
- Hot-reload automatico su file changes
- CSS theme switcher nella navbar (test con Default e Modern)
- Due temi dimostrano separazione CSS puro da logica Rust

### 3. Backend — Dev Server (Axum)

```bash
cd backend
cargo run
```

- API disponibile su `http://localhost:3001`
- `/api/health` — health check

## Struttura Principale

- `frontend/` — Leptos SPA (CSR: Client-Side Rendering)
  - `css/` — Due temi completamente separati (style-default.css, style-modern.css)
  - `src/components/` — UI components (Card, Button, AdminDashboard)
  - `src/pages/` — Pages (Shop, Admin)
  - `Trunk.toml` — WASM build config
  - `index.html` — Theme switcher script (CSS-only, localStorage persisted)

- `backend/` — Axum web server
  - `src/api.rs` — HTTP endpoints (scaffold)
  - `src/images.rs` — Image processing utility (resize >4MB to <1MB)
  - `Cargo.toml` — Dependencies

## CSS Themes Feature

**Completamente CSS-based, senza logica di business impattata**:

1. **Default Theme** (`style-default.css`) — Minimalista, pulito
2. **Modern Theme** (`style-modern.css`) — Gradients, effetti, contemporaneo

Cambia tema via:
- Browser console: `switchTheme('default')` o `switchTheme('modern')`
- Pulsanti nella navbar
- Salvato in localStorage (persiste al refresh)

Aggiungere un nuovo tema:
- Crea `frontend/css/style-mytheme.css` con le stesse classi CSS
- Usa `switchTheme('mytheme')` per attivarlo

## Roadmap (Ordine Priorità)

1. ✅ Frontend buildable con trunk + 2 temi CSS
2. ⏳ Backend persistence (SQLx, Postgres/SQLite) + approval workflow per immagini
3. ⏳ Integrazione API frontend-backend
4. ⏳ Docker + docker-compose + Makefile end-to-end deploy

## Comandi Makefile

```bash
make help              # Show all targets
make check             # cargo check backend & frontend
make build-backend     # Build backend release
make docker-build      # Build docker images
make docker-up         # Run docker-compose up
```

## Note Tecniche

- **Frontend**: Leptos 0.5 (stable-friendly), Trunk, WASM (CSR)
- **Backend**: Axum 0.8, Tokio, SQLx (scaffold; DB non implementato ancora)
- **Database**: Postgres (docker-compose) / SQLite (local)
- **Themes**: Pure CSS, nessuno inline style nel Rust
- **Responsive**: Entrambi i temi supportano mobile

## Prossimo Step

1. Build frontend release: `cd frontend && trunk build --release`
2. Verificare CSS switching nei due temi
3. Implementare backend persistence + approval workflow
4. Connettere API frontend-backend
5. Dockerizzare e deploy

