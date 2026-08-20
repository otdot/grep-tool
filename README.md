# Command line password manager 

This is a local password manager that stores passwords in a file. For optimal setup the file should be encryped, but this tool doesn't include encrypting or decrypting the file. This tool should have support to define the schema of the credential objects and basic CRUD opertations. 

|Action|Command|Note|
|--------|-------|----|
|GET password | `pwtool --c ./path-to-config-file get --P <pattern>` | GET a password, if multiple passwords match show password object keys and let user choose which password to copy|
|SET password | `pwtool --c ./path-to-config-file set --k <key> --u <user>  --p <password>` | SET a password, if a matching key already exists return an error|


## Arguments
|Argument|flag|Example|Note|
|--------|-------|----|
|Config_path (optional) |--c, --config_path| `pwtool --c ./config.toml get --P secret` | file where the config file is searched from. Defaults to .pwtool/config.toml.|
|Pattern |--P, --pattern| `pwtool get --P secret` |pattern to look for|
|Password entry fields (key*, url, username, password, note) (*=required) |--k, --key, --url, --u, --username, --p, --password, --n, --note| `pwtool set --k <key> --url <url> --u <username> --p <password> --n <note>` |fields for a new password entry|

## Running and building

### Run from source
Use Cargo to compile and run directly in one step:

```bash
cargo run -- get --P my-pattern
```

### Build a release binary
Create an optimized executable:

```bash
cargo build --release
```

The resulting binary is in:

 - target/release/pwtool (Linux/macOS)
 - target/release/pwtool.exe (Windows)

#### Run the binary
Execute the built binary with the same arguments:
```bash
target/release/grrs --c ./config-path.toml set --k my-key --u  my-username --p my-password
```