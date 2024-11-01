#!/bin/bash

# Build the Rust project first
cargo build

# Start each node in a new terminal window
gnome-terminal -- bash -c "cargo run --bin node1; exec bash"
gnome-terminal -- bash -c "cargo run --bin node2; exec bash"
gnome-terminal -- bash -c "cargo run --bin node3; exec bash"

