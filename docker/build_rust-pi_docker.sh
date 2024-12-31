#!/usr/bin/env bash

SCRIPT_DIR=$(dirname "$(realpath "$0")")
echo "The script is located in: $SCRIPT_DIR"

pushd "$SCRIPT_DIR"

# docker buildx build -f Dockerfile.cross --tag cross-stretch .
# docker buildx build -f Dockerfile.cross-pi --tag cross-pi .
docker buildx build --no-cache -f Dockerfile --tag rust-pi .
popd
