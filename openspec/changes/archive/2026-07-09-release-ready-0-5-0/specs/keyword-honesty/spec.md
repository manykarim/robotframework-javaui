## ADDED Requirements

### Requirement: No keyword returns fabricated or placeholder data
A keyword SHALL NOT return hardcoded, empty, or placeholder data while presenting itself as functional. Any keyword that cannot perform its documented function SHALL either be implemented, or removed, or clearly raise/return a documented not-supported result.

#### Scenario: List Applications resolved
- **WHEN** `List Applications` is called
- **THEN** it either enumerates real connectable Java applications (JVM discovery) or is removed from the public keyword surface; it MUST NOT silently return an empty placeholder list while documented as returning available applications

### Requirement: Keyword behavior matches its documentation
Every keyword's documented behavior SHALL match what it actually does. Docstrings SHALL NOT claim mechanisms the implementation does not perform.

#### Scenario: Type Text is honest about mechanism
- **WHEN** `Type Text` is invoked
- **THEN** it either simulates real per-character key events as documented, or its documentation is corrected to state it sets text via the same path as `Input Text`

#### Scenario: Log UI Tree honors its locator argument
- **WHEN** `Log UI Tree` is called with a `locator` argument
- **THEN** it logs the subtree rooted at the located element; if the argument cannot be honored it SHALL be removed from the signature rather than silently ignored

### Requirement: Documented keyword count is accurate and single-sourced
The project SHALL publish one authoritative keyword count/reference; conflicting counts across documents SHALL be eliminated.

#### Scenario: Consistent count
- **WHEN** the README and keyword reference are reviewed
- **THEN** they cite one consistent keyword count derived from the actual keyword surface, with no contradictory figures elsewhere in shipped docs
