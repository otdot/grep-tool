# grep-like command-line tool

follows the guide in https://rust-cli.github.io/book/index.html

## Arguments
|Argument|Example|Note|
|--------|-------|----|
|Pattern |secret |pattern to look for|
|Path    |src/main|file where the pattern is searched from|

## Running and building

### Run from source
Use Cargo to compile and run directly in one step:

```bash
cargo run -- secret main.rs
```
 - secret is the search pattern.
 - src/main.rs is the file (or folder) path to search.

### Build a release binary
Create an optimized executable:

```bash
cargo build --release
```

The resulting binary is in:

 - target/release/grrs (Linux/macOS)
 - target/release/grrs.exe (Windows)

#### Run the binary
Execute the built binary with the same arguments:
```bash
target/release/grrs secret main.rs
```
