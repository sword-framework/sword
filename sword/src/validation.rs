use serde_json::{Value, json};
use validator::{ValidationError, ValidationErrors};

pub fn format_validation_errors(e: &ValidationErrors) -> Value {
    let mut errors = Vec::new();

    let to_error_message = |err: &ValidationError| {
        err.message
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or(err.code.to_string())
    };

    for (field, field_errors) in e.field_errors() {
        let field_name = field.to_string();
        let messages: Vec<String> =
            field_errors.iter().map(to_error_message).collect();

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
