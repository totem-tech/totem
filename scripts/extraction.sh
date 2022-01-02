
# !!!!!!!!!!! These scripts are not intented to be executed by calling this file directly as there are manual tasks involved.

#______________________________________________________________#
# Compile node, 
#______________________________________________________________#

# cargo build --release -p parachain-totem-lego-node
# cargo build --release -p parachain-totem-wapex-node
# cargo build --release -p parachain-totem-kapex-node

#______________________________________________________________#
# Then extract the local version of the chainspec.
# This will provide ther WASM blob that needs to be added to the readable chainspec
#______________________________________________________________#

# ./target/release/parachain-totem-lego-node build-spec --chain local > ./res/lego-local.json
# ./target/release/parachain-totem-wapex-node build-spec --chain local > ./res/wapex-local.json
# ./target/release/parachain-totem-kapex-node build-spec --chain local > ./res/kapex-local.json

#______________________________________________________________#
# Once added the next step is to convert the readable chainspec to raw format for inclusion in the runtime.
#______________________________________________________________#

# ./target/release/parachain-totem-lego-node build-spec --chain ./res/totem-lego-readable.json --raw > ./res/totem-lego-raw.json
# ./target/release/parachain-totem-wapex-node build-spec --chain ./res/totem-wapex-readable.json --raw > ./res/totem-wapex-raw.json
# ./target/release/parachain-totem-kapex-node build-spec --chain ./res/totem-kapex-readable.json --raw > ./res/totem-kapex-raw.json

#______________________________________________________________#
# now the raw file is ready to be included in the runtime. Change the path in `command.rs`` to point to the raw version of the file.
# Recompile the node and extract the genesis state and wasm blob.
#______________________________________________________________#

# ./target/release/parachain-totem-lego-node export-genesis-state > ./res/genesis-states/lego-genesis-state-new && \
# ./target/release/parachain-totem-lego-node export-genesis-wasm > ./res/wasm-blobs/lego-genesis-wasm-new

# ./target/release/parachain-totem-wapex-node export-genesis-state > ./res/genesis-states/wapex-genesis-state-new && \
# ./target/release/parachain-totem-wapex-node export-genesis-wasm > ./res/wasm-blobs/wapex-genesis-wasm-new

# ./target/release/parachain-totem-kapex-node export-genesis-state > ./res/genesis-states/kapex-genesis-state-new && \
# ./target/release/parachain-totem-kapex-node export-genesis-wasm > ./res/wasm-blobs/kapex-genesis-wasm-new

#______________________________________________________________#
# Cleanup
#______________________________________________________________#

# rm -f ./res/lego-local.json
# rm -f ./res/wapex-local.json
# rm -f ./res/kapex-local.json


#______________________________________________________________#
# Note on how to extract a local chainspec from polkadot (not in this repo)

#./target/release/polkadot build-spec --chain rococo-local --disable-default-bootnode --raw > rococo-local-cfde.json