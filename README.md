# Fastly-worker-template

Simple template project created as a way to bootstrap a new Fastly compute@edge RESTful API worker, without having to write all the boilerplate.
Written in Rust, because it's just better 🦀

Includes a simple router module, and a utils module that handles environment variable deserialization into a simple `AppConfig` struct, as well as a logger that can interface with any industry-standard logging dashboard via http.
