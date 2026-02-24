## 1. Project Setup

- [x] 1.1 Add harper-core dependency to Cargo.toml
- [x] 1.2 Add ratatui dependency to Cargo.toml
- [x] 1.3 Add clipboard and terminal dependencies (arboard, crossterm)
- [x] 1.4 Verify empty shell compiles

## 2. TUI Editor

- [x] 2.1 Initialize Ratatui app with terminal setup
- [x] 2.2 Create main layout (editor, suggestions, status bar)
- [x] 2.3 Implement text input handling
- [x] 2.4 Add line numbers display
- [x] 2.5 Show cursor position in status bar

## 3. Grammar Checker

- [x] 3.1 Initialize harper-core with FstDictionary and PlainEnglish parser
- [x] 3.2 Implement 10ms debounce timer
- [x] 3.3 Connect text input to linting pipeline
- [x] 3.4 Display lint results with highlights
- [x] 3.5 Show suggestions panel

## 4. File Handler

- [x] 4.1 Add file open command (load from disk)
- [x] 4.2 Implement 10MB size check
- [x] 4.3 Implement auto-save on lint completion
- [x] 4.4 Show file name in status bar

## 5. Clipboard Paste

- [x] 5.1 Add arboard clipboard integration
- [x] 5.2 Handle Ctrl+Shift+V (Linux/Windows)
- [x] 5.3 Handle Cmd+Shift+V (macOS)
- [x] 5.4 Insert pasted text at cursor position

## 6. Dialect Selector

- [x] 6.1 Add dialect dropdown to UI
- [x] 6.2 Support US, UK, CA, AU, IN dialects
- [x] 6.3 Default to American English
- [x] 6.4 Re-lint on dialect change
