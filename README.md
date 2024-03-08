# Statiker

A simple static file hosting server wRiTtEn iN rUsT.

It provides a simple way to host files from a directory.

It also supports uploading files to the server using a simple web interface. 
Those files are saved to the root directory of the server and also served by the server. See more about this in the [Enable upload page](#enable-upload-page) section.

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
  -u, --enable_upload      Enables file upload at route '/' and saves files to root directory of server
  -h, --help               Print help
  -V, --version            Print version
```

# Enable upload page
To enable the upload page, you can use the `-u` or `--enable_upload` flag. This will enable the upload page at route `/upload`. 

All files uploaded will be saved to the root directory of the server and also served by the server.

> **NOTE:** In case you're uploading files with the same name, the old files will be overwritten by the new ones. (Currently working on this for a better solution.)


# Project structure

The project is divided into two separate crates:
- `statiker`: The main crate containing the executable server for hosting files.
- `surfer`: Library containing the server logic. (Own repository: [surfer](https://github.com/Padi2312/surfer))