#!/bin/bash

echo "Generating command types..."

# Default paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TMP_SCHEMA_DIR="$(mktemp -d)"
TYPES_DIR="${1:-$PWD/types}"  # <-- This uses the current working directory

# Ensure the output types directory exists
mkdir -p "$TYPES_DIR"

# Generate schemas in the temporary folder
cargo run --manifest-path src-tauri/Cargo.toml --bin generate_schemas "$TMP_SCHEMA_DIR"

# Generate TypeScript types from the temporary schema folder
node "$SCRIPT_DIR/generate-ts-cmd-types.js" "$TMP_SCHEMA_DIR" "$TYPES_DIR"

# Clean up the temporary schema folder
rm -rf "$TMP_SCHEMA_DIR"

echo "Command types generation completed."
