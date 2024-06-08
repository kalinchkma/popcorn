<!-- @format -->

# Usefull functions of Actix-Web

- `App`: instance is used for registering routes for resources and
  middlerware.It also stores application state and shared across all handlers
  within the same scope
- `App::service`:for the handlers which is using routing macros.
- `App::route`: for manual routes handler.

- `web::scope`: used for wrape namespace for route groupe.
- `state`: application state is shared with all routes and resources within the
  same scope. state can be accessed with the `web::Data<T>` extractor. state is
  also accessible for middleware.
- `Shared mutable state`: `HttpServer` accepts an application factory rather
  than an application instance. `HttpServer` constructs an application instance
  for each thread. If we want to use mutable shared state we have to wrap our
  state with `Arc` to avoid creating two `Arc`s
- `Application scope`: The `web::scope()` method allows settings a resource
  group prefix.
- `Application guards`:
- `Configure`: For simplicity and reuseability both `App` and ``web::Scope`
  provide the `configure` method .

# Details of actix-web

### Application:

- App
- Scope
- Gurds
- State
- Configuration

### Server

- Multi-Threading
- TLS/HTTPS supports
- Keep Alive
- Graceful shutdown

### Extractor

- Path
- Path dynamic segments
- Query
- Json
- URL-Encoded Froms
- Application state extractor
- Data
- HttpRequest
- Bytes
- Payload

### Request handler

- Request Handlers
- Response with custom type
- Streaming response body

### Errors

- Custom error repose
- Error helpers
- Error logging

### URL Dispatch

- Resource configuration
- Configure a route
- Route matching
- Resource pattern syntax
- Scoping Routes
- Match information
- Path information extractor
- Generating resource URLs
- External resources
- Path normalization and redirecting to slash-appended routes
- Custom route guard

### Requests

- Content Encoding
- Chunked transfer encoding
- multipart body
- urlencoded body
- streaming request

### Responses

- JSON Response
- Content encoding

### Testing

- Integration Testing For Applications
- Stream Response Testing
- Unit Testing Extractors

### Middleware

- Pre-process the request
- Post-process the request
- Modify application state
- Access external services
- Logging

### Static files

- Individual file
- Directory
- Configuration

### Protocols

- Websockets
- HTTP/2

### Auto-Reloading Development Server

- Its done using cargo-watch

### Databases

- Postgres
- SQLite
- MongoDB
