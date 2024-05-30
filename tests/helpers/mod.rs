use std::{error::Error, fs::File, io::BufReader};

use num::Num;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use subprocess::{Popen, PopenConfig, Redirection, Redirection::Pipe};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestEntry {
    pub x: String,
    pub y: String,
    pub flags: String,
    pub result: TestResult,
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
pub struct TestResult {
    pub value: String,
    pub flags: String,
}

pub fn stream_json_from_array_file<F>(filename: &str, mut f: F)
where
    F: FnMut(TestEntry) -> (),
{
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let deserializer = serde_json::Deserializer::from_reader(reader);
    let array_iter = deserializer.into_iter::<TestEntry>();
    for item in array_iter {
        match item {
            Ok(value) => f(value),
            Err(e) => panic!("died"),
        }
    }
}
