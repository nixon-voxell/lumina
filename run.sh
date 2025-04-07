#!/bin/bash

# Check if an argument is provided
if [ -z "$1" ]; then
  echo "Please provide the number of client instances as an argument."
  exit 1
fi

echo "Building..."
cargo build

count=$1

echo "Starting the server..."
cargo run --bin lumina_server &

sleep 1

echo "Starting $count client instances..."
for ((i = 1; i <= count; i++)); do
  echo "Starting client instance $i..."
  cargo run --bin lumina_client &
  sleep 1
done

echo "All client instances are running."

