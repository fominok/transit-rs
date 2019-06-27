# transit-rs
Transit serialization format support (early WIP)

[Transit](https://github.com/cognitect/transit-format) is a format and set of
libraries for conveying values between applications written in different
programming languages. This repository is an attempt to provide an
implementation for the [Rust](https://www.rust-lang.org/) language.

## How to use it
Like Serde, transit-rs defines and intermediate model --- a set of functions
representing base types of Transit format (all JSON types). This is needed as
we need to abstract over three possible implementations (JSON, JSON Verbose
and MessagePack). If a type needs to be serializeable into Transit it needs
to implement `TransitSerialize` trait.

At this moment serialization works only with JSON Verbose serializer, also
there is a derive macro. Caching and deserialization are coming.
Examples are located in `ser/json_verbose.rs` file within `test` module.

## Roadmap

- [ ] Serialization
  - [x] API
  - [x] Standard types implementation
  - [x] JSON (Verbose)
    - [x] Serializer
    - [x] Derive macro
  - [ ] JSON (Non-verbose/caching)
    - [ ] Serializer
    - [ ] Derive macro
- [ ] Deserialization
  - [ ] API
  - [ ] Standard types implementation
  - [ ] JSON (Verbose)
    - [ ] Deserializer
    - [ ] Derive macro
  - [ ] JSON (Non-verbose/caching)
    - [ ] Deserializer
    - [ ] Derive macro
- [ ] MessagePack?


## License
MIT
