# rstls
Basic implementation of ls command written in Rust

## Build

```bash
cargo build --release
```

## Usage

```bash
./target/release/rstls [-a] [path]
```

### Examples
```bash
# List current directory
./target/release/rstls

# Show hidden files
./target/release/rstls -a

# Target a specific directory
./target/release/rstls -a src/
```

### Output Format
```text
-rw-r--r--      149 Cargo.lock
-rw-r--r--       76 Cargo.toml
-rw-r--r--      345 README.md
drwxr-xr-x       14 src
drwxr-xr-x       98 target
```
