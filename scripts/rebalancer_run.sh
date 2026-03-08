#!/bin/bash

PORTS=(3000 4000 5001)

NODES=$(IFS=,; echo "${PORTS[*]/#/localhost:}")
echo "Starting cluster_rebalancer (interactive)... with nodes: $NODES"
cargo run --bin cluster_rebalancer -- --nodes="$NODES"

cleanup