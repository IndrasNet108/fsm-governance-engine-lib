# Fuzzing

## Scope
Fuzzing targets definition parsing and validation logic.
The goal is to detect crashes and invalid acceptance.

## Tooling
This project uses cargo fuzz with libFuzzer.

## Targets

- definition_parser
  - Parses definition JSON into FsmDefinition.

- definition_validate
  - Parses definition JSON and runs validate.

## Workspace
The fuzz crate is included in workspace members to keep CI stable.

## Budget
Nightly budget: ten minutes per target.

## Commands

- cargo fuzz run definition_parser
- cargo fuzz run definition_validate

## Triage
Crashes and invalid acceptance must be minimized and turned into tests.
