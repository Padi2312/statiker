<p align="center">
  <img src="./docs/statiker_logo.png" alt="Surfer Logo" width="200">
</p>

<h1 align="center">Statiker</h1>

<p align="center">
  <strong>A simple, efficient static file hosting server wRiTtEn iN rUsT.</strong>
</p>

Statiker provides a simple and efficient solution for hosting files from any directory.📁️ It also offers a user-friendly web interface for easy file uploads.🌐 Uploaded files are saved to the root directory and served by the server.

## 🚀 Features
- **Simple**: Statiker is designed to be simple and easy to use.
- **Efficient**: It's built with Rust, so it (_should be_) fast and efficient.
- **File Uploads**: Statiker supports file uploads with a simple web interface.
- **Docker Support**: Run Statiker using Docker or Docker Compose.
- **Cross-Platform**: Statiker is available as a pre-built binary for Windows, ~~macOS~~, and Linux.
- **Environment Variables**: Use environment variables to configure Statiker.
- **Command Line**: Use the command line to run Statiker.
 

## 🛠️ Setup

### 🐳 Docker
Docker Hub: <https://hub.docker.com/r/padi2312/statiker>

Run Statiker using Docker with the following command:

```sh
docker run -d -p 8080:8080 -v /path/to/your/files:/app/public padi2312/statiker
```

### 📦 Docker Compose
Alternatively, deploy Statiker using `docker-compose`. Check out the `docker-compose.yml` file in the repository for guidance.

### 📥 Binary
Download pre-built binaries from the releases section for a quick start.

### 🔨 Build from Source
For those who prefer to build from source, ensure Rust and Cargo are properly set up, then follow these steps:

```sh
git clone https://github.com/statiker/statiker.git
cd statiker
cargo build --release

./target/release/statiker
```

## 📚 Usage

To get started, use the following command structure:

```
statiker [OPTIONS] --dir <DIR>
```

Options include:
- `-d, --dir <DIR>`: Set the root directory for file serving (default: "./public").
- `-a, --address <ADDRESS>`: Specify the server's binding address (default: 0.0.0.0).
- `-p, --port <PORT>`: Set the server's port (default: 8080).
- `-u, --enable_upload`: Enable file uploads at '/upload', saving files to the server's root directory.
- `-h, --help`: Display help information.
- `-V, --version`: Show the version number.


## ⚙️ Environment Variables

When not using command line arguments, Statiker will revert to these environment variables:

| Variable        | Description                                                    | Default    |
| --------------- | -------------------------------------------------------------- | ---------- |
| `ROOT_DIR`      | Root directory for file serving                                | "./public" |
| `ADDRESS`       | Server's binding address                                       | "0.0.0.0"  |
| `PORT`          | Server's port                                                  | 8080       |
| `ENABLE_UPLOAD` | Enable '/upload' for file uploads, saving to the server's root | false      |
