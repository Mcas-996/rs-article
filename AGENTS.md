# AGENTS.md - Agent Coding Guidelines for rs-grammar

## Project Overview

`rs-grammar` is a grammar/linting tool built with Rust and Harper-core. It supports two build targets:
- **Native**: TUI application using ratatui, crossterm, and arboard
- **Web**: WebAssembly application for browser environments

## Build Commands

### Standard Rust Commands

```bash
# Build the native version (default)
cargo build

# Build the web (WASM) version
cargo build --features web

# Check code without building
cargo check
cargo check --features web

# Release build with optimizations
cargo build --release

# Run the application (native only)
cargo run

# Run with specific features
cargo run --features native
```

### Running Tests

This project currently has no test files. If tests are added:

```bash
# Run all tests
cargo test

# Run a single test by name
cargo test test_name_here

# Run tests with output
cargo test -- --nocapture

# Run doc tests
cargo test --doc
```

### Linting and Formatting

```bash
# Run clippy for linting
cargo clippy

# Fix auto-fixable clippy warnings
cargo clippy --fix --allow-dirty

# Format code
cargo fmt

# Check formatting
cargo fmt --check
```

### WASM/Web Build

```bash
# Build WASM (requires wasm-pack or wasm-bindgen-cli)
wasm-pack build --target web

# Or using wasm-bindgen directly
cargo build --features web --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./pkg --target web ./target/wasm32-unknown-unknown/rs_grammar.wasm
```

## Code Style Guidelines

### General Conventions

- Use **Rust 2024 edition** (as specified in Cargo.toml)
- Follow standard Rust idioms and patterns
- Use `#[cfg(feature = "...")]` for feature-gated code (see src/lib.rs:1-6)
- Feature names: `native` (default), `web`

### Imports and Module Organization

Group imports in this order (as seen in src/main.rs):
1. Standard library imports (`std::`)
2. External crate imports
3. In-project module imports

```rust
// Example import organization
use std::io::{self, Write};
use std::sync::Arc;

use harper_core::{Dictionary, Document, LintSet, Linter};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
```

### Naming Conventions

- **Variables and functions**: `snake_case` (e.g., `run_linter`, `cursor_pos`)
- **Types and structs**: `PascalCase` (e.g., `App`, `LintResult`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_FILE_SIZE`)
- **Private fields**: prefix with underscore (e.g., `_internal_field`)

### Error Handling

Use the `map_err` pattern for error conversion:

```rust
// Preferred pattern (seen throughout src/main.rs)
std::fs::read_to_string(path).map_err(|e| e.to_string())?

// Or with custom error messages
std::fs::metadata(path).map_err(|e| e.to_string())?;

// Return Result<T, String> for user-facing errors
fn load_file(&mut self, path: &str) -> Result<(), String> { ... }
```

### Struct and Type Definitions

Use `#[derive(...)]` for common traits:

```rust
#[derive(Clone, Debug)]
struct LintResult {
    line: usize,
    start_col: usize,
    end_col: usize,
    message: String,
}
```

### Concurrency

Use `Arc` and `AtomicBool` for shared state with thread-safe triggers:

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

lint_triggered: Arc<AtomicBool>,
```

When using atomics, prefer `Ordering::SeqCst` for simplicity (as used in this codebase).

### Feature Gating

Feature-gated code should be clearly organized:

```rust
#[cfg(feature = "web")]
mod web;

#[cfg(feature = "native")]
pub mod native {
    include!("main.rs");
}
```

### WASM-Specific Guidelines

When writing WebAssembly code:
- Use `wasm-bindgen` for JS interoperability
- Set up panic hook for better error messages
- Avoid blocking operations

```rust
use console_error_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn wasm_main() {
    console_error_panic_hook::set_once();
    // ... code
}
```

### UI/TUI Guidelines

When using ratatui:
- Use `Constraint` for flexible layouts
- Follow the Block/Borders/Paragraph pattern
- Use consistent styling with `Style::default()`

## Cargo Features

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `native` (default) | TUI application | ratatui, crossterm, arboard |
| `web` | WASM browser version | wasm-bindgen, web-sys, js-sys |

## CI/CD

The project uses GitHub Actions (`.github/workflows/rust.yml`):
- Runs on Ubuntu latest
- Executes `cargo check` on push to main and PRs

## Key Dependencies

- `harper-core`: Grammar checking engine
- `ratatui`: Terminal UI framework
- `crossterm`: Terminal manipulation
- `arboard`: Clipboard access
- `wasm-bindgen`: WASM JS interop

## File Structure

```
src/
├── lib.rs      # Crate root, feature gates
├── main.rs     # Native TUI application
└── web.rs      # WASM entry point
```

## Important Notes

- Maximum file size for loading: 10MB (see src/main.rs:77)
- Uses standard atomic ordering (`Ordering::SeqCst`)
- Clipboard paste supports both Ctrl+Shift+V and Super+Shift+V
- Linting runs with 10ms debounce after changes
