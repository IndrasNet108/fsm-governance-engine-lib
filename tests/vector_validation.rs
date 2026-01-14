use std::fs;
use std::path::Path;

use fsm_governance_engine_lib::{FsmDefinition, FsmError};

fn parse_expected(expected: &str) -> Result<(), String> {
    let trimmed = expected.trim();
    if trimmed == "OK" {
        return Ok(());
    }

    if let Some(code) = trimmed.strip_prefix("ERR:") {
        return Err(code.to_string());
    }

    Err("InvalidExpectedFormat".to_string())
}

fn parse_error_code(code: &str) -> FsmError {
    match code {
        "InvalidInput" => FsmError::InvalidInput,
        "InvalidStateTransition" => FsmError::InvalidStateTransition,
        "InsufficientMembers" => FsmError::InsufficientMembers,
        "InvalidState" => FsmError::InvalidState,
        "Overflow" => FsmError::Overflow,
        _ => FsmError::InvalidInput,
    }
}

#[test]
fn validate_test_vectors() {
    let dir = Path::new("tests/vectors");
    let entries = fs::read_dir(dir).expect("read vector dir");

    for entry in entries {
        let entry = entry.expect("vector entry");
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }

        let expected_path = path.with_extension("expected");
        let expected_raw = fs::read_to_string(&expected_path)
            .unwrap_or_else(|_| panic!("missing expected file for {:?}", path));
        let expected = parse_expected(&expected_raw);

        let raw = fs::read_to_string(&path).expect("read definition");
        let definition: FsmDefinition = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("parse definition {:?}: {}", path, err));
        let result = definition.validate();

        match (expected, result) {
            (Ok(()), Ok(())) => {}
            (Err(code), Err(err)) => {
                let expected_err = parse_error_code(&code);
                assert_eq!(err, expected_err, "vector {:?}", path);
            }
            (Ok(()), Err(err)) => panic!("expected OK for {:?}, got {:?}", path, err),
            (Err(code), Ok(())) => panic!("expected ERR:{} for {:?}", code, path),
        }
    }
}
