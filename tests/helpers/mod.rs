use num::Num;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestEntry {
    x: String,
    y: String,
    flags: String,
    result: TestResult,
}

fn value_as<T: Num>(value: &str) -> T where {
    match T::from_str_radix(value, 16) {
        Ok(v) => return v,
        Err(_) => {
            panic!("can't convert")
        }
    }
}

pub fn x_as<T: Num>(t: &TestEntry) -> T {
    return value_as(&t.x);
}

pub fn y_as<T: Num>(t: &TestEntry) -> T {
    return value_as(&t.y);
}

pub fn result_value<T: Num>(t: &TestEntry) -> T {
    return value_as(&t.result.value);
}

#[derive(Debug, Serialize, Deserialize)]
struct TestResult {
    value: String,
    flags: String,
}
