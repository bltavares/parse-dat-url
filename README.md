# parse-dat-url

<a href="https://docs.rs/parse-dat-url">
 <img src="https://docs.rs/parse-dat-url/badge.svg?version=0.1.0" />
</a>
<a href="https://crates.io/crates/parse-dat-url">
  <img src="https://img.shields.io/crates/v/parse-dat-url" />
</a>

url parser to support versioned [dat](https://dat.foundation) URLs

Useful links:

- [dat.foundation](https://dat.foundation) - Main webpage
- [How dat works](https://datprotocol.github.io/how-dat-works/) - Detailed Guide
- [datprocol](https://github.com/datprotocol) - Main implementation
- [datrs](https://github.com/datrs/) - Rust implementation

## Usage

```toml
[dependencies]
parse-dat-url = "0.1.0"
```

It is possible to avoid pulling `serde` as a dependency, by disabling default features:

```toml
[dependencies]
parse-dat-url = { version = "0.1.0", default-features = false }
```

### Example

```rust
use parse_dat_url::DatUrl;

fn main() {
    let url = DatUrl::parse("dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1.0.0/path/to/file.txt");
}
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT> )

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
