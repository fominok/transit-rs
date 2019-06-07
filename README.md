# transit-rs
Transit serialization format support

[Transit](https://github.com/cognitect/transit-format) is a format and set of libraries for
conveying values between applications written in different programming languages. This repository
is an attempt to provide an implementation for the [Rust](https://www.rust-lang.org/) language.

## How to use it
Like Serde, transit-rs defines and intermediate model --- a set of functions representing base types of Transit format
(all JSON types). This is needed as we need to abstract over three possible implementations (JSON, JSON Verbose and MessagePack).
If a type needs to be serializeable into Transit it needs to implement `TransitSerialize` trait.

## License
MIT

_Note: while deserialization works, the serialization part is covered much better and on "no-caching" branch.
Also marking types as composite or not probably will be done as intermediate return types. Thus, the project is still WIP._
