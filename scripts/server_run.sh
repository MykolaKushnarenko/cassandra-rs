#!/bin/bash

PORTS=(3000 4000 5001)
PIDS=()

cleanup() {
  echo ""
  echo "Stopping all servers..."
  for pid in "${PIDS[@]}"; do
    kill "$pid" 2>/dev/null && echo "Stopped PID $pid"
  done
  exit 0
}

trap cleanup SIGINT SIGTERM EXIT

for PORT in "${PORTS[@]}"; do
  cargo run --bin server -- port="$PORT" > /dev/null 2>&1 &
  PID=$!
  PIDS+=("$PID")
  echo "Starting server on port $PORT (PID: $PID)"
done

echo ""
echo "Waiting for servers to be ready..."
wait