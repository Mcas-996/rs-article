## ADDED Requirements

### Requirement: Real-time Grammar Checking
The application SHALL perform grammar checking in real-time as the user types, with a 10ms debounce delay.

#### Scenario: Check text on input
- **WHEN** user types in the editor
- **THEN** grammar checking SHALL occur after 10ms of no typing

#### Scenario: Display lint results
- **WHEN** harper-core returns lint results
- **THEN** errors SHALL be highlighted in the text and shown in a suggestions panel

#### Scenario: Multiple errors
- **WHEN** multiple lint errors exist in the document
- **THEN** all errors SHALL be displayed with their positions and suggestions
