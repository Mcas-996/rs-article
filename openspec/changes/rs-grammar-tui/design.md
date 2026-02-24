## Context

Build a TUI grammar checker using harper-core and Ratatui. This is a new standalone application - no existing codebase to integrate with.

Constraints:
- Offline-first: no network calls
- Lightweight: <50MB memory
- Fast: <10ms per lint cycle
- Max file size: 10MB
- Must support English dialects: US, UK, CA, AU, IN

## Goals / Non-Goals

**Goals:**
- Real-time grammar checking as user types
- Full editor view with line numbers
- Load and auto-save files
- Clipboard paste support
- Dialect selection

**Non-Goals:**
- Plugin system (explicitly not needed)
- Syntax highlighting (keep simple first)
- Multiple languages (English only)
- Network/cloud features

## Decisions

### 1. TUI Framework: Ratatui over cursive
- **Choice**: Ratatui
- **Rationale**: More active maintenance, better async support, crossterm-native
- **Alternative**: cursive (older, less active)

### 2. Architecture: Event-driven with debounce
- **Choice**: Input → 10ms debounce → harper-core lint → UI update
- **Rationale**: Simple pipeline, avoids blocking UI thread
- **Alternative**: Background thread (overkill for this use case)

### 3. Editor: Custom buffer over embedded
- **Choice**: Custom text buffer with Ratatui widgets
- **Rationale**: Full control over real-time lint integration
- **Alternative**: termwiz, tealdeer (more complex)

### 4. File loading: Load into memory
- **Choice**: Load entire file into memory (up to 10MB)
- **Rationale**: Simpler, harper-core handles this size easily
- **Alternative**: Memory-mapped files (unnecessary complexity)

### 5. Auto-save: Immediate write on lint completion
- **Choice**: Write file after each lint cycle when content changes
- **Rationale**: Simple, ensures no data loss
- **Alternative**: Periodic save, manual save (rejected - user wanted auto)

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| 10ms too aggressive for harper | Profile first, increase if needed |
| Blocking UI on large files | 10MB limit, warn user |
| Auto-save overwrites formatting | Save raw, rely on user to manage |

## Open Questions

- Should we add undo/redo? (Not in initial scope)
- Error recovery on corrupted files? (Basic error handling first)
