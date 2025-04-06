# MiniGrep

MiniGrep is a simple command-line tool written in Rust that replicates the basic functionality of the Unix `grep` command. It searches for lines containing a query string in a given file, with support for both case-sensitive and case-insensitive searches.

## 🚀 Features

- Search for a specific query in a text file
- Optional case-insensitive search via environment variable
- Clean and idiomatic Rust code
- Simple and readable CLI interface
- Unit tests for core functionality

## 🛠️ Usage

### 1. Build the project

```bash
cargo build --release
```
### 2. Run the project

```bash
cargo run -- <query> <file_path>
```

### 3. Case-insensitive search

To perform a case-insensitive search, set the IGNORE_CASE environment variable:

Linux / macOS:

```bash
IGNORE_CASE=1 cargo run -- <query> <file_path>
```
Windows (PowerShell):

```bash
$env:IGNORE_CASE=1
cargo run -- <query> <file_path>
```
🧪 Example

Given a file poem.txt:

Rust:
safe, fast, productive.
Pick three.
Trust me.

Case-sensitive search:

cargo run -- rust poem.txt

Output:

Rust:

Case-insensitive search:

```bash
IGNORE_CASE=1 cargo run -- rust poem.txt
```
Output:

Rust:
Trust me.

Redirecting output to a file

```bash
waleed@linux:~/Desktop/Embedded_Linux/08-Rust/01-Exercises/07-Exercise07/minigrep$ cargo run -- to poem.txt > output.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/minigrep to poem.txt`
```

✔️ This writes the result into output.txt as expected.

Now try missing the arguments:

waleed@linux:~/Desktop/Embedded_Linux/08-Rust/01-Exercises/07-Exercise07/minigrep$ cargo run > output.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/minigrep`
Problem parsing arguments: Not enough arguments

❌ Since the arguments are missing, the program prints an error message and exits. The output.txt file will remain empty because the error is printed to stderr, not stdout.
📂 Project Structure

src/
├── main.rs     # Entry point; handles CLI argument parsing
└── lib.rs      # Core logic (search functions, config builder, etc.)

🧠 How It Works

    Argument Parsing: main.rs collects CLI arguments and builds a Config object.

    File Reading: Reads the content of the file into a string.

    Search Logic:

        If IGNORE_CASE is set, performs a case-insensitive search.

        Otherwise, performs a regular case-sensitive search.

    Output: Prints the matching lines to standard output.

🧪 Running Tests

To run the included unit tests:

```bash
cargo test
```

📄 License

This project is open source and available under the MIT License.


✨ Credits

Built as a Rust learning exercise, inspired by the "minigrep" project in The Rust Programming Language book.