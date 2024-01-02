# Project README

## Description
This project is a web application built in Rust using the [warp](file:///c%3A/Users/Ashish%20Gupta/Desktop/Rust/FirstWebApp/Cargo.toml#10%2C1-10%2C1) framework. It provides APIs for managing todos and serves static content.

## Getting Started
To start the application, follow these steps:

1. Clone the repository.
2. Make sure you have Rust and Cargo installed.
3. Navigate to the project directory.
4. Run the following command to start the application:
```
cargo run
```

5. The application will start running on `http://127.0.0.1:1313`.

## APIs
The application provides the following APIs:

### Get Todos
- Endpoint: `/todos/{id}`
- Method: GET
- Headers:
  - [X-Auth-Token](file:///c%3A/Users/Ashish%20Gupta/Desktop/Rust/FirstWebApp/dev.http#4%2C1-4%2C1): The authentication token
- Example Request:
  http
  GET http://127.0.0.1:1313/todos/123
  X-Auth-Token: 123.exp.signature
  Content-Type: application/json
- Example Response:
  ```js
  json
  {
  "id": 5,
  "title": "Todo test"
  }
  ```

### Hi API
- Endpoint: `/hi`
- Method: GET
- Example Request:
  http
  GET http://127.0.0.1:1313/hi
- Example Response:
  Hi, World!


## Static Content
The application serves static content from the [web-folder](file:///c%3A/Users/Ashish%20Gupta/Desktop/Rust/FirstWebApp/src/main.rs#8%2C27-8%2C27) directory. The root URL (`http://127.0.0.1:1313/`) serves the `index.html` file from the [web-folder](file:///c%3A/Users/Ashish%20Gupta/Desktop/Rust/FirstWebApp/src/main.rs#8%2C27-8%2C27).

## Dependencies
The project uses the following dependencies:

- [tokio](file:///c%3A/Users/Ashish%20Gupta/Desktop/Rust/FirstWebApp/Cargo.toml#9%2C1-9%2C1) version 1 (with the [full](file:///c%3A/Users/Ashish%20Gupta/Desktop/Rust/FirstWebApp/Cargo.toml#9%2C38-9%2C38) feature)
- [warp](file:///c%3A/Users/Ashish%20Gupta/Desktop/Rust/FirstWebApp/Cargo.toml#10%2C1-10%2C1) version 0.3
- [serde_json](file:///c%3A/Users/Ashish%20Gupta/Desktop/Rust/FirstWebApp/Cargo.toml#11%2C1-11%2C1) version 1.0
- [cargo-watch](file:///c%3A/Users/Ashish%20Gupta/Desktop/Rust/FirstWebApp/Cargo.toml#12%2C1-12%2C1) version 8.4.1
