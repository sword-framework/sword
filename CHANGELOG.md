# Sword web framework changelog

## [0.1.8]

### Added

- Added `helmet` feature to `sword`. This allows users to enable security-related HTTP headers for their applications. It's built on top of the `rust-helmet` and `axum-helmet` crates.

- Added built-in `Timeout` middleware based on `tower_http` TimeoutLayer. This middleware allows users to set a timeout duration for requests, enhancing the robustness of their applications. The number of seconds can be configured on the config .toml file at the `application` key. See the examples for usage.

- Added documentation comments to all public functions and structs in the `sword` crate. This improves code readability and helps users understand the functionality of the framework better.

- Added `cookies` feature flag to `sword`. This enables cookie parsing and management. It uses the `tower-cookies` crate for cookie handling. This feature allows users to use Cookies, PrivateCookies, and SignedCookies in their handlers. See the examples for usage.

- Added `multipart` feature flag to `sword`. This enables multipart form data handling using the `multipart` feature flag of `axum` crate. Basically it provides `Multipart` extractor for handling file uploads and form data.

- Added support for axum run with graceful shutdown. This allows the application to handle shutdown signals gracefully, ensuring that ongoing requests are completed before the server stops.

- Added `tower_http` layers support to `middleware macro`. This allows users to easily add middleware layers from the `tower_http` to controllers using the `#[middleware]` attribute.

### Changed

- Move `hot-reload` functionality to another branch due to its constantly changing development state.

- Changed the `serde_qs` dependency to native `axum` query extraction. This simplifies the codebase and reduces dependencies.

- Changed the `#[controller_impl]` macro to `#[routes]`. This change improves clarity and consistency in the codebase.

- Changed the builder pattern for `Application` struct. Now, all the build methods start with `with_` prefix. For example, `with_layer`, `with_controller`, etc. This change enhances code readability and consistency.
