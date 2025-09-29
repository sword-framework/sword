use serde_json::{Map, Value, json};

#[cfg(feature = "garde")]
use garde::Report;

#[cfg(feature = "validator")]
use validator::ValidationErrors;

#[cfg(feature = "validator")]
/// Following the RFC 9457 (Problem Details for HTTP APIs)
/// formats validation errors from the `validator` crate into a structured JSON object.
///
/// # Example
///
/// ```json
/// {
///   "email": [
///     {
///       "code": "invalid",
///       "message": "Must be a valid email address"
///     }
///   ],
///   "password": [
///     {
///       "code": "length",
///       "message": "Must be at least 8 characters long"
///     },
///     {
///       "code": "strength",
///       "message": "Must contain a number"
///     }
///   ]
/// }
/// ```
pub fn format_validator_errors(e: ValidationErrors) -> Value {
    let mut formatted_errors = Map::new();

    for (field, field_errors) in e.field_errors() {
        let mut formatted_field_errors = vec![];

        for error in field_errors {
            formatted_field_errors.push(json!({
                "code": error.code,
                "message": error.message,
            }));
        }

        formatted_errors
            .insert(field.to_string(), Value::Array(formatted_field_errors));
    }

    Value::Object(formatted_errors)
}

#[cfg(feature = "garde")]
/// Following the RFC 9457 (Problem Details for HTTP APIs)
/// formats validation errors from the `garde` crate into a structured JSON object.
///
/// ## Important Note
/// Garde does not provide error codes for its validation errors.
///
/// ## Example
///
/// ```json
/// {
///   "email": [
///     {
///       "message": "Must be a valid email address"
///     }
///   ],
///   "password": [
///     {
///       "message": "Must be at least 8 characters long"
///     },
///   ]
/// }
/// ```
pub fn format_garde_errors(report: Report) -> Value {
    let mut formatted_errors = Map::new();

    for (path, err) in report.into_inner() {
        let key = path.to_string();
        let entry = formatted_errors
            .entry(key)
            .or_insert_with(|| Value::Array(vec![]));

        if let Value::Array(arr) = entry {
            arr.push(json!({
                "message": err.message()
            }));
        }
    }

    Value::Object(formatted_errors)
}
