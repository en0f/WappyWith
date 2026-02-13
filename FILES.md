# FILE CHECKLIST

Before building, make sure you have ALL these files:

## Root Directory Files

```
wappalyzer-rust-final/
├── Cargo.toml           ← Configuration file (REQUIRED)
├── README.md            ← Documentation
├── INSTALL.md           ← Installation guide
├── QUICKSTART.md        ← Quick start guide
├── FILES.md             ← This file
├── verify.sh            ← Verification script (run this!)
└── .gitignore           ← Git ignore file
```

## Source Files (CRITICAL - ALL REQUIRED)

```
src/
├── main.rs              ← CLI entry point (REQUIRED)
├── lib.rs               ← Core library (REQUIRED)
├── technologies.rs      ← Technology database (REQUIRED)
└── detector.rs          ← Pattern matching (REQUIRED)
```

## Verification Steps

1. **Check you have all files:**

   On Linux/Mac:
   ```bash
   chmod +x verify.sh
   ./verify.sh
   ```

   On Windows:
   ```bash
   dir /b src
   # Should show: detector.rs, lib.rs, main.rs, technologies.rs
   ```

2. **Manually verify:**
   ```bash
   ls -la src/
   # You should see 4 .rs files
   ```

## If Files Are Missing

The error "file not found for module" means you're missing these files:

- `src/technologies.rs` - Contains the technology detection database
- `src/detector.rs` - Contains pattern matching logic
- `src/lib.rs` - Core library code
- `src/main.rs` - Command-line interface

**Solution**: Download ALL files from the package, not just some of them.

## File Sizes (approximate)

To verify you downloaded complete files:

- `Cargo.toml` - ~400 bytes
- `src/main.rs` - ~2.3 KB
- `src/lib.rs` - ~6.4 KB
- `src/technologies.rs` - ~9.6 KB
- `src/detector.rs` - ~1.7 KB

If your files are much smaller (like only a few bytes), they didn't download completely.

## Common Issues

### "file not found for module technologies"
→ You're missing `src/technologies.rs`

### "file not found for module detector"  
→ You're missing `src/detector.rs`

### "couldn't read src/lib.rs"
→ You're either in the wrong directory or `src/lib.rs` is missing

### How to Fix
1. Make sure you're in the `wappalyzer-rust-final` directory
2. Run `ls src/` and verify you see all 4 .rs files
3. If files are missing, download the complete package again
4. Make sure to extract/copy ALL files, including the `src/` subdirectory

## Ready to Build?

Once all files are present:

```bash
# Verify files
./verify.sh  # or check manually

# Update Rust
rustup update stable

# Build
cargo build --release

# Test
cargo run -- https://example.com
```
