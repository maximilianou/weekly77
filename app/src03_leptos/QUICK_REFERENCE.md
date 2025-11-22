# Quick Reference Card - src03_leptos

## 🎯 Current Status: READY FOR DEVELOPMENT

### Frontend ✅
- **URL:** http://localhost:8080
- **Status:** Running (Trunk hot-reload)
- **Build:** `cd frontend && trunk serve`
- **CSS Themes:** `switchTheme('default')` or `switchTheme('modern')` in console

### Backend ✅
- **URL:** http://localhost:3001 (when running)
- **Status:** Compiled, ready to run
- **Build:** `cd backend && cargo run`
- **DB:** SQLite auto-initialized with schema + seed data

---

## 📋 Essential Commands

| Task | Command |
|------|---------|
| Start Frontend | `make serve-frontend` (already running) |
| Start Backend | `cd backend && cargo run` |
| Test API | `curl http://localhost:3001/api/health` |
| Build Release | `make build-backend && make build-frontend` |
| Check All | `make check` |
| Clean | `make clean` |

---

## 🧪 API Endpoints

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/api/health` | GET | Server status |
| `/api/auth/seed` | POST | Create user |
| `/api/products` | GET | List products |
| `/api/uploads` | POST | Upload image |

### Test API:
```bash
# Health
curl http://localhost:3001/api/health

# Create user (username → username01 password)
curl -X POST http://localhost:3001/api/auth/seed \
  -H "Content-Type: application/json" \
  -d '{"username":"seller01"}'

# Get products
curl http://localhost:3001/api/products

# Upload image
curl -F "file=@image.jpg" http://localhost:3001/api/uploads
```

---

## 🎨 CSS Themes

**Current Themes:**
- `css/style-default.css` — Minimalist (light, clean)
- `css/style-modern.css` — Modern (gradients, shadows)

**Switch Themes (Browser Console):**
```javascript
switchTheme('default')   // Minimalist
switchTheme('modern')    // Gradients
localStorage.getItem('preferred-theme')  // Check saved
```

**Add New Theme:**
1. Create `css/style-newname.css` with same class structure
2. Add to `switchTheme()` options in `index.html`
3. No Rust code changes needed!

---

## 📂 Project Layout

```
src03_leptos/
├── frontend/              # Leptos 0.5 WASM app
│   ├── css/              # Themes (2x 700 lines)
│   ├── src/
│   │   ├── main.rs       # WASM mount
│   │   ├── lib.rs        # App root
│   │   ├── pages/
│   │   │   └── shop.rs   # Products + cart
│   │   └── components/
│   │       └── ui.rs     # Card, Button
│   └── Trunk.toml        # WASM config
│
├── backend/               # Axum web server
│   └── src/
│       ├── main.rs       # Server + DB init
│       ├── api.rs        # Routes
│       ├── db.rs         # Models + schema
│       └── images.rs     # Resize/compress
│
├── Makefile              # Build automation
├── docker-compose.yml    # Multi-service
├── PROJECT_STATUS.md     # Full docs
└── IMPLEMENTATION_SUMMARY.md
```

---

## 🔗 File Changes

### Last Session:
1. ✅ Fixed Leptos 0.5 CSR API (Scope removal, signal access)
2. ✅ Fixed Trunk configuration
3. ✅ Created SQLite schema (users, products, images, sales)
4. ✅ Wired API endpoints to database
5. ✅ Frontend live on http://localhost:8080
6. ✅ Backend compiled and ready

### Next Session:
1. Start backend: `cargo run` from `backend/` directory
2. Test API endpoints with curl
3. Connect frontend components to backend APIs
4. Implement image upload/approval workflow

---

## 🚦 Troubleshooting

### Frontend not loading?
```bash
# Check trunk is running
lsof -i :8080
# Restart if needed
pkill trunk
cd frontend && trunk serve
```

### Backend won't compile?
```bash
cd backend
cargo clean
cargo check
```

### API endpoint 404?
```bash
# Check backend is running
curl http://localhost:3001/api/health
# If not, start it: cargo run
```

### Database issues?
```bash
# SQLite file location: backend/src03_backend.db
# Delete to reset: rm backend/src03_backend.db
# Then run: cargo run (auto-recreates schema)
```

---

## 📊 Architecture

```
User Browser (http://localhost:8080)
    ↓
Frontend (Leptos WASM) — Hot-reload via Trunk
    ↓
API Calls (http://localhost:3001)
    ↓
Backend (Axum + Tokio)
    ↓
Database (SQLite)
```

**Data Flow Example:**
1. User clicks "Add to Cart" on shop page
2. Frontend component updates signal (reactive)
3. Cart counter displays live update
4. On checkout → sends to `/api/uploads` or similar

---

## 🎓 Learning Resources

- **Leptos 0.5:** https://docs.rs/leptos/0.5/
- **Axum:** https://docs.rs/axum/0.8/
- **SQLx:** https://docs.rs/sqlx/0.7/
- **Trunk:** https://trunkrs.io/

---

## 💡 Tips

- **Hot Reload:** Edit CSS/HTML/Rust in frontend → instantly see changes
- **DB Reset:** Delete `backend/src03_backend.db` and restart to reinit schema
- **Parallel Dev:** Run `trunk serve` and `cargo run` in separate terminals
- **Theme Testing:** Use browser DevTools to test both themes side-by-side
- **API Debugging:** Use `curl` or Postman to test endpoints before integrating

---

**Last Updated:** 2025-11-22  
**Project Root:** `/home/dev01/projects/weekly77/app/src03_leptos/`  
**Frontend URL:** http://localhost:8080 ✅  
**Backend Ready:** Run `cargo run` in backend directory
