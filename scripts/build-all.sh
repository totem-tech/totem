#!/usr/bin/env bash

# This script should be run on Unix/Linux based systems
# This builds all totem chains for production

cargo build --release -p parachain-totem-kapex-node && \
cargo build --release -p parachain-totem-wapex-node && \
cargo build --release -p parachain-totem-lego-node