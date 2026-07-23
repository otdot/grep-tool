# Command line interface password manager 

This is a local password manager that stores passwords in a file. For optimal setup the file should be encryped, but this tool doesn't include encrypting or decrypting the file. This tool should have support to define the schema of the credential objects and basic CRUD opertations. 

|Action|Command|Note|
|--------|-------|----|
|GET password | `pwtool get '<pattern>'` | GET a password, if multiple passwords match show password object keys and let user choose which password to copy|
|SET password | `pwtool set key, username, password ` | SET a password, if a matching key already exists return an error|


## Arguments
|Argument|Example|Note|
|--------|-------|----|
|Pattern |secret |pattern to look for|
|Config_path (optional) |src/main|file where the config file is searched from|

## Running and building

### Run from source
Use Cargo to compile and run directly in one step:

```bash
cargo run -- secret main.rs
```
 - secret is the search pattern.
 - src/main.rs is the config file path to search.

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
