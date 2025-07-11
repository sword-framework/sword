use std::error::Error;

pub fn format_error_with_sources(error: &dyn Error) -> String {
    let mut result = String::new();
    let mut current_error = Some(error);
    let mut level = 0;

    while let Some(err) = current_error {
        if level == 0 {
            result.push_str(&format!("Error: {err}"));
        } else {
            result.push_str(&format!("\n  Caused by ({level}): {err}"));
        }

        current_error = err.source();
        level += 1;
    }

    result
}

pub fn display_error_chain(error: &dyn Error) {
    eprintln!("{}", format_error_with_sources(error));
}
