#!/bin/sh -e

cargo +nightly rustdoc --all-features -- --cfg docsrs
