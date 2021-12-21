#!/usr/bin/env bash

# This script should be run on Unix/Linux based systems
# This is likely only to be required to run once but can be repurposed for other parachains.

./target/release/parachain-totem-kapex-node export-genesis-state > ./res/genesis-states/kapex-genesis-state && \
./target/release/parachain-totem-wapex-node export-genesis-state > ./res/genesis-states/wapex-genesis-state && \
./target/release/parachain-totem-lego-node export-genesis-state > ./res/genesis-states/lego-genesis-state && \
./target/release/parachain-totem-kapex-node export-genesis-wasm > ./res/wasm-blobs/kapex-genesis-wasm && \
./target/release/parachain-totem-wapex-node export-genesis-wasm > ./res/wasm-blobs/wapex-genesis-wasm && \
./target/release/parachain-totem-lego-node export-genesis-wasm > ./res/wasm-blobs/lego-genesis-wasm && \
./target/release/parachain-totem-kapex-node build-spec > ./res/kapex-latest.json && \
./target/release/parachain-totem-wapex-node build-spec > ./res/wapex-latest.json && \
./target/release/parachain-totem-lego-node build-spec > ./res/lego-latest.json && \
./target/release/parachain-totem-kapex-node build-spec --raw > ./res/totem-kapex.json && \
./target/release/parachain-totem-wapex-node build-spec --raw > ./res/totem-wapex.json && \
./target/release/parachain-totem-lego-node build-spec --raw > ./res/totem-lego.json

#./target/release/polkadot build-spec --chain rococo-local --disable-default-bootnode --raw > rococo-local-cfde.json
