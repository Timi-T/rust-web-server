# Rust-web-server

The rust-web-server is a project that utilizes rust and some of it's powerful features to create a static web server.

## Library crate installation

Using [cargo](https://docs.rs/cargo/latest/cargo/) install the package by;

1. Adding this line of code to Cargo.toml file...

   ```
   ope-rust-web-server = "0.1.1"
   ```

2. Navigating to your project directory and running this command..
   ```bash
   cargo add ope-rust-web-server
   ```

### Usage

The library API is properly documented at [doc.rs](https://docs.rs/ope-rust-web-server/0.1.1/server/).

## Source code installation

Simply clone from github and start the project on your local machine.

```bash
git clone git@github.com:Timi-T/rust-web-server.git
```

### Usage

```bash
# Navigate to project directory
cd rust-web-server

# compile the source code and execute binary.
cargo build && ./target/debug/rust-web-server [Options]
# or simply run
cargo run -- [options]

# Options include
#   --port -> Port to bind the server application to.
#     Example. (cargo run -- --port n). Where n is an integer.
#   --threads -> Number of threads required for the server.
#     Example. (cargo run -- --threads n). Where 0 < n < 101.
#   --dir-path -> Directory path to serve static files from.
#     Example. (cargo run -- --dir-path asset/files).
#.             Path can be relative or absolute to the current directory

# Run the command below for full documentation

cargo docs --open
```

Full Documentation can be gotten from [docs](https://docs.rs/ope-rust-web-server/0.1.0/server/).

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://github.com/Timi-T/rust-web-server/blob/main/LICENSE)
