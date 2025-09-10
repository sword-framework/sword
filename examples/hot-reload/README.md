# Sword web framework - Hot Reload Example

This example demonstrates how to set up a basic web server using the Sword web framework with hot reload capabilities. To run this example, ensure you have the necessary dependencies installed and follow the instructions below.

## Prerequisites

- Dioxus CLI installed. You can install it using Cargo:

  ```
  cargo install --git https://github.com/DioxusLabs/dioxus.git dioxus-cli
  ```

## Running the Example

```
dx serve --hot-patch
```

This command starts the server with hot reload enabled. Any changes you make to the source code will be automatically detected, and the server will reload without needing to restart it manually.
