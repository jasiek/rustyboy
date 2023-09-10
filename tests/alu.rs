use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct TestEntry {
    x: String,
    y: String,
    flags: String,
    result: TestResult,
}

#[derive(Debug, Deserialize)]
struct TestResult {
    value: String,
    flags: String,
}

#[test]
fn test_add() {
    let mut file = File::open("sm83-test-data/alu_tests/v1/add.json").unwrap();
    let json: Vec<TestEntry> = serde_json::from_reader(file).expect("bad json");
}
