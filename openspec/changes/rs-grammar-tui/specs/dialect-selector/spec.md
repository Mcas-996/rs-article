## ADDED Requirements

### Requirement: English Dialect Selection
The application SHALL allow users to select from available English dialects.

#### Scenario: Default dialect
- **WHEN** the application starts
- **THEN** American English SHALL be the default dialect

#### Scenario: Change dialect
- **WHEN** user selects a different dialect from the dropdown
- **THEN** all subsequent linting SHALL use the selected dialect
