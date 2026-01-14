# Triage Process

## Scope
This process covers bugs and security relevant defects in validation behavior.
The goal is fast reproduction, minimization, and risk classification.

## Intake

Required information:
- Clear description of the issue.
- Reproduction steps.
- Expected result and actual result.
- Inputs used to reproduce.

## Severity levels

- Critical: validation accepts invalid transitions or invariants.
- High: validation rejects valid transitions or invariants.
- Medium: incorrect error classification or partial validation failure.
- Low: documentation or test coverage gaps.

## Triage steps

1. Reproduce the issue from the report.
2. Minimize inputs to the smallest failing case.
3. Classify severity.
4. Add or update a test that captures the failure.
5. Fix the issue and update changelog if critical.

## Escalation

Critical issues are handled first and require a test, fix, and changelog entry.

