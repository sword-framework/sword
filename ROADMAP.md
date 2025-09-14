# Roadmap of the Project

This document outlines the planned features, improvements for future releases. If you have any suggestions or would like to contribute, please feel free to open an issue or a pull request. (See [CONTRIBUTING.md](CONTRIBUTING.md) for more details.)

## Planned Features

- **Tracing logging**: Implement built-in tracing logging to enhance debugging and monitoring capabilities.

- **Improved error handling**: Enhance error handling mechanisms.

- **Implement missing HTTP methods**: Add support for any missing HTTP methods to ensure full compliance with HTTP standards. (OPTIONS and HEAD).

- **Benchmarking and performance comparisons**: Conduct benchmarking tests and performance comparisons.

- **OpenAPI/Swagger support**: Integrate OpenAPI/Swagger support with `utoipa` for api documentation.

- **Feature based validation library**: Delegate the choice of validation library to the user. (e.g. `validator`, `garde`, and others).

- **Built-in CORS by tower-http**: Implement built-in CORS support using the `tower-http` crate. Currently, users need to manually add CORS middleware using the `with_layer` method.
