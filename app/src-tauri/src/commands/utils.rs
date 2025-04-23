use serde::Serialize;
use serde_json::Value;

pub fn serialize<T: Serialize>(v: T) -> Value {
    serde_json::to_value(v).unwrap()
}
