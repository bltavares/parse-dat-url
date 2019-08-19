# parse-dat-url

<a href="https://docs.rs/parse-dat-url">
 <img src="https://docs.rs/parse-dat-url/badge.svg?version=0.1.0" />
</a>
<a href="https://crates.io/crates/parse">
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

### Example

```rust
use parse_dat_url::DatUrl;

fn main() {
    let url = DatUrl::parse("dat://584faa05d394190ab1a3f0240607f9bf2b7e2bd9968830a11cf77db0cea36a21+v1.0.0/path/to/file.txt");
}
```
