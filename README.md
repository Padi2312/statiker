# Statiker

A simple static file hosting server wRiTtEn iN rUsT.

It provides a simple way to host files from a directory.

It also supports uploading files to the server using a simple web interface. \
Those files are saved to the root directory of the server and also served by the server. 

# Setup

## Docker
You can use the docker image to run the server.

```sh
docker run -d -p 8080:8080 -v /path/to/your/files:/app/public padi2312/statiker
```

## Docker Compose
You can also use docker-compose to run the server.
Have a look at the  `docker-compose.yml` file in the repository.

## Binary
You can download prebuild binaries from the release section.

## Build from source
In case you don't trust anyone, you can look up the source code and build it yourself.

Make sure you have setup rust and cargo correctly.

Proceed with the following steps:
```sh
git clone https://github.com/Padi2312/statiker.git
cd statiker
cargo build --release

./statiker/target/release/statiker 
```

# Usage 
```
Usage: statiker [OPTIONS] --dir <DIR>

Options:
  -d, --dir <DIR>          Sets the root directory to serve files from [default: "./public"]
  -a, --address <ADDRESS>  Sets the address to bind the server to [default: 0.0.0.0]
  -p, --port <PORT>        Sets the port to bind the server to [default: 8080]
  -u, --enable_upload      Enables file upload at route '/upload' and saves files to root directory of server
  -h, --help               Print help
  -V, --version            Print version
```

# Environment variables 
Environment variables are used if no command line arguments are provided.

| Option        | Description                                                                        | Default    |
| ------------- | ---------------------------------------------------------------------------------- | ---------- |
| ROOT_DIR      | Sets the root directory to serve files from                                        | "./public" |
| ADDRESS       | Sets the address to bind the server to                                             | "0.0.0.0"  |
| PORT          | Sets the port to bind the server to                                                | 8080       |
| ENABLE_UPLOAD | Enables file upload at route '/upload' and saves files to root directory of server | false      |
