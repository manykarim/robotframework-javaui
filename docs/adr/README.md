# Architecture Decision Records (ADRs)

This directory contains Architecture Decision Records for the robotframework-javagui unified keywords implementation.

## Overview

The ADRs document key architectural decisions for unifying the Swing, SWT, and RCP keyword APIs into a consistent, maintainable library.

## ADR Index

| ADR | Title | Status | Summary |
|-----|-------|--------|---------|
| [ADR-001](./ADR-001-unified-base-class-architecture.md) | Unified Base Class Architecture | Proposed | Trait-based composition architecture for sharing code across technologies |
| [ADR-002](./ADR-002-locator-syntax-strategy.md) | Locator Syntax Strategy | Proposed | Unified locator syntax with automatic normalization |
| [ADR-003](./ADR-003-keyword-naming-convention.md) | Keyword Naming Convention | Proposed | Element-based naming with backwards-compatible aliases |
| [ADR-004](./ADR-004-technology-detection-and-mode-selection.md) | Technology Detection and Mode Selection | Proposed | Auto-detection and explicit mode selection for GUI technologies |
| [ADR-005](./ADR-005-error-handling-strategy.md) | Error Handling Strategy | Proposed | Unified exception hierarchy with rich error context |
| [ADR-006](./ADR-006-backwards-compatibility-approach.md) | Backwards Compatibility Approach | Proposed | Three-phase deprecation with automated migration tools |
| [ADR-007](./ADR-007-UNIFIED-KEYWORD-API.md) | Unified Keyword API Design | Proposed | Browser Library-style API with ~20 core keywords and assertion engine |
| [ADR-008](./ADR-008-security-architecture.md) | Security Architecture | Proposed | Defense-in-depth security for RPC, locators, and data protection |
| [ADR-009](./ADR-009-implementation-and-migration-plan.md) | Implementation and Migration Plan | Proposed | Four-phase delivery strategy with quality gates, testing, and rollback procedures |
| [ADR-011](./ADR-011-PYTHON-KEYWORD-LAYER.md) | Python Keyword Layer Design | Proposed | AssertionEngine integration, ElementState flags, retry mechanism, and formatters |
| [ADR-014](./ADR-014-IMPLEMENTATION-PLAN-ASSERTIONENGINE.md) | AssertionEngine Implementation Plan | Proposed | 5-phase implementation with Claude-flow self-learning integration (8 weeks) |

## Context

### Current State
- **SwingLibrary**: ~55 keywords for Java Swing applications
- **SwtLibrary**: ~55 keywords for Eclipse SWT applications
- **RcpLibrary**: ~83 keywords (55 SWT + 28 RCP-specific) for Eclipse RCP applications

### Goals
1. Reduce code duplication (~70% of keywords have identical semantics)
2. Provide consistent API across all technologies
3. Maintain full backwards compatibility
4. Support auto-detection and mode switching
5. Improve error messages and debugging experience

### Technology Stack
- **Rust**: Core implementation with PyO3 bindings
- **Python**: Robot Framework integration
- **Java**: Agent for GUI instrumentation
- **JSON-RPC**: Communication protocol

## Decision Flow

```
ADR-001 (Base Architecture)
    |
    +---> ADR-002 (Locators)
    |         |
    |         +---> ADR-008 (Security) <--- Locator validation
    |
    +---> ADR-003 (Naming)
    |         |
    |         +---> ADR-006 (Compatibility)
    |         |
    |         +---> ADR-007 (Unified API)
    |                   |
    |                   +---> ADR-011 (Python Keyword Layer)
    |
    +---> ADR-004 (Mode Detection)
    |
    +---> ADR-005 (Errors)
    |         |
    |         +---> ADR-006 (Compatibility)
    |         |
    |         +---> ADR-007 (Unified API)
    |         |
    |         +---> ADR-008 (Security) <--- Audit logging
    |
    +---> ADR-008 (Security Architecture)
    |         |
    |         +---> RPC security, session management
    |         +---> Data sanitization, PII protection
    |
    +---> ADR-009 (Implementation Plan) <--- Coordinates all ADRs
              |
              +---> Phase 1: Foundation (ADR-001, ADR-002)
              +---> Phase 2: Core Keywords (ADR-003, ADR-005)
              +---> Phase 3: Advanced Features (ADR-007, ADR-008)
              +---> Phase 4: Migration Support (ADR-006)
```

## Implementation Phases

### Phase 1: Foundation (Weeks 1-4)
- ADR-001: Core traits and shared implementation
- ADR-002: Locator normalization layer

### Phase 2: API Unification (Weeks 5-8)
- ADR-003: Keyword renaming with aliases
- ADR-004: Mode detection and selection

### Phase 3: Unified API (Weeks 9-14)
- ADR-007: Assertion engine implementation
- ADR-007: Core keyword consolidation (~20 keywords)
- ADR-007: Backwards compatibility aliases
- ADR-011: Python keyword layer with AssertionEngine integration

### Phase 4: Security (Weeks 15-20)
- ADR-008: Locator validation and injection prevention
- ADR-008: Session security and RPC hardening
- ADR-008: Data sanitization and audit logging

### Phase 5: Polish (Weeks 21-24)
- ADR-005: Error handling improvements
- ADR-006: Migration tools and documentation
- Security testing and penetration testing

## ADR Template

New ADRs should follow this format:

```markdown
# ADR-XXX: Title

| ADR ID | ADR-XXX |
|--------|---------|
| Title | Title |
| Status | Proposed / Accepted / Deprecated / Superseded |
| Date | YYYY-MM-DD |
| Authors | Team/Person |

## Context
Why is this decision needed?

## Decision
What did we decide?

## Consequences
### Positive
- Good outcomes

### Negative
- Trade-offs

### Risks
- Potential issues

## Alternatives Considered
What else was considered and why rejected?

## Implementation Plan
High-level implementation steps.

## References
Related documents and resources.
```

## Status Definitions

| Status | Meaning |
|--------|---------|
| **Proposed** | Under discussion, not yet accepted |
| **Accepted** | Approved for implementation |
| **Implemented** | Fully implemented in code |
| **Deprecated** | No longer recommended, superseded |
| **Superseded** | Replaced by another ADR |

## Related Documents

- [Unified Keywords Research](/docs/unify_keywords_research.md) - Detailed analysis of keyword unification
- [Locator Implementation Plan](/docs/LOCATOR_IMPLEMENTATION_PLAN.md) - Locator syntax details
- [Test Coverage Report](/docs/test-coverage-report.md) - Current test coverage analysis
