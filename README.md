# linkchecker_rust

A simple Rust tool to check for broken links in plain text and Markdown files.

This project provides a basic link checker implemented in Rust. It scans input text for URLs and verifies whether they are reachable. Broken links are reported to help identify issues such as typos or removed resources.

---

## Installation

Build the project with Cargo:

```bash
cargo build --release

Run it with:

```linkchecker_rust path/to/file.md

Example:

```linkchecker_rust input.md
