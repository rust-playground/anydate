# anydate &emsp; [![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/anydate.svg
[crates.io]: https://crates.io/crates/anydate

This crate is used to parse an unknown DateTime or Date format into a normalized version.

---

Any significant changes to Chrono are documented in
the [`CHANGELOG.md`](https://github.com/rust-playground/anydate/blob/main/CHANGELOG.md) file.

## Usage
```toml
[dependencies]
anydate = "0.1"
```

### Features

Optional features:

- [`serde`][]: Enable deserialize_with helper functions via serde.

[`serde`]: https://github.com/serde-rs/serde

### Example usages
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // see parse_utc() for convenience conversion to UTC
    let parsed = anydate::parse("2021-11-10T03:25:06.533447000Z");
    println!("{:#?}", parsed);
    Ok(())
}
```

or if you know it's only a date with no time component

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed = anydate::date::parse("2021-11-10");
    println!("{:#?}", parsed);
    Ok(())
}
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Proteus by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>