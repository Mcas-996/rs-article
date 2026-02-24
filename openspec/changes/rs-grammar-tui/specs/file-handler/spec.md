## ADDED Requirements

### Requirement: File Loading
The application SHALL load text files from disk into the editor.

#### Scenario: Load small file
- **WHEN** user opens a file <= 10MB
- **THEN** the file content SHALL be loaded into the editor

#### Scenario: Reject large file
- **WHEN** user opens a file > 10MB
- **THEN** an error message SHALL be displayed and file SHALL not be loaded
