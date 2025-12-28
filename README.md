# ini-piyo

> **Warning**: This tool is under development and may contain several bugs.

A command-line tool for merging INI configuration files. It compares a base INI file with a target INI file and can check for missing keys or merge missing keys from the base into the target.


## Features

- **Check Mode**: Identifies missing keys and empty values in the target file compared to the base file.
- **Merge Mode**: Adds missing keys from the base file to the target file.
- Supports standard INI file format with sections and key-value pairs.

## Installation

### From Precompiled Binaries
Download the latest release from the [GitHub Releases](https://github.com/OhashiReon/ini-piyo/releases) page and follow the instructions for your operating system.

### From Source

Ensure you have Rust installed. Then, clone the repository and build:

```bash
git clone https://github.com/OhashiReon/ini-piyo.git
cd ini-piyo
cargo build --release
```

The binary will be available at `target/release/ini-piyo`.

## Usage

```
ini-piyo <base_file> <target_file> [--check|--merge]
```

### Options

- `--check` (default): Check for missing keys and empty values in the target file.
- `--merge`: Merge missing keys from the base file into the target file.

### Examples

#### Check Mode

```bash
ini-piyo base.ini target.ini --check
```

This will output:
- `+` for missing keys that would be added.
- `*` for keys with empty values in the target but non-empty in the base.
- Regular lines for existing keys.

#### Merge Mode

```bash
ini-piyo base.ini target.ini --merge
```

This will add missing keys from `base.ini` to `target.ini` and overwrite the target file.

## Sample Output

For a check operation:

```
+  | key1=value1
*  | key2=
   | key3=value3

Check Complete.
 - Missing keys : 1
 - Empty vals   : 1
```

## Dependencies

- [colored](https://crates.io/crates/colored) for colored output.
- [indexmap](https://crates.io/crates/indexmap) for ordered maps.

## License

This project is licensed under the MIT License.