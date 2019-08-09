# Climer
A simple CLI timer, written in Rust.

## As a CLI app
This project's main use is as a CLI app, but it can also be used as a crate / library.

### Installation
#### From [crates.io]
__(TODO: Not yet published)__
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

[crates.io]: https://crates.io/crates/climer
