rgrep 🦀
A lightweight, high-performance command-line utility built in Rust to search for regex patterns within files. This tool is designed to be memory-efficient by leveraging stream buffering and heap-allocation reuse.
🚀 Features
Regex Support: Full regular expression support via the regex crate.
Memory Efficient: Uses a single reusable buffer for line reading, making it capable of processing large files without high memory overhead.
Robust Error Handling: Propagates errors using Box<dyn Error> for clear, actionable feedback.
CLI Power: Built with clap for a polished interface, including automatic --help and --version flags.
🛠 Installation
Ensure you have Rust and Cargo installed, then:
bash
git clone https://github.com
cd rgrep
cargo build --release
Use code with caution.
The binary will be available at ./target/release/rgrep.
📖 Usage
bash
rgrep --path <FILE_PATH> --keyword <REGEX_PATTERN>
Use code with caution.
Examples
Search for a simple string:
bash
./rgrep -p log.txt -k "ERROR"
Use code with caution.
Search using a regex pattern (e.g., finding all emails):
bash
./rgrep -p users.csv -k "[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,4}"
Use code with caution.
🏗 Built With
Rust - Programming language
Clap - Command Line Argument Parser
Regex - Regular expression engine
