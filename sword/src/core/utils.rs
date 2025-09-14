use byte_unit::Byte;
use regex::Regex;
use serde::Deserialize;
use std::{env, str::FromStr};

/// Custom deserializer for size values in configuration.
///
/// This function allows configuration values like "10MB", "1GB", etc. to be
/// automatically converted to byte counts as `usize` values. It uses the
/// `byte_unit` crate to parse human-readable size specifications.
///
/// ### Supported Units
///
/// - B (bytes)
/// - KB, MB, GB, TB (decimal)
/// - KiB, MiB, GiB, TiB (binary)
///
/// ### Example
///
/// ```toml,ignore
/// [application]
/// body_limit = "10MB"  # Parsed as 10,000,000 bytes
/// # or
/// body_limit = "10MiB" # Parsed as 10,485,760 bytes
/// ```
pub fn deserialize_size<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    Byte::from_str(&s)
        .map(|b| b.as_u64() as usize)
        .map_err(serde::de::Error::custom)
}

pub(crate) fn expand_env_vars(content: &str) -> Result<String, String> {
    let re = Regex::new(r"\$\{([A-Za-z_][A-Za-z0-9_]*):?([^}]*)\}")
        .map_err(|e| format!("Regex error: {e}"))?;

    let mut result = content.to_string();

    for caps in re.captures_iter(content) {
        let full_match = caps.get(0).unwrap().as_str();
        let var_name = caps.get(1).unwrap().as_str();
        let default_value = caps.get(2).map(|m| m.as_str()).unwrap_or("");

        let replacement = match env::var(var_name) {
            Ok(value) => value,
            Err(_) => {
                if default_value.is_empty() {
                    return Err(format!(
                        "environment variable '{var_name}' not found"
                    ));
                } else {
                    default_value.to_string()
                }
            }
        };

        result = result.replace(full_match, &replacement);
    }

    let simple_re = Regex::new(r"\$([A-Za-z_][A-Za-z0-9_]*)")
        .map_err(|e| format!("Regex error: {e}"))?;

    for caps in simple_re.captures_iter(&result.clone()) {
        let full_match = caps.get(0).unwrap().as_str();
        let var_name = caps.get(1).unwrap().as_str();

        if content.contains(&format!("${{{var_name}")) {
            continue;
        }

        let replacement = match env::var(var_name) {
            Ok(value) => value,
            Err(_) => {
                return Err(format!("environment variable '{var_name}' not found"));
            }
        };

        result = result.replace(full_match, &replacement);
    }

    Ok(result)
}
