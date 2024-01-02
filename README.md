# Project README

## Description
This project is a web application built in Rust using the [warp](file:///c%3A/Users/Ashish%20Gupta/Desktop/Rust/FirstWebApp/Cargo.toml#10%2C1-10%2C1) framework. It provides APIs for managing todos and serves static content.

## Getting Started
To start the application, follow these steps:

1. Clone the repository.
2. Make sure you have Rust and Cargo installed.
3. Navigate to the project directory.
4. Install the `cargo-watch` crate by running the following command:
   ```
    cargo install cargo-watch
   ```
5. Run the following command to start the application:
  ```
  cargo run
  ```
  or
  ```
  cargo watch -x run
  ```

6. The application will start running on `http://127.0.0.1:1313`.

## Test Apis
1. Install the REST Client extension for VS Code. This extension allows you to send HTTP requests directly from VS Code       using the `dev.http` file.
2. Navigate to the project directory.
3. Run the following command to start the application:
   ```
  cargo run
  ```
  or
  ```
  cargo watch -x run
  ```
4. Open the `dev.http` file. Inside this file you will `Send Request`. Click on that to make API request. 

## APIs
The application provides the following APIs:

### Create Todos
- Endpoint: `/todos`
- Method: POST
- Headers:
  - [X-Auth-Token](file:///c%3A/Users/Ashish%20Gupta/Desktop/Rust/FirstWebApp/dev.http#4%2C1-4%2C1): The authentication token
- Example Request:
  http
  ```
  POST http://127.0.0.1:1313/todos
  X-Auth-Token: 123.exp.signature
  Content-Type: application/json

  {
    "id":5,
    "title":"Todo test"
  }
  ```
- Example Response:
  ```js
  HTTP/1.1 200 OK
  content-type: application/json
  content-length: 28
  date: Tue, 02 Jan 2024 11:07:59 GMT
  
  {
    "id": 5,
    "title": "Todo test"
  }
  ```

### Get Todos
- Endpoint: `/todos/{id}`
- Method: GET
- Headers:
  - [X-Auth-Token](file:///c%3A/Users/Ashish%20Gupta/Desktop/Rust/FirstWebApp/dev.http#4%2C1-4%2C1): The authentication token
- Example Request:
  http
  ```
  GET http://127.0.0.1:1313/todos/123
  X-Auth-Token: 123.exp.signature
  Content-Type: application/json
  ```
- Example Response:
  ```js
  HTTP/1.1 200 OK
  content-type: application/json
  content-length: 97
  date: Tue, 02 Jan 2024 11:06:22 GMT

  [
    {
      "completed": true,
      "id": 1,
      "task": "Learn Rust"
    },
    {
      "completed": false,
      "id": 2,
      "task": "Learn GraphQL"
    }
  ]
  ```

### Get Todos
- Endpoint: `/todos/{id}`
- Method: GET
- Headers:
  - [X-Auth-Token](file:///c%3A/Users/Ashish%20Gupta/Desktop/Rust/FirstWebApp/dev.http#4%2C1-4%2C1): The authentication token
- Example Request:
  http
  ```
  GET http://127.0.0.1:1313/todos/123
  X-Auth-Token: 123.exp.signature
  Content-Type: application/json
  ```
- Example Response:
  ```js
  HTTP/1.1 200 OK
  content-type: application/json
  content-length: 45
  date: Tue, 02 Jan 2024 11:07:03 GMT
  
  {
    "completed": true,
    "id": 1,
    "task": "Learn Rust"
  }
  ```

### Hi API
- Endpoint: `/hi`
- Method: GET
- Example Request:
  http
  ```
  GET http://127.0.0.1:1313/hi
  X-Auth-Token: 123.exp.signature
  Content-Type: application/json
  ```
- Example Response:
  ```
  HTTP/1.1 200 OK
  content-type: text/plain; charset=utf-8
  content-length: 10
  date: Tue, 02 Jan 2024 11:11:37 GMT
  
  Hi, World!
  ```

## Static Content
The application serves static content from the [web-folder](file:///c%3A/Users/Ashish%20Gupta/Desktop/Rust/FirstWebApp/src/main.rs#8%2C27-8%2C27) directory. The root URL (`http://127.0.0.1:1313/`) serves the `index.html` file from the [web-folder](file:///c%3A/Users/Ashish%20Gupta/Desktop/Rust/FirstWebApp/src/main.rs#8%2C27-8%2C27).

## Dependencies
The project uses the following dependencies:

- [tokio](file:///c%3A/Users/Ashish%20Gupta/Desktop/Rust/FirstWebApp/Cargo.toml#9%2C1-9%2C1) version 1 (with the [full](file:///c%3A/Users/Ashish%20Gupta/Desktop/Rust/FirstWebApp/Cargo.toml#9%2C38-9%2C38) feature)
- [warp](file:///c%3A/Users/Ashish%20Gupta/Desktop/Rust/FirstWebApp/Cargo.toml#10%2C1-10%2C1) version 0.3
- [serde_json](file:///c%3A/Users/Ashish%20Gupta/Desktop/Rust/FirstWebApp/Cargo.toml#11%2C1-11%2C1) version 1.0
- [cargo-watch](file:///c%3A/Users/Ashish%20Gupta/Desktop/Rust/FirstWebApp/Cargo.toml#12%2C1-12%2C1) version 8.4.1
