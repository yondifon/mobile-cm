use std::fs;
use std::path::PathBuf;

use mobile_cm::{check, is_camtel, is_mtn, is_nexttel, is_orange};
use serde_json::Value;

fn spec_cases() -> Vec<(String, Option<String>)> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../spec/cases.json");
    let data = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    let value: Value = serde_json::from_str(&data).expect("parse cases.json");

    value
        .as_array()
        .expect("cases.json is a JSON array")
        .iter()
        .map(|case| {
            let input = case["input"]
                .as_str()
                .expect("case.input is a string")
                .to_string();
            let operator = case["operator"].as_str().map(str::to_string);
            (input, operator)
        })
        .collect()
}

#[test]
fn matches_spec_cases() {
    for (input, expected) in spec_cases() {
        let expected = expected.as_deref();

        assert_eq!(
            check(&input).map(|operator| operator.as_str()),
            expected,
            "check({input:?})"
        );
        assert_eq!(is_mtn(&input), expected == Some("mtn"), "is_mtn({input:?})");
        assert_eq!(
            is_orange(&input),
            expected == Some("orange"),
            "is_orange({input:?})"
        );
        assert_eq!(
            is_nexttel(&input),
            expected == Some("nexttel"),
            "is_nexttel({input:?})"
        );
        assert_eq!(
            is_camtel(&input),
            expected == Some("camtel"),
            "is_camtel({input:?})"
        );
    }
}
