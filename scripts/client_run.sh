#!/bin/bash

PORTS=(3000 4000 5001 6001)

NODES=$(IFS=,; echo "${PORTS[*]/#/localhost:}")
echo "Starting client (interactive)... with nodes: $NODES"
cargo run --bin client -- --nodes="$NODES"

cleanup