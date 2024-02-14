#!/bin/sh
TARGET=aarch64-unknown-linux-gnu
# TARGET=armv7-unknown-linux-gnueabihf
cross build --release --target $TARGET
