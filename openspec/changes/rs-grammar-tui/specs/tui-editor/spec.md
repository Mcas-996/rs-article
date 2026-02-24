## ADDED Requirements

### Requirement: TUI Editor View
The application SHALL display a full-screen terminal editor with line numbers, text area, and status bar using Ratatui.

#### Scenario: Display editor
- **WHEN** the application starts
- **THEN** a full-screen TUI editor SHALL be displayed with a text area and status bar

#### Scenario: Show line numbers
- **WHEN** text is displayed in the editor
- **THEN** each line SHALL show a line number in the gutter

#### Scenario: Status bar information
- **WHEN** the editor is active
- **THEN** the status bar SHALL show current file name, cursor position, and error count
