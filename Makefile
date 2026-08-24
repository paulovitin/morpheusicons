.PHONY: help build check test clean run run-gpui run-egui run-iced run-cli run-rust-ui run-web wasm css site gpui egui iced cli rust-ui web watch-web

# Handle positional arguments for "make run <framework>" (e.g. make run gpui)
ifeq ($(firstword $(MAKECMDGOALS)),run)
  RUN_TARGET := $(word 2,$(MAKECMDGOALS))
  # Ignore remaining goals so make doesn't throw "No rule to make target" error
  $(eval $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS)):;@:)
endif

## help: Display available Makefile commands
help:
	@echo "MorpheusIcons Makefile Commands:"
	@echo ""
	@echo "  make run gpui        Run GPUI morphing demo (alias: make run-gpui, make gpui)"
	@echo "  make run egui        Run egui morphing demo (alias: make run-egui, make egui)"
	@echo "  make run iced        Run Iced morphing demo (alias: make run-iced, make iced)"
	@echo "  make run cli         Run CLI morphing demo (alias: make run-cli, make cli)"
	@echo "  make run rust-ui     Run rust-ui / Leptos integration demo (alias: make run-rust-ui)"
	@echo "  make run web         Run web browser showcase with watch & live reload (alias: make web, make watch-web)"
	@echo "  make wasm            Build WASM module only (pkg/)"
	@echo "  make css             Build Tailwind CSS only (dist/output.css)"
	@echo ""
	@echo "  make test            Run all unit and integration tests"
	@echo "  make check           Check code compilation for all features"
	@echo "  make build           Build the library crate"
	@echo "  make clean           Clean cargo target directory"

## run: Run an example based on argument (gpui, egui, iced, cli, rust-ui, web)
run:
	@if [ "$(RUN_TARGET)" = "gpui" ]; then \
		echo "🚀 Running GPUI morphing demo..."; \
		cargo run --example gpui_morph_demo --features gpui; \
	elif [ "$(RUN_TARGET)" = "egui" ]; then \
		echo "🚀 Running egui morphing demo..."; \
		cargo run --example egui_morph_demo --features egui; \
	elif [ "$(RUN_TARGET)" = "iced" ]; then \
		echo "🚀 Running Iced morphing demo..."; \
		cargo run --example iced_morph_demo --features iced; \
	elif [ "$(RUN_TARGET)" = "cli" ]; then \
		echo "🚀 Running CLI morphing demo..."; \
		cargo run --example cli_morph; \
	elif [ "$(RUN_TARGET)" = "rust-ui" ] || [ "$(RUN_TARGET)" = "rust_ui" ] || [ "$(RUN_TARGET)" = "leptos" ]; then \
		echo "🚀 Running rust-ui / Leptos integration demo..."; \
		cargo run --example rust_ui_integration --features egui,leptos; \
	elif [ "$(RUN_TARGET)" = "web" ] || [ "$(RUN_TARGET)" = "browser" ] || [ "$(RUN_TARGET)" = "watch-web" ]; then \
		$(MAKE) run-web; \
	elif [ -z "$(RUN_TARGET)" ]; then \
		echo "❌ Usage: make run [gpui|egui|iced|cli|rust-ui|web]"; \
		echo "Or use direct commands: make run-gpui, make run-egui, make run-iced, make run-cli, make run-rust-ui, make web"; \
		exit 1; \
	else \
		echo "❌ Unknown run target: '$(RUN_TARGET)'"; \
		echo "Available run targets: gpui, egui, iced, cli, rust-ui, web"; \
		exit 1; \
	fi

## Direct targets
run-gpui:
	@cargo run --example gpui_morph_demo --features gpui

run-egui:
	@cargo run --example egui_morph_demo --features egui

run-iced:
	@cargo run --example iced_morph_demo --features iced

run-cli:
	@cargo run --example cli_morph

run-rust-ui:
	@cargo run --example rust_ui_integration --features egui,leptos

run-web: wasm css
	@echo "🌐 Serving MorpheusIcons at http://localhost:8765"
	@echo "👀 Watch mode active (watching Rust, CSS, and HTML with Live Reload)..."
	@echo "   Press Ctrl+C to stop.\n"
	@bash -c '\
		trap "kill \$$(jobs -p) 2>/dev/null" EXIT INT TERM; \
		if command -v cargo-watch >/dev/null 2>&1 || cargo watch --version >/dev/null 2>&1; then \
			cargo watch -w src -w Cargo.toml -i pkg -i dist -i target -s "make wasm" & \
		else \
			echo "⚠️  cargo-watch not found. Install it with: cargo install cargo-watch"; \
		fi; \
		npm run watch:css & \
		cargo run --example web_server \
	'

## wasm: Build WASM module with wasm-pack
wasm:
	@echo "🦀 Building WASM module..."
	@wasm-pack build --target web --no-default-features --features std,catalog,wasm
	@echo "✅ WASM build complete (pkg/)"

## css: Build Tailwind CSS for production
css:
	@echo "🎨 Building production CSS..."
	@npm run build:css
	@echo "✅ CSS build complete (dist/output.css)"

## assemble: Copy pages/ + built assets into _site/ (no WASM/CSS rebuild)
assemble:
	@echo "📦 Assembling static website in _site/..."
	@rm -rf _site
	@mkdir -p _site
	@cp pages/*.html _site/
	@cp pages/*.js _site/
	@cp pages/llms.txt _site/
	@cp -r assets _site/
	@cp -r dist _site/
	@cp -r pkg _site/
	@touch _site/.nojekyll
	@echo "✅ Site assembled (_site/)"

## site: Build WASM + CSS + assemble static site directory in _site/
site: wasm css assemble
	@echo "✅ Site build complete (_site/)"


gpui: run-gpui
egui: run-egui
iced: run-iced
cli: run-cli
rust-ui: run-rust-ui
web: run-web
watch-web: run-web

## test: Run unit & integration tests
test:
	@cargo test --all-targets --all-features

## check: Verify compilation across all features
check:
	@cargo check --all-features

## build: Build the crate
build:
	@cargo build --all-features

## clean: Clean target build directory
clean:
	@cargo clean
