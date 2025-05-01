#!/usr/bin/env bash

set -e

APP_NAME=lemp

cargo build --release

echo

if [ -f "target/release/$APP_NAME" ]; then
    sudo rm -rf "/usr/local/bin/$APP_NAME"
    sudo cp "target/release/$APP_NAME" "/usr/local/bin"
    echo "Copied to /usr/local/bin"
else
    echo "App binary does not exist"
fi

echo