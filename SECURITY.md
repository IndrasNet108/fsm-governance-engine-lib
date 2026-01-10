# Security Policy

## Scope

FSM Governance Engine is a validation-only library for deterministic verification of integrity-critical processes.

The project:
- does **not** execute actions,
- does **not** automate decisions,
- does **not** enforce outcomes,
- does **not** manage credentials, keys, or secrets,
- does **not** perform network or filesystem operations.

Its sole responsibility is to validate process state transitions against formally defined rules and invariants.

---

## Threat Model

The primary threat model addressed by this project is **process integrity failure**, including:

- invalid or undocumented state transitions,
- hidden or implicit process states,
- out-of-order execution of governance or compliance steps,
- lack of reproducible audit evidence,
- human error in integrity-critical workflows.

The project does **not** aim to mitigate:
- runtime compromise of the host system,
- cryptographic attacks,
- network-level attacks,
- side-channel attacks,
- denial-of-service scenarios.

---

## Security Properties

The following properties are explicitly provided:

- **Deterministic behavior**  
  Identical inputs always produce identical validation outcomes.

- **Validation-only semantics**  
  The engine validates transitions but never performs side effects.

- **Explicit transition rules**  
  All allowed transitions are formally defined and exhaustively enumerated.

- **Invariant enforcement**  
  Declared invariants are checked as first-class validation constraints.

- **Fail-closed behavior**  
  Invalid transitions are rejected and surfaced as explicit errors.

- **Reproducibility**  
  Validation results can be reproduced using the same inputs and definitions.

- **Auditability**  
  Validation steps can produce structured, append-only audit records.

---

## Out of Scope

The following are explicitly out of scope:

- authorization or access control,
- identity management,
- key handling or cryptography,
- policy enforcement,
- workflow orchestration,
- execution of business logic,
- automated remediation.

Downstream systems are responsible for enforcing decisions and actions.

---

## Secure Usage Guidelines

Consumers of this library are expected to:

- treat validation failures as hard errors,
- ensure FSM definitions are schema-validated before use,
- persist audit artifacts in immutable or append-only storage where required,
- apply independent authorization and execution controls outside this library,
- review FSM definitions and invariants as part of change management.

---

## Reporting Vulnerabilities

Security issues can be reported by email to:

**info@indrasnet.ee**

Please include:
- a clear description of the issue,
- steps to reproduce (if applicable),
- affected versions or commit references.

We aim to acknowledge reports within a reasonable timeframe and coordinate disclosure where appropriate.

---

## Security Updates

Security-relevant changes will be documented in the project changelog.
No automatic update or notification mechanism is provided.

---

## Disclaimer

This project provides validation primitives only.
It is not a complete security solution and must be integrated into a broader security architecture.
