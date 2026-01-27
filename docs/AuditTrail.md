# Audit trail specification

Audit trail records validation outcomes for attempted state transitions. Each entry is immutable and can be serialized both as binary (`borsh`) and JSON (`serde`) for downstream verification or export.

## Entry format (`AuditEntry`)

| Field | Type | Description |
|-------|------|-------------|
| `grant_id` | `u64` | Identifier for the domain entity (e.g., grant). |
| `actor` | `[u8; 32]` | Public key or identifier of the actor performing the transition. |
| `from_state` | `GrantStatus` | Source state. |
| `to_state` | `GrantStatus` | Destination state. |
| `action` | `&'static str` | Human-readable action label (e.g., `"approve"`). |
| `timestamp` | `i64` | Unix timestamp. |
| `metadata` | `Option<String>` | Optional payload with a link or comment. |

## API

```rust
let mut trail = AuditTrail::new();
trail.record(AuditEntry::new(
    grant.id,
    actor_key,
    GrantStatus::Pending,
    GrantStatus::Approved,
    "approve",
    clock.unix_timestamp,
    Some("committee".into()),
))?;
trail.verify()?; // confirms chained transitions align
```

`record` validates that the entry respects the FSM transition graph before appending.

`verify` ensures the sequence does not skip states or mix `grant_id`s improperly; returns `FsmError::InvalidStateTransition` for violations.

## Exporting

- To stream entries to JSON: `serde_json::to_string(&trail.entries())`.
- To persist to disk: `trail.entries().try_to_vec()` (Borsh).

Use this audit trail as an immutable log for compliance, replay, or integration with external verifiers.
