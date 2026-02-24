## ADDED Requirements

### Requirement: Clipboard Paste
The application SHALL support pasting from clipboard using Ctrl/Cmd+Shift+V.

#### Scenario: Paste from clipboard
- **WHEN** user presses Ctrl+Shift+V (Linux/Windows) or Cmd+Shift+V (macOS)
- **THEN** clipboard content SHALL be inserted at cursor position and checked

#### Scenario: Empty clipboard
- **WHEN** user pastes with empty clipboard
- **THEN** nothing SHALL happen
