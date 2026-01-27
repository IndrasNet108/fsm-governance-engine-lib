---
name: Proposal
about: Propose a normative change affecting the model, artifacts, or operational rules
labels: proposal
---

**Target document / component**
Specify what this proposal affects:
- ISO / spec document
- API / library behavior
- evidence / artifact format
- operational rules / governance docs
- public language / claims

**Normative change**
Describe the proposed change precisely.
Avoid motivation or benefits here — only what changes.

**Rationale**
Explain why this change is necessary within the project’s scope.
Reference existing constraints, failures, or limitations.

**Backward compatibility**
- Is this a breaking change? (yes/no)
- If yes, describe required versioning or migration.

**Falsifiability analysis**
Explain how the proposal preserves:
- falsification-only behavior
- claim asymmetry (if applicable)
- artifacts as the truth object

**Misuse and language drift risks**
Describe possible misinterpretations or misuse introduced by this change.
Explain how they are mitigated.

**Acceptance criteria**
List objective conditions for acceptance, such as:
- updated specs/docs
- tests or fixtures
- CI/verification steps updated

**Scope acknowledgement**
- [ ] I understand that this project is validation-only and does not provide correctness, safety, compliance, or decision-making guarantees.
