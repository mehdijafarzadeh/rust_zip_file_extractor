# ZIP File Extractor (Rust)

This project is a simple command-line utility written in Rust for extracting ZIP archives. It reads a ZIP file from the given path and extracts all files and directories contained within, handling any file comments, permissions, and directory structures. 

## Features

- Extracts all files and directories from a ZIP archive.
- Preserves original directory structures.
- Supports extraction of file comments if present.
- Ensures security by preventing directory traversal attacks (e.g., files with `../` paths).
- Sets Unix file permissions if running on Unix-like systems.

## Usage

### Command-line Arguments

This program expects a single command-line argument:  
1. `<filename>`: The path to the ZIP file that needs to be extracted.

### Example

```bash
cargo run -- my_archive.zip
