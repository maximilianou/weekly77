#!/bin/bash
# Quick Start: src03_leptos E-Commerce Platform

PROJECT_DIR="/home/dev01/projects/weekly77/app/src03_leptos"
cd "$PROJECT_DIR" || exit 1

echo "🚀 src03_leptos Quick Start Guide"
echo "=================================="
echo ""

# Check prerequisites
echo "📦 Checking prerequisites..."
if ! command -v rustup &> /dev/null; then
    echo "❌ rustup not found. Install from https://rustup.rs"
    exit 1
fi

if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "📥 Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

if ! command -v trunk &> /dev/null; then
    echo "📥 Installing trunk WASM bundler..."
    curl -L https://github.com/thedodd/trunk/releases/download/v0.19.1/trunk-x86_64-unknown-linux-gnu.tar.gz \
        -o /tmp/trunk.tar.gz && \
    tar -xz -C /tmp -f /tmp/trunk.tar.gz && \
    sudo mv /tmp/trunk /usr/local/bin/
fi

echo "✅ Prerequisites satisfied"
echo ""

# Display quick commands
echo "📋 Quick Commands:"
echo ""
echo "  Frontend (Leptos + Trunk):"
echo "    make serve-frontend     # ✅ Already running on http://localhost:8080"
echo ""
echo "  Backend (Axum + SQLx):"
echo "    make run-backend        # Start server on http://localhost:3001"
echo ""
echo "  Testing:"
echo "    curl http://localhost:3001/api/health"
echo ""
echo "  CSS Theme Switching (Browser DevTools Console):"
echo "    switchTheme('default')  # Minimalist theme"
echo "    switchTheme('modern')   # Modern gradient theme"
echo ""
echo "  Full Build:"
echo "    make build-backend      # cargo build --release backend"
echo "    make build-frontend     # trunk build --release frontend"
echo ""
echo "  Docker:"
echo "    make docker-build       # Build images"
echo "    make docker-up          # Start services"
echo ""
echo "=========================================="
echo ""
echo "✅ Frontend already running at http://localhost:8080"
echo ""
echo "To start backend in another terminal:"
echo "  cd $PROJECT_DIR"
echo "  make run-backend"
echo ""
echo "📄 Full documentation: PROJECT_STATUS.md"
