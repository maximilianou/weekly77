# src03_leptos - Full-Stack E-Commerce Platform

## ✅ Completed Milestones

### Frontend (Leptos 0.5 CSR + Trunk)
- **Status:** ✅ **RUNNING** on http://localhost:8080
- Leptos 0.5 CSR mode (stable-friendly, no nightly required)
- Trunk WASM bundler configured and serving locally
- Two complete CSS themes (700 lines each):
  - `css/style-default.css` — Minimalist, clean design
  - `css/style-modern.css` — Modern gradients and shadows
- **CSS Theme Switching:** JavaScript localStorage-based (no Rust code changes needed)
  - Test in browser console: `switchTheme('default')` or `switchTheme('modern')`
- Components implemented:
  - **Shop** page with 3 product cards + interactive cart counter
  - **AdminDashboard** with sidebar menu + dashboard cards
  - **UI components** (Card, Button, ButtonSuccess, ButtonDanger)
- Responsive design with mobile media queries

**How to verify:**
```bash
# Already running at http://localhost:8080
# Open DevTools → Console and run:
switchTheme('default')    # Switch to minimalist theme
switchTheme('modern')     # Switch to gradient theme
# Refresh page → theme persists (localStorage working)
```

### Backend (Axum + SQLx + SQLite)
- **Status:** ✅ **COMPILED** - ready to run
- Axum 0.8 web framework configured
- SQLite database with schema for:
  - `users` — seller/buyer accounts (username/password)
  - `products` — product listings
  - `product_images` — image uploads (max 6 per product, approval workflow: pending/approved/rejected)
  - `sales` — transaction records
- API endpoints implemented:
  - `GET /api/health` — server status
  - `POST /api/auth/seed` — create user with username → username01 password pattern
  - `GET /api/products` — list all products
  - `POST /api/uploads` — multipart image upload with processing
- Image processing utility:
  - Auto-downscale images >4MB to <1MB
  - Iterative JPEG quality reduction
  - Returns metadata (input size, output size, quality used)

**How to run backend:**
```bash
cd /home/dev01/projects/weekly77/app/src03_leptos/backend
cargo run
# Server listens on http://0.0.0.0:3001
```

### Build System
- **Workspace:** Cargo workspace with `resolver = "2"` (Edition 2021)
  - `backend/` — Axum server binary
  - `frontend/` — Leptos WASM frontend
- **Makefile targets:**
  ```bash
  make check           # cargo check all
  make build-backend   # cargo build --release backend
  make build-frontend  # trunk build --release frontend
  make run-backend     # cargo run --release backend
  make serve-frontend  # trunk serve frontend (already running on :8080)
  make docker-build    # Build Docker images
  make docker-up       # Start docker-compose services
  make clean           # Remove build artifacts
  ```
- **docker-compose.yml:** Services configured for postgres/backend/frontend orchestration (not yet tested)

---

## 🔄 Next Steps (Priority Order)

### 1. **Frontend-Backend API Integration** (Ready now)
   - Connect Shop component to `GET /api/products` endpoint
   - Connect Admin dashboard to product approval workflow
   - Cart state management (currently client-side signal-based)

### 2. **Backend Server Startup & Testing**
   ```bash
   cd backend && cargo run
   # Test endpoints with curl:
   curl http://localhost:3001/api/health
   curl -X POST http://localhost:3001/api/auth/seed -H "Content-Type: application/json" -d '{"username":"alice"}'
   ```

### 3. **Image Upload Approval Workflow** (Backend feature)
   - Implement `POST /api/products/{id}/images/approve` endpoint
   - Add seller approval confirmation for uploaded images
   - Persist processed images to filesystem

### 4. **Authentication & Session Management**
   - JWT tokens or session-based auth
   - Protect admin endpoints
   - Enforce max 6 images per product rule

### 5. **Docker End-to-End Deployment**
   ```bash
   make docker-build
   make docker-up
   # Verify: frontend on :8080, backend on :3001, postgres on :5432
   ```

---

## 📁 Project Structure

```
/home/dev01/projects/weekly77/app/src03_leptos/
├── Cargo.toml                    # Workspace manifest (resolver = "2")
├── Makefile                      # Build automation
├── README.md                     # This file
├── docker-compose.yml            # Multi-service orchestration
│
├── frontend/                     # Leptos WASM app (CSR mode)
│   ├── Cargo.toml
│   ├── Trunk.toml               # WASM bundler config
│   ├── index.html               # Entry point + theme switcher
│   ├── src/
│   │   ├── main.rs              # WASM mount point
│   │   ├── lib.rs               # App root component
│   │   ├── components/
│   │   │   ├── mod.rs
│   │   │   └── ui.rs            # Card, Button components
│   │   └── pages/
│   │       ├── mod.rs
│   │       ├── shop.rs          # Public shop with products
│   │       └── admin.rs         # Admin dashboard
│   └── css/
│       ├── style-default.css    # Minimalist theme (700 lines)
│       └── style-modern.css     # Modern gradient theme (700 lines)
│
├── backend/                      # Axum web server
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs              # Server entry point
│   │   ├── lib.rs               # Module exports
│   │   ├── api.rs               # HTTP endpoints
│   │   ├── db.rs                # SQLite models + schema init
│   │   └── images.rs            # Image processing (resize/compress)
│   ├── Dockerfile               # Production build
│   └── README.md                # Backend API reference
│
└── deployment/                   # Docker configs (placeholder)
```

---

## 🎨 CSS Theme Architecture

Both themes use **identical HTML structure** with different visual implementations:

```html
<div class="product-card">
  <div class="product-image">...</div>
  <div class="product-info">
    <div class="product-name">...</div>
    <div class="product-price">...</div>
  </div>
</div>
```

**Theme Switching Mechanism** (in `index.html`):
```javascript
function switchTheme(name) {
  localStorage.setItem('preferred-theme', name);
  const link = document.getElementById('theme-stylesheet');
  link.href = `css/style-${name}.css`;
}
// Auto-load on page load based on localStorage
```

**Key Differences:**
- **Default (Minimalist):** Light grays, simple borders, clear typography
- **Modern (Gradients):** Purple/pink gradients, shadows, smooth transitions, animation effects

**Benefit:** Style changes without touching Rust code. Add new theme by creating `css/style-xyz.css` with same class names.

---

## 🚀 Running Locally

### Prerequisites
```bash
rustup target add wasm32-unknown-unknown
cargo install trunk  # or: curl https://github.com/thedodd/trunk/releases/.../trunk-x86_64-unknown-linux-gnu.tar.gz | tar xz && sudo mv trunk /usr/local/bin
```

### Development (Terminal 1 - Frontend)
```bash
cd /home/dev01/projects/weekly77/app/src03_leptos/frontend
trunk serve
# Open http://localhost:8080 in browser
```

### Development (Terminal 2 - Backend)
```bash
cd /home/dev01/projects/weekly77/app/src03_leptos/backend
cargo run
# Server on http://localhost:3001
```

### Test API
```bash
# Health check
curl http://localhost:3001/api/health

# Create seed user
curl -X POST http://localhost:3001/api/auth/seed \
  -H "Content-Type: application/json" \
  -d '{"username":"seller01"}'
# Response: {"id":"...", "username":"seller01", "password":"seller0101"}

# List products
curl http://localhost:3001/api/products

# Upload image
curl -F "file=@test.jpg" http://localhost:3001/api/uploads
```

---

## 📊 Build Status

| Component | Status | Command |
|-----------|--------|---------|
| **Frontend (Cargo check)** | ✅ PASS | `cd frontend && cargo check` |
| **Frontend (Trunk serve)** | ✅ RUNNING | http://localhost:8080 |
| **Backend (Cargo check)** | ✅ PASS | `cd backend && cargo check` |
| **Backend (Runtime)** | ⏳ READY | `cd backend && cargo run` |
| **Docker** | 🔄 UNTESTED | `make docker-up` |

---

## 🎯 Key Achievements

1. **Frontend → CSS Separation:** Zero business logic in stylesheets; theme switching via JavaScript localStorage
2. **Dual-Theme Proof:** Two 700-line CSS files demonstrate identical UX with different designs
3. **Local Dev:** Trunk hot-reload enables rapid frontend iteration
4. **Backend Async:** Tokio + Axum + SQLx for production-grade async web server
5. **Image Processing:** Auto-resize/compress >4MB images to <1MB on server
6. **Database Schema:** Approval workflow ready (images pending/approved/rejected)
7. **Workspace Setup:** Unified Cargo workspace with shared resolver for 2021 edition

---

## 📝 Notes

- **SQLite file:** `src03_backend/src03_backend.db` (auto-created on first run)
- **WASM target:** `wasm32-unknown-unknown` required for Trunk
- **Stable Rust:** Leptos 0.5 CSR mode works on stable (no nightly)
- **CSS Themes:** 100% decoupled from Rust; modify CSS without recompiling
- **Leptos 0.5 API:** No `Scope` parameter; signals created with `create_signal(value)` directly
- **Axum Router:** Uses `State(pool)` extractor for database access

---

## 🔗 Resources

- [Leptos 0.5 Docs](https://docs.rs/leptos/0.5/)
- [Axum Web Framework](https://docs.rs/axum/0.8/)
- [SQLx Database Toolkit](https://docs.rs/sqlx/0.7/)
- [Trunk WASM Bundler](https://trunkrs.io/)

---

**Last Updated:** 2025-11-22  
**Project Location:** `/home/dev01/projects/weekly77/app/src03_leptos/`
