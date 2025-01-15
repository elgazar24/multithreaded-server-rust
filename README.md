# Multithreaded Server

## Description

This project is a simple multi-threaded server-client application written in Rust. The server handles incoming requests and processes them concurrently using multiple threads. The client is responsible for establishing connections to the server, sending HTTP requests, and receiving responses.

The project also includes various tests to simulate multiple client connections, stress test the server, and verify proper request handling and server shutdown. The client is designed to be reusable across tests to simplify testing logic.

## Features

- Multi-threaded server handling multiple client connections.
- Implemented compatible SOLID principle.
- Client implementation for sending HTTP requests and receiving responses.
- Router basic implementation for managing the requests and send proper response.
- Worker to manage the threading mechanism 
- Worker Task to manage the task 
- Automated tests for server functionality:
  - Stress testing with many clients.
  - Handling invalid requests.
  - Handling raw messages and bounce them back to the client
  - Sending multiple requests to the server.
  - Simulating simultaneous client connections.
  - Server shutdown handling after processing requests.

## Setup and Installation

1. To get started with this project, clone the repository and follow the instructions below.
2. To start the server after installation run 'cargo build' then 'cargo run' in your terminal 
3. To test the functionalty run 'cargo test' in your terminal 


### Prerequisites

- Rust programming language (installed via [rustup](https://rustup.rs/)).
- Clone the repository from : https://github.com/elgazar24/multithreaded-server-rust.git 

