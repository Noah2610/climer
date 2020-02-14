# Climer
A simple CLI timer, written in Rust.

## CLI app
This project's main use is as a CLI app, but it can also be used as a crate / library.

### Installation
#### From [crates.io]
```
cargo install climer
```
#### From source
You will need `cargo` to compile from source, which is shipped with Rust.
```
git clone https://github.com/Noah2610/climer # Clone the repository
cargo install --path ./climer                # Compile and install
```

### Usage
For the very basic timing feature, you can run something like:
```
climer 2m 30s
```
This will run the timer and print a readable representation of the remaining time to `stdout`.  
When the time is up, the app will simply exit with exit code `0`,  
so it is up to you to then play an alarm sound or whatever you may want it to trigger;  
for example:
```
climer 1h 2m 30s && mpv ~/Music/alarm.mp3
```

For more detailed usage information, check out the help page with:
```
climer -h      # Brief help
climer --help  # More detailed help
```

## Library crate
In your `Cargo.toml` ...
```toml
# TODO: version
[dependencies]
climer = { version = "*", default-features = false }
```
See below for available features.

### Compilation features
| Name        | Description | Default? |
|:----------- |:----------- |:--------:|
| `cli`       | Required for the binary app. You should disable this for library crates | `true` |
| `serialize` | Adds `serde` dependency and implements `Serialize` and `Deserialize` for `time::Time` | `false` |
| `parser`    | Adds `regex` dependency and adds `time::parser` module with functions for parsing a time from a given string | <small>enabled by `cli` feature</small> |

### Documentation
Documentation should be available at [docs.rs/climer][docs].

## License
[MIT License][license]

[license]:   ./LICENSE
[crates.io]: https://crates.io/crates/climer
[docs]:      https://docs.rs/climer
