# rust_web_server

Welcome to rust-web-server, a robust and efficient web server built with Rust that facilitates file uploads, automatic compression, and saves the files to a database. This server is designed to handle file management seamlessly, ensuring high performance and reliability. Whether you are working on an application that requires file handling or simply want to implement a secure and fast file server, this project has you covered.

---
## Features
- **File Upload**: Secure and easy-to-use file upload functionality.

- **Automatic Compression**: Files once uploaded can compressed to save space and reduce storage requirements.

- **Database Integration**: Saves uploaded and compressed files to a database for easy access and management.

- **High Performance**: Written in Rust, leveraging its memory safety and performance advantages.

- **Async Support**: Handles multiple concurrent uploads and operations with asynchronous processing.

---
## Getting Started

### Prerequisites
- **Rust** (Stable version recommended)

- **Cargo** (comes with Rust)

- A running **database** (e.g., PostgreSQL, MySQL) for storing file metadata.

### Installation
1. Clone the repository:
```bash 
git clone https://github.com/Guy-Ghis/rust_web_server.git
```
2. Build the project:
```bash
cargo build --release
```
3. Run the server:
```bash
cargo run
```
Your server will start running on http://localhost:8000.

---
## Contributing
We are open to contributions from developers of all levels! Whether it's fixing bugs, suggesting new features, or improving documentation, feel free to open an issue or submit a pull request. Here’s how you can contribute:

1. Fork the repository.
2. Clone your fork to your local machine:
```bash
git clone https://github.com/Guy-Ghis/rust_web_server.git
```
3. Create a new branch for your feature or fix:
```bash
git checkout -b my-feature
```
4. Make your changes and commit them:
```bash
git commit -m "Add new project feature"
```
5. Push your changes to your fork:
```bash
git push origin my-feature
```
6. Submit a pull request to the main repository.

---
## License
This repository is licensed under the MIT License. See the LICENSE file for more information.
