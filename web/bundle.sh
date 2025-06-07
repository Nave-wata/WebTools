#!/bin/bash

CARGO_FILE="Cargo.toml"
CARGO_PRD_FILE="Cargo-prd.toml"
CARGO_BACKUP_FILE="Cargo.toml.bak"

if [ ! -f "$CARGO_FILE" ]; then
    echo "Error: $CARGO_FILE file not found"
    exit 1
fi

if [ ! -f "$CARGO_PRD_FILE" ]; then
    echo "Error: $CARGO_PRD_FILE file not found"
    exit 1
fi

if [ -f "$CARGO_BACKUP_FILE" ]; then
    echo "Error: $CARGO_BACKUP_FILE file already exists"
    exit 1
fi

if [ -d "target/dx/WebTools/release/web" ]; then
    rm -rf target/dx/WebTools/release/web
fi

mv $CARGO_FILE $CARGO_BACKUP_FILE
cp $CARGO_PRD_FILE $CARGO_FILE

dx bundle \
  --features production \
  --platform web \
  --ssg \
  --release

mv $CARGO_BACKUP_FILE $CARGO_FILE
