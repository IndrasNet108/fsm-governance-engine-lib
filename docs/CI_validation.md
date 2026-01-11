# CI Validation Step

This project is designed to run as a validation step in CI/CD.

## Example usage

```bash
./scripts/validate_fsm.sh docs/example_fsm_definition.json docs/FSM_schema.json
```

## GitHub Actions snippet

```yaml
- name: validate fsm definitions
  run: ./scripts/validate_fsm.sh
```

## Notes

- The script runs `fsm_validate` in strict mode.
- Override paths by passing `definition` and `schema` as arguments.
- This step validates process definitions and transitions; it does not execute actions or deploy artifacts.
- Expected: OK: FSM definition is valid.
