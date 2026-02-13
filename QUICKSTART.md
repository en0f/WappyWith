# QUICK START

## What You Need

**Rust 1.83 or newer** - This is critical!

## What This Does

Detects **100+ web technologies** across 20+ categories - similar to BuiltWith.com! 
See [TECHNOLOGIES.md](TECHNOLOGIES.md) for the complete list.

## Installation

```bash
# 1. Update Rust (IMPORTANT!)
rustup update stable
rustc --version  # Must be 1.83+

# 2. Navigate to project
cd wappalyzer-rust-final

# 3. Build
cargo build --release

# 4. Run
cargo run -- https://example.com
```

## The Build Error You Encountered

The error "couldn't read `src/lib.rs`" happens when:
1. You're not in the correct directory
2. The source files weren't extracted properly
3. You're running `cargo build` from the wrong location

## Solution

Make sure you have all these files:

```
wappalyzer-rust-final/
├── Cargo.toml           ← Config file
├── src/
│   ├── main.rs          ← Must exist
│   ├── lib.rs           ← Must exist  
│   ├── technologies.rs  ← Must exist
│   └── detector.rs      ← Must exist
└── README.md
```

Run this to check:

```bash
ls -la src/
```

You should see all four .rs files.

## If It Still Doesn't Work

1. **Check Rust version**: `rustc --version` (must be 1.83+)
2. **Check you're in the right directory**: `pwd` and `ls Cargo.toml`
3. **Check all files exist**: `ls src/*.rs` (should show 4 files)
4. **Clean and rebuild**: `cargo clean && cargo build --release`

## Full Documentation

- **README.md** - Complete documentation
- **INSTALL.md** - Detailed installation guide with troubleshooting

## Example Usage

```bash
# Basic
cargo run -- https://wordpress.org

# Verbose (shows more details)
cargo run -- https://react.dev --verbose

# JSON output
cargo run -- https://github.com --json
```
