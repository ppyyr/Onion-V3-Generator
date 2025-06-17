# Tor Onion V3 Address Generator (Rust)

<p align="center">
    <img src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
    <img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT">
</p>

## 📜 Description

This is a high-performance Rust implementation of a Tor .onion V3 address generator. It supports multi-threaded parallel processing for faster address generation and includes all the features of the original Python version with significant performance improvements.

### Key Features

- **Multi-threaded Processing**: Utilizes all available CPU cores for maximum performance
- **Configurable Workers**: Manually specify the number of worker threads
- **Real-time Statistics**: Live updates on generation progress
- **Interactive Mode**: Press Enter to see current statistics
- **Cross-platform**: Works on Linux, macOS, and Windows
- **Memory Efficient**: Optimized for low memory usage
- **Fast Cryptography**: Uses optimized Ed25519 implementation

## 🚀 Performance

The Rust implementation is significantly faster than the Python version:
- **~10-50x faster** address generation (depending on hardware)
- **Lower memory usage** due to Rust's zero-cost abstractions
- **Better CPU utilization** with native multi-threading

## 🛠️ Installation

### Prerequisites
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))

### Build from Source
```bash
git clone https://github.com/ppyyr/Onion-V3-Generator
cd onion-generator-rust
cargo build --release
```

### Install from Crates.io (when published)
```bash
cargo install onion-generator
```

## ⚙️ Usage

### Basic Usage
Generate addresses with specific prefixes:
```bash
./target/release/onion-generator github example
```

### Advanced Options

#### Specify Number of Workers
```bash
# Use 8 worker threads
./target/release/onion-generator -w 8 github example

# Use all available CPU cores (default)
./target/release/onion-generator github example
```

#### Single-threaded Mode
```bash
./target/release/onion-generator -s github example
```

#### Custom Update Interval
```bash
# Update statistics every 60 seconds
./target/release/onion-generator -u 60 github example
```

#### Help
```bash
./target/release/onion-generator --help
```

### Command Line Options

- `prefixes`: List of prefixes to search for (required)
- `-w, --workers <NUM>`: Number of worker threads (default: CPU cores)
- `-s, --single-threaded`: Run in single-threaded mode
- `-u, --update-interval <SECONDS>`: Statistics update interval (default: 30)
- `-h, --help`: Show help information
- `-V, --version`: Show version information

## 📊 Example Output

```
[@] Onion V3 Address Generator
[@] Searching for prefixes: ["github", "example"]
[@] Using 8 worker threads
[+] Worker 0 started
[+] Worker 1 started
...
[@] Started 8 worker threads
[@] Generating addresses...
[i] Press Enter to see the current status:

[@] 14:30:15: Generated 1250000 addresses, Found 0 addresses
[@] 14:30:45: Generated 2100000 addresses, Found 0 addresses
[√] Address generated successfully!
Hostname:                      github7x4f2k3m9n8p1q2r3s4t5u6v7w8x9y0z1a2b3c4d5e6f.onion
Public Key (Base64 encoded):   PT0gZWQyNTUxOXYxLXB1YmxpYzogdHlwZTAgPT0AAAA...
Private Key (Base64 encoded):  PT0gZWQyNTUxOXYxLXNlY3JldDogdHlwZTAgPT0AAAA...
```

## 🏗️ Architecture

The Rust implementation uses a multi-threaded worker pool architecture:

- **Main Thread**: Handles CLI parsing and coordinates workers
- **Worker Threads**: Generate addresses in parallel
- **Statistics Thread**: Periodically reports generation statistics
- **Input Monitor Thread**: Handles user input for real-time stats

## 🧪 Testing

Run the test suite:
```bash
cargo test
```

Run tests with output:
```bash
cargo test -- --nocapture
```

## 🔧 Development

### Debug Build
```bash
cargo build
./target/debug/onion-generator github
```

### Release Build (Optimized)
```bash
cargo build --release
./target/release/onion-generator github
```

### Benchmarking
```bash
cargo bench
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Original Python implementation by [joshuavanderpoll](https://github.com/joshuavanderpoll)
- Tor Project for the .onion address specification
- Rust community for excellent cryptography crates
