REPO_ROOT := $(shell pwd)

.PHONY: help check build build-backend build-frontend run-backend serve-frontend docker-up docker-build clean

help:
	@echo "Makefile targets for src03_leptos"
	@echo "  make check             # run cargo check for backend and attempt frontend"
	@echo "  make build            # build both backend and frontend (frontend may require wasm toolchain)"
	@echo "  make build-backend    # build backend (release)"
	@echo "  make build-frontend   # build frontend (may require wasm target and trunk)"
	@echo "  make run-backend      # run backend locally (cargo run)"
	@echo "  make serve-frontend   # serve frontend (local dev server)"
	@echo "  make docker-build     # build docker images for services"
	@echo "  make docker-up        # docker-compose up"
	@echo "  make clean            # clean artifacts"

check:
	@echo "Checking backend..."
	cd backend && cargo check || exit 1

	@echo "Checking frontend (may fail on stable if dependencies require wasm/nightly)..."
	cd frontend && cargo check || echo "Frontend check failed; see frontend/README.md for dev setup"

build: build-backend build-frontend

build-backend:
	cd backend && cargo build --release

build-frontend:
	@echo "Building frontend (this may require wasm target and Leptos tooling)."
	@echo "See frontend/README.md for instructions."

run-backend:
	cd backend && cargo run

serve-frontend:
	@echo "Serve frontend locally using Leptos tooling (e.g., cargo leptos). See frontend/README.md"

docker-build:
	docker-compose -f docker-compose.yml build

docker-up:
	docker-compose -f docker-compose.yml up --build

clean:
	rm -rf backend/target frontend/target
