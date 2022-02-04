
# !!!!!!!!!!! These scripts are not intented to be executed by calling this file directly as there are manual tasks involved.

#______________________________________________________________#
# Compile node, 
#______________________________________________________________#

# cargo build --release -p lego-node
# cargo build --release -p wapex-node
# cargo build --release -p kapex-node

#______________________________________________________________#
# Then extract the local version of the chainspec.
# This will provide ther WASM blob that needs to be added to the readable chainspec
#______________________________________________________________#

# ./target/release/lego-node build-spec --chain lego > ./res/lego-node-readable.json && \
# ./target/release/wapex-node build-spec --chain wapex > ./res/wapex-node-readable.json && \
# ./target/release/kapex-node build-spec --chain kapex > ./res/kapex-node-readable.json

#______________________________________________________________#
# Once added the next step is to convert the readable chainspec to raw format for inclusion in the runtime.
#______________________________________________________________#

# ./target/release/lego-node build-spec --chain ./res/lego-node-readable.json --raw > ./res/lego-node-raw-new.json && \
# ./target/release/wapex-node build-spec --chain ./res/wapex-node-readable.json --raw > ./res/wapex-node-raw-new.json && \
# ./target/release/kapex-node build-spec --chain ./res/kapex-node-readable.json --raw > ./res/kapex-node-raw-new.json

#______________________________________________________________#
# now the raw file is ready to be included in the runtime. Change the path in `command.rs`` to point to the raw version of the file.
# Recompile the node and extract the genesis state and wasm blob.
#______________________________________________________________#

# ./target/release/lego-node export-genesis-state --chain ./res/lego-node-raw-new.json > ./res/genesis-states/lego-genesis-state-new && \
# ./target/release/lego-node export-genesis-wasm --chain ./res/lego-node-raw-new.json > ./res/wasm-blobs/lego-genesis-wasm-new

# ./target/release/wapex-node export-genesis-state --chain ./res/wapex-node-raw-new.json > ./res/genesis-states/wapex-genesis-state-new && \
# ./target/release/wapex-node export-genesis-wasm --chain ./res/wapex-node-raw-new.json > ./res/wasm-blobs/wapex-genesis-wasm-new

# ./target/release/kapex-node export-genesis-state --chain ./res/kapex-node-raw-new.json > ./res/genesis-states/kapex-genesis-state-new && \
# ./target/release/kapex-node export-genesis-wasm --chain ./res/kapex-node-raw-new.json > ./res/wasm-blobs/kapex-genesis-wasm-new

#______________________________________________________________#
# Cleanup
#______________________________________________________________#

# Rename all the *-new files to just their normal names
# move the readable files to the final folder

#______________________________________________________________#
# Note on how to extract a local chainspec from polkadot (not in this repo)

#./target/release/polkadot build-spec --chain rococo-local --disable-default-bootnode --raw > rococo-local-cfde.json