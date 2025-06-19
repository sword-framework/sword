use serde_json::{Value, json};

#[cfg(feature = "validation")]
use validator::{ValidationError, ValidationErrors};

#[cfg(feature = "validation")]
pub fn format_validation_errors(e: &ValidationErrors) -> Value {
    let mut errors = Vec::new();

    let to_error_message = |err: &ValidationError| {
        err.message
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| err.code.to_string())
    };

    for (field, field_errors) in e.field_errors() {
        let field_name = field.to_string();
        let messages: Vec<String> = field_errors.iter().map(to_error_message).collect();

        for message in messages {
            let error = json!({
                "field": field_name,
                "message": message,
            });

            errors.push(error);
        }
    }

    Value::Array(errors)
}
