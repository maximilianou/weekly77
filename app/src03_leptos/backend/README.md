# Backend — src03_backend

Questo backend fornisce le API per il progetto `src03_leptos`.

Endpoints principali (scaffold):

- `POST /api/auth/seed` — crea utente di seed (username/password = username + "01")
- `POST /api/login` — autenticazione (placeholder)
- `GET /api/products` — lista prodotti
- `POST /api/products` — crea / aggiorna prodotto (supporta upload immagini multipart)
- `POST /api/uploads` — endpoint per upload immagini; applica elaborazione se necessario

Immagini vengono salvate in `uploads/` (server). Se l'immagine superano 4MB verrà ridimensionata/ricompressa per ottenere <1MB.

Per avviare (da cartella `backend`):

```
cargo run
```
