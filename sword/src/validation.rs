use serde_json::{Map, Value};
use validator::{ValidationError, ValidationErrors};

pub fn format_errors(e: &ValidationErrors) -> Value {
    let mut map = Map::new();

    let to_error_message = |err: &ValidationError| {
        err.message
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| err.code.to_string())
    };

    for (field, errs) in e.field_errors() {
        let msgs = errs
            .iter()
            .map(&to_error_message)
            .map(Value::String)
            .collect();

        map.insert(field.to_string(), Value::Array(msgs));
    }

    Value::Object(map)
}
