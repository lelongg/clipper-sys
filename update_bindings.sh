#!/bin/sh

set -eux

cd bind
cargo run > ../generated/bindings.rs
