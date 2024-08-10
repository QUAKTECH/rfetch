#!/bin/bash

echo "Building project..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "Build failed. Exiting."
    exit 1
fi

if [ ! -f target/release/rfetch ]; then
    echo "Error: target/release/rfetch not found. Exiting."
    exit 1
fi

echo "Copying rfetch to /usr/bin..."
sudo cp target/release/rfetch /usr/bin/
if [ $? -ne 0 ]; then
    echo "Failed to copy rfetch to /usr/bin/. Exiting."
    exit 1
fi

echo "rfetch has been installed.🔥"