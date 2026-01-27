#!/bin/sh -e

BUILD_DIR=build

cmake -S . -B "$BUILD_DIR"
cmake --build "$BUILD_DIR"
ctest --test-dir "$BUILD_DIR" -T test --output-on-failure
