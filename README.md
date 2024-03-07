# Statiker

A simple static file hosting server written in Rust.

# Setup

## Download from releases
You can download prebuild binaries from the release section.

## Build it yourself
In case you don't trust anyone, you can look up the source code and build it yourself.

Make sure you have setup rust and cargo correctly.

Proceed with the following steps:
```sh
git clone https://github.com/Padi2312/statiker.git
cd statiker
cargo build --release

# Run it with (or any different location where you build is)
./target/release/statiker 
```

# Usage 
```
Usage: statiker [OPTIONS] --dir <DIR>

Options:
  -d, --dir <DIR>          Sets the root directory to serve files from [default: "."]
  -a, --address <ADDRESS>  Sets the address to bind the server to [default: 0.0.0.0]
  -p, --port <PORT>        Sets the port to bind the server to [default: 8080]
  -h, --help               Print help
  -V, --version            Print version
```

# Project structure

The project is divided into two separate crates:
- `statiker`: The main crate containing the executable server for hosting files.
- `surfer`: Library containing the server logic. (Own repository: [surfer](https://github.com/Padi2312/surfer))