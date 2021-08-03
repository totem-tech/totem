# The Totem Live Accounting Main Code Repository

## Content

This repository contains:

- The Totem pallets,
- The Lego — Totem parachain — node and runtime,
- The Totem blockchain node and runtime,
- The Dockerfile to build the nodes.

## Documentation

### Embedded Nodes Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/[totem-lego-parachain-node | totem-mainnet-node] -h
```

### The Totem Docs

- [The pallets' Rust API](https://rustdocs.totemaccounting.com)

## Run

### Test

Use Rust's native `cargo` command to build and launch the Lego node:

```sh
cargo run --release -p totem-lego-parachain-node -- --dev --tmp
```

Use Rust's native `cargo` command to build and launch the blockchain node:

```sh
cargo run --release -p totem-mainnet-node -- --dev --tmp
```

### Single-Node Development Chain

This command will start the single-node development chain with persistent state (replace `<node>` with either `totem-lego-parachain-node` or `totem-mainnet-node`):

```bash
./target/release/<node> --dev
# Or with compilation
cargo run --release -p <node> -- --dev
```

Purge the development chain's state:

```bash
./target/release/<node> purge-chain --dev
# Or with compilation
cargo run --release -p <node> -- purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/<node> -lruntime=debug --dev
# Or with compilation
RUST_LOG=debug RUST_BACKTRACE=1 cargo run --release -p <node> -- -lruntime=debug --dev
```

### Connect with Polkadot-JS Apps Front-end

Once the node template is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain. [Click here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your local node template.

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, refer to
[our Start a Private Network tutorial](https://substrate.dev/docs/en/tutorials/start-a-private-network/).

## Docker 

The dockerfile is conveniently placed here. It can build an image for either node using the following commands:

```shell

# Totem Lego Parachain

docker build \
--build-arg chain=totem-lego-parachain-node \
--build-arg buildtype=build -t yourtag:yourversion .

# Totem Mainnet 

docker build \
--build-arg chain=totem-mainnet-node \
--build-arg buildtype=build -t yourtag:yourversion .

```

### Execution of Docker image

You can use the standard substrate commands to run your node. The following is an example of running an image based on the parachain tagged `yourtag:yourversion` and executing in `dev` mode with automatic purging of the chain data using `--tmp` and automatic removal of container after it is completed using `docker run --rm`.

    docker run --rm yourtag:yourversion totem-lego-parachain-node -- --dev --tmp


## Structure

A Substrate project such as this consists of a number of components that are spread across a few
directories.

### Node

A blockchain node is an application that allows users to participate in a blockchain network.
Substrate-based blockchain nodes expose a number of capabilities:

- Networking: Substrate nodes use the [`libp2p`](https://libp2p.io/) networking stack to allow the
  nodes in the network to communicate with one another.
- Consensus: Blockchains must have a way to come to
  [consensus](https://substrate.dev/docs/en/knowledgebase/advanced/consensus) on the state of the
  network. Substrate makes it possible to supply custom consensus engines and also ships with
  several consensus mechanisms that have been built on top of
  [Web3 Foundation research](https://research.web3.foundation/en/latest/polkadot/NPoS/index.html).
- RPC Server: A remote procedure call (RPC) server is used to interact with Substrate nodes.

There are several files in the `node` directory - take special note of the following:

- [`chain_spec.rs`](./node/src/chain_spec.rs): A
  [chain specification](https://substrate.dev/docs/en/knowledgebase/integrate/chain-spec) is a
  source code file that defines a Substrate chain's initial (genesis) state. Chain specifications
  are useful for development and testing, and critical when architecting the launch of a
  production chain. Take note of the `development_config` and `testnet_genesis` functions, which
  are used to define the genesis state for the local development chain configuration. These
  functions identify some
  [well-known accounts](https://substrate.dev/docs/en/knowledgebase/integrate/subkey#well-known-keys)
  and use them to configure the blockchain's initial state.
- [`service.rs`](./node/src/service.rs): This file defines the node implementation. Take note of
  the libraries that this file imports and the names of the functions it invokes. In particular,
  there are references to consensus-related topics, such as the
  [longest chain rule](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#longest-chain-rule),
  the [Aura](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#aura) block authoring
  mechanism and the
  [GRANDPA](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#grandpa) finality
  gadget.
