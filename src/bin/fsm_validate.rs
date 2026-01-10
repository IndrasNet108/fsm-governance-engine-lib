use std::env;
use std::fs;
use std::process;

use fsm_governance_engine_lib::FsmDefinition;
use jsonschema::JSONSchema;
use serde_json::Value;

fn main() {
    let mut args = env::args().skip(1);
    let mut path: Option<String> = None;
    let mut schema_path: Option<String> = None;
    let mut strict = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--schema" => {
                schema_path = args.next();
                if schema_path.is_none() {
                    eprintln!("--schema requires a path");
                    process::exit(2);
                }
            }
            "--strict" => {
                strict = true;
            }
            _ => {
                if path.is_none() {
                    path = Some(arg);
                } else {
                    eprintln!("Unexpected argument: {}", arg);
                    print_usage();
                    process::exit(2);
                }
            }
        }
    }

    let path = match path {
        Some(path) => path,
        None => {
            print_usage();
            process::exit(2);
        }
    };

    let raw = match fs::read_to_string(&path) {
        Ok(raw) => raw,
        Err(err) => {
            eprintln!("Failed to read {}: {}", path, err);
            process::exit(1);
        }
    };

    let json_value: Value = match serde_json::from_str(&raw) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Invalid JSON: {}", err);
            process::exit(1);
        }
    };

    if let Some(schema_path) = schema_path {
        let schema_raw = match fs::read_to_string(&schema_path) {
            Ok(schema_raw) => schema_raw,
            Err(err) => {
                eprintln!("Failed to read schema {}: {}", schema_path, err);
                process::exit(1);
            }
        };

        let schema_json: Value = match serde_json::from_str(&schema_raw) {
            Ok(value) => value,
            Err(err) => {
                eprintln!("Invalid schema JSON: {}", err);
                process::exit(1);
            }
        };

        let compiled = match JSONSchema::compile(&schema_json) {
            Ok(compiled) => compiled,
            Err(err) => {
                eprintln!("Schema compile error: {}", err);
                process::exit(1);
            }
        };

        if let Err(errors) = compiled.validate(&json_value) {
            eprintln!("Schema validation failed:");
            for error in errors.take(5) {
                eprintln!("- {}", error);
            }
            process::exit(1);
        }
    }

    let definition: FsmDefinition = match serde_json::from_value(json_value) {
        Ok(definition) => definition,
        Err(err) => {
            eprintln!("Invalid definition: {}", err);
            process::exit(1);
        }
    };

    if let Err(err) = definition.validate() {
        eprintln!("Validation failed: {}", err);
        process::exit(1);
    }

    if strict {
        if let Err(err) = validate_strict(&definition) {
            eprintln!("Strict validation failed: {}", err);
            process::exit(1);
        }
    }

    println!("OK: FSM definition is valid.");
}

fn validate_strict(definition: &FsmDefinition) -> Result<(), fsm_governance_engine_lib::FsmError> {
    if definition.invariants.is_empty() {
        return Err(fsm_governance_engine_lib::FsmError::InvalidInput);
    }

    let has_initial = definition
        .defaults
        .as_ref()
        .and_then(|defaults| defaults.initial_state.as_ref())
        .is_some();

    if !has_initial {
        return Err(fsm_governance_engine_lib::FsmError::InvalidInput);
    }

    Ok(())
}

fn print_usage() {
    eprintln!("Usage: fsm_validate <path_to_definition.json> [--schema <schema.json>] [--strict]");
}
