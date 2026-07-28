# Rust Text Search CLI(grep)

A lightweight, fast, and simple command-line text searching utility written in Rust. It reads a specified file and outputs every line containing a given search query string.

---

--

## Prerequisites

Before running or building this project, make sure you have the Rust toolchain installed on your machine.

- **Rust & Cargo**: [Install Rust](https://www.rust-lang.org/tools/install) (Edition 2024 support required)

Verify installation:
```bash
rustc --version
cargo --version
```


---

## Usage

Run the program using `cargo run` followed by `--`, your `<search_query>`, and `<file_path>`:

```bash
cargo run -- <search_query> <file_path>
```

### Example

Searching for the word `hello` inside `test.txt`:

```bash
cargo run -- hello test.txt
```

**Output:**
```text
Raw arguments: ["target/debug/text-adventure", "hello", "test.txt"]
Searching for: hello
In file: test.txt
fwefewhellofwef
```

---

