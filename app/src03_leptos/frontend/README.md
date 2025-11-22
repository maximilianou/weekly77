# Frontend — Leptos + Trunk

Questo è il frontend della piattaforma e-commerce costruito con Leptos (Rust framework web) e compilato a WebAssembly tramite Trunk.

## Prerequisiti

- Rust 1.70+
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- `trunk`: `cargo install trunk`

## Struttura

- `css/` — Fogli di stile separati:
  - `style-default.css` — Tema minimalista (pulito e semplice)
  - `style-modern.css` — Tema moderno (gradients, effetti, contemporaneo)
- `src/` — Codice Leptos:
  - `lib.rs` — App root component
  - `main.rs` — Entry point per build WASM
  - `components/` — Componenti UI riutilizzabili (Card, Button, Admin Dashboard)
  - `pages/` — Pagine (Shop, Admin)
- `index.html` — Template HTML con CSS theme switcher
- `Trunk.toml` — Configurazione Trunk

## CSS Themes

**Il CSS è completamente separato dalla logica di business e UI**. Cambiare tema è semplicissimo:

1. **Da console browser**:
   ```javascript
   switchTheme('default')  // Cambia a tema minimalista
   switchTheme('modern')   // Cambia a tema moderno
   ```

2. **Via pulsanti** (inclusi nell'app nella navbar)

3. **Persistenza**: Il tema scelto viene salvato in `localStorage`

## Run (Dev)

```bash
cd frontend
trunk serve
```

Questo avvia un server dev a `http://localhost:8080` con hot-reload.

## Build (Production)

```bash
cd frontend
trunk build --release
```

Output: `dist/` (serve con nginx o static server)

## Como Estendere CSS

- **Aggiungere nuovo tema**: Crea `css/style-mytheme.css` con le stesse classi (`.card`, `.btn`, `.product-card`, ecc.)
- **Modificare stile**: Edita i `.css` direttamente — il codice Rust usa solo classi CSS standard, nessun inline style di business logic
- **Responsive**: Entrambi i temi supportano media queries per mobile

## Componenti Disponibili

- `Card` — Container generico con titolo
- `Button`, `ButtonSuccess`, `ButtonDanger` — Pulsanti stilizzati
- `AdminDashboard` — Pannello admin con sidebar
- `Shop` — Pagina shop con prodotti demo e carrello

Ogni componente usa classi CSS; il tema è applicato globalmente via CSS.

## Note

- Questo è uno scaffold. Pronto per estensione business logic
- Nessuno stile inline nel Rust — tutto CSS
- Due temi dimostrano la potenza del CSS puro e della separazione concerns

