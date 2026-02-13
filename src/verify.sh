#!/bin/bash
# Verify all required files are present

echo "Checking Wappalyzer-Rust project files..."
echo ""

missing=0

# Check main files
if [ ! -f "Cargo.toml" ]; then
    echo "❌ MISSING: Cargo.toml"
    missing=$((missing+1))
else
    echo "✓ Found: Cargo.toml"
fi

# Check src directory
if [ ! -d "src" ]; then
    echo "❌ MISSING: src/ directory"
    missing=$((missing+1))
else
    echo "✓ Found: src/ directory"
fi

# Check required source files
required_files=(
    "src/main.rs"
    "src/lib.rs"
    "src/technologies.rs"
    "src/detector.rs"
)

for file in "${required_files[@]}"; do
    if [ ! -f "$file" ]; then
        echo "❌ MISSING: $file"
        missing=$((missing+1))
    else
        echo "✓ Found: $file"
    fi
done

echo ""
if [ $missing -eq 0 ]; then
    echo "✅ All required files are present!"
    echo ""
    echo "You can now run:"
    echo "  cargo build --release"
    exit 0
else
    echo "❌ $missing file(s) missing!"
    echo ""
    echo "Please make sure you downloaded ALL files from the package."
    echo "You need:"
    echo "  - Cargo.toml (in root)"
    echo "  - src/main.rs"
    echo "  - src/lib.rs"
    echo "  - src/technologies.rs"
    echo "  - src/detector.rs"
    exit 1
fi
