# rgrep 🦀

A lightweight, high-performance command-line utility built in Rust to search for regex patterns within files. This tool is designed to be memory-efficient by leveraging stream buffering and heap-allocation reuse.

## 🚀 Features

- **Regex Support**: Full regular expression support via the `regex` crate.
- **Memory Efficient**: Uses a single reusable buffer for line reading, making it capable of processing large files without high memory overhead.
- **Robust Error Handling**: Propagates errors using `Box<dyn Error>` for clear, actionable feedback.
- **CLI Power**: Built with `clap` for a polished interface, including automatic `--help` and `--version` flags.

## 🛠 Installation

Ensure you have [Rust and Cargo](https://rustup.rs) installed, then:

```bash
git clone https://github.com
cd rgrep
cargo build --release
```

The binary will be available at `./target/release/rgrep`.

## 📖 Usage

```bash
rgrep --path <FILE_PATH> --keyword <REGEX_PATTERN>
```

### Examples

Search for a simple string:
```bash
./rgrep -p log.txt -k "ERROR"
```

Search using a regex pattern (e.g., finding all emails):
```bash
./rgrep -p users.csv -k "[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,4}"
```

## 🏗 Built With

- [Rust](https://rust-lang.org) - Programming language
- [Clap](https://docs.rs) - Command Line Argument Parser
- [Regex](https://docs.rs) - Regular expression engine
