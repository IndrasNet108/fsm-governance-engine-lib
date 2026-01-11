# Audit Model

The audit model records validation events for state transitions. The engine does
not enforce storage; it defines the record structure and validation semantics.

## AuditEntry fields

- `grant_id`: domain identifier of the process instance.
- `actor`: 32-byte actor identifier.
- `from_state`: previous state.
- `to_state`: next state.
- `action`: symbolic action label.
- `timestamp`: integer timestamp.
- `metadata`: optional string metadata.

## Ordering and integrity

- Records are append-only within a single audit trail.
- Ordering is deterministic within the validation scope; storage layers define
  persistence and global ordering.
- `AuditTrail::verify()` checks state continuity across entries for a single
  process identifier.

## Usage guidance

- Persist audit entries in immutable or append-only storage where required.
- Treat validation failures as hard errors in downstream systems.
- Use JSON serialization for audit export and review.
