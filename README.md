<!-- @format -->

# Popcorn

- This is a open secure media project that will give you a real news

## Setup project

- install rust
- copy the project repository
- make a file called `.env` and set environtment variables as `.env.example`
  file
- after setting everything run `cargo run`

## Folder Description

#### main.rs

- Entry point of the application where the server initialized

#### lib.rs

- For exporting modules or functionality that might be shared across multiple
  binaries or test

#### config

- Contains configuration files and modules for setting up application-wide
  settings

#### api

- Defines the API routes

#### handlers

- Defines handlers for routes

#### services

- Contains business logic and operations related to various functionlity

#### Utils

- Contains utility functions and helper modules

#### middleware

- Define custom middleware for the server

#### models

- Define the data structures and database schemas

#### tests

- Contains test cases for the application
