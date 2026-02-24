## Why

Build a quick, offline grammar checker as a TUI application using harper-core and Ratatui. Lightweight (<50MB), fast (milliseconds), and privacy-first - no network calls needed.

## What Changes

- New TUI application using Ratatui
- Real-time grammar checking with 10ms debounce
- File support (load and auto-save corrections)
- Clipboard paste support via Ctrl/Cmd+Shift+V
- Support files up to 10MB
- English dialect selection (American, British, Canadian, Australian, Indian)

## Capabilities

### New Capabilities
- `tui-editor`: Full editor view with Ratatui - line numbers, syntax highlighting
- `grammar-checker`: Real-time linting using harper-core with 10ms debounce
- `file-handler`: Load and auto-save files up to 10MB
- `clipboard-paste`: Ctrl/Cmd+Shift+V to paste and check
- `dialect-selector`: Choose English dialect from dropdown

### Modified Capabilities
None - this is a new project.

## Impact

- New Rust crate `rs-grammar`
- Dependencies: harper-core, ratatui, clipboard integration
- Single binary TUI application
- No external services or network calls
