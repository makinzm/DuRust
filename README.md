# DuRust

Du means Disk Usage. This is a simple tool to check disk usage of directories and files.

## Installation

```bash
cargo build --release
```

```bash
sudo cp target/release/durust /usr/local/bin
```

## Example

```bash
> durust

Current directory: .

[DIR] target - 62.23 MiB
[DIR] .git - 45.68 KiB
[FILE] Cargo.lock - 11.19 KiB
[DIR] src - 4.10 KiB
[FILE] README.md - 209 B
[FILE] Cargo.toml - 189 B
[FILE] .gitignore - 8 B

Select a directory to explore or enter `:q` to quit:
✔ Choose an option · [DIR] target

Current directory: ./target

[DIR] release - 62.23 MiB
[FILE] .rustc_info.json - 1.01 KiB
[FILE] CACHEDIR.TAG - 177 B

Select a directory to explore or enter `:q` to quit:
✔ Choose an option · [DIR] release

Current directory: ./target/release

[DIR] deps - 56.30 MiB
[DIR] build - 5.27 MiB
[FILE] durust - 643.35 KiB
[DIR] .fingerprint - 27.92 KiB
[FILE] durust.d - 91 B
[FILE] .cargo-lock - 0 B
[DIR] incremental - 0 B
[DIR] examples - 0 B

Select a directory to explore or enter `:q` to quit:
Exiting...
```
