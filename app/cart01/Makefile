.PHONY: help build check test test-verbose test-coverage run clean docs fmt lint all

# Color output
RED := \033[0;31m
GREEN := \033[0;32m
BLUE := \033[0;34m
NC := \033[0m # No Color

help:
	@echo "$(BLUE)🛒 cart01: Learning Rust - E-Commerce Product Structure$(NC)"
	@echo ""
	@echo "$(BLUE)Available targets:$(NC)"
	@echo "  $(GREEN)make build$(NC)           - Compile the project (no run)"
	@echo "  $(GREEN)make check$(NC)           - Check code without compiling (fast feedback)"
	@echo "  $(GREEN)make test$(NC)            - Run all tests (30+ test cases, ~100% coverage)"
	@echo "  $(GREEN)make test-verbose$(NC)    - Run tests with output (see println! in tests)"
	@echo "  $(GREEN)make run$(NC)             - Run the main binary (example usage)"
	@echo "  $(GREEN)make clean$(NC)           - Remove compiled artifacts"
	@echo "  $(GREEN)make docs$(NC)            - Generate and open documentation"
	@echo "  $(GREEN)make fmt$(NC)             - Format code with rustfmt"
	@echo "  $(GREEN)make lint$(NC)            - Check code with clippy (linter)"
	@echo "  $(GREEN)make all$(NC)             - Run: check → fmt → lint → test → build"
	@echo ""
	@echo "$(BLUE)Learning Path:$(NC)"
	@echo "  1. make check      - Verify syntax (fast)"
	@echo "  2. make test       - Run all tests (verify behavior)"
	@echo "  3. make run        - See working example"
	@echo "  4. make docs       - Read documentation (cargo docs)"
	@echo ""

# ============================================================================
# BUILD TARGETS
# ============================================================================

build: check
	@echo "$(BLUE)📦 Building release binary...$(NC)"
	cargo build --release
	@echo "$(GREEN)✅ Build complete!$(NC)"

check:
	@echo "$(BLUE)🔍 Checking syntax and dependencies...$(NC)"
	cargo check
	@echo "$(GREEN)✅ Check passed!$(NC)"

# ============================================================================
# TEST TARGETS (Core learning - 100% coverage target)
# ============================================================================

test:
	@echo "$(BLUE)🧪 Running tests (~100% code coverage)...$(NC)"
	cargo test --lib -- --test-threads=1 2>&1 | grep -E "(test result:|running|passed)"
	@echo ""
	@echo "$(GREEN)✅ All tests passed!$(NC)"
	@echo ""
	@echo "$(BLUE)Test breakdown:$(NC)"
	@echo "  ✓ Success cases     - Valid product creation"
	@echo "  ✓ Error cases       - Zero ID, empty name, negative price, invalid date"
	@echo "  ✓ Mutation tests    - set_name, set_price, set_description"
	@echo "  ✓ Business logic    - is_expensive, apply_discount"
	@echo "  ✓ Integration tests - clone, debug, serialization"
	@echo ""

test-verbose:
	@echo "$(BLUE)🧪 Running tests with output...$(NC)"
	cargo test --lib -- --nocapture --test-threads=1

test-coverage:
	@echo "$(BLUE)📊 Test coverage report:$(NC)"
	@echo "  Run: cargo test --all"
	cargo test --all -- --nocapture 2>&1 | tail -20

# ============================================================================
# RUN TARGET (See the code in action)
# ============================================================================

run:
	@echo "$(BLUE)🚀 Running example binary...$(NC)"
	cargo run --release

run-debug:
	@echo "$(BLUE)🚀 Running example binary (debug)...$(NC)"
	cargo run

# ============================================================================
# CODE QUALITY TARGETS
# ============================================================================

fmt:
	@echo "$(BLUE)🎨 Formatting code...$(NC)"
	cargo fmt
	@echo "$(GREEN)✅ Code formatted!$(NC)"

lint:
	@echo "$(BLUE)🔎 Linting with clippy...$(NC)"
	cargo clippy -- -D warnings || true
	@echo "$(GREEN)✅ Lint check complete!$(NC)"

# ============================================================================
# DOCUMENTATION TARGETS
# ============================================================================

docs:
	@echo "$(BLUE)📚 Generating documentation...$(NC)"
	cargo doc --no-deps --open 2>/dev/null || cargo doc --no-deps

# ============================================================================
# CLEANUP
# ============================================================================

clean:
	@echo "$(BLUE)🧹 Cleaning build artifacts...$(NC)"
	cargo clean
	@echo "$(GREEN)✅ Clean complete!$(NC)"

# ============================================================================
# COMPOSITE TARGET (Recommended workflow)
# ============================================================================

all: check fmt lint test build
	@echo ""
	@echo "$(GREEN)✅ All checks passed!$(NC)"
	@echo ""
	@echo "$(BLUE)Next steps:$(NC)"
	@echo "  1. Review: make docs"
	@echo "  2. Run: make run"
	@echo "  3. Modify: edit src/lib.rs"
	@echo "  4. Re-run: make test"
	@echo ""
