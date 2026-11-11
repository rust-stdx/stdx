#![cfg(feature = "alloc")]

//! Optional integration with the `nst/JSONTestSuite` corpus.
//!
//! Set `JSON_TEST_SUITE_DIR` to the repository's `test_parsing` directory to
//! run it. The test is skipped when the variable is unset, so CI does not need
//! to download the corpus.

use std::path::Path;

#[test]
fn json_test_suite() {
    let Ok(dir) = std::env::var("JSON_TEST_SUITE_DIR") else {
        return;
    };
    let dir = Path::new(&dir);
    assert!(dir.is_dir(), "JSON_TEST_SUITE_DIR is not a directory: {dir:?}");

    let mut failures = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("json") {
            continue;
        }
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let bytes = std::fs::read(&path).unwrap();
        let validated = json::validate(&bytes).is_ok();
        let parsed = json::from_slice::<json::Value>(&bytes).is_ok();
        if name.starts_with("y_") && (!validated || !parsed) {
            failures.push(format!("{name}: expected acceptance (validate={validated}, parse={parsed})"));
        } else if name.starts_with("n_") && (validated || parsed) {
            failures.push(format!("{name}: expected rejection (validate={validated}, parse={parsed})"));
        }
    }

    assert!(failures.is_empty(), "JSONTestSuite failures:\n{}", failures.join("\n"));
}
