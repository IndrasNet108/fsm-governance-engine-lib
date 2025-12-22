# Release checklist

1. **Update version** in `Cargo.toml` and `Cargo.lock`.
2. **Cargo package**:
   ```
   CARGO_TARGET_DIR=/home/indrasnet/fsm_target cargo package
   ```
3. **Testing**: ensure `cargo test` (via `CARGO_TARGET_DIR`) passes after changes.
4. **Create audit trail docs**: update `docs/AuditTrail.md` with any schema changes.
5. **Publish**:
   ```
   cargo publish --locked
   ```
6. **Announce**: update README, upload release notes, push tag `vX.Y.Z`.
7. **Translate docs** (Phase 3): provide `docs/README.<lang>.md` for at least two languages.

Keep changelog references in GitHub release and mention NLnet deliverables.
