# Substrate PoI node

## Introduction

This repository is a fork of the substrate node-template.  
We modified the consensus layer from Aura to Proof-of-Work (PoW). We then tried to replace PoW with a new consensus, Proof-of-Interaction (PoI, [paper link](https://hal.archives-ouvertes.fr/hal-02479891/document)).
The PoW consensus is designed to do local-only computations but the PoI consensus needed the ability to get the network address of other validator nodes. So we integrated the authority-discovery pallet to the PoW node to be able to retrieve other nodes network addresses. We also added a custom RPC method to the node that is able to sign arbitrary messages sent to the nodes (feature needed to do the interaction/signing part of PoI).

The technology stack used is: the Rust programming language and the Substrate blockchain framework.

## Dependencies

Depending on your operating system and Rust version, there might be additional packages required to compile this template.
Check the [Install](https://docs.substrate.io/install/) instructions for your platform for the most common dependencies.

As there is currently a bug with the latest version of rust, you will then have to rollback your rust version for the project to compile without issues:

```sh
rustup toolchain install nightly-2023-03-20 --force
rustup default nightly-2023-03-20-x86_64-unknown-linux-gnu
rustup target add wasm32-unknown-unknown --toolchain nightly-2023-03-20-x86_64-unknown-linux-gnu
```

## Build

### Locally

Use the following command to build the node without launching it:

```sh
cargo build --release
```

### Docker

A pre-built docker image of the node is also available here: https://hub.docker.com/r/ambula/node.  
The compiled binary of the substrate node is available in the image at `/usr/local/bin/node-template`.

## Run locally

After you build the project, you can use the following command to explore its parameters and subcommands:

```sh
./target/release/node-template -h
```

To run multiple nodes and connect them together, run the following commands:

1. Start the first node (bootstrap node) with the Alice predefined account:
```sh
./target/release/node-template \
  --base-path /tmp/alice \
  --chain local \
  --alice \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --validator
```

2. From another terminal, start the second node with the Bob predefined account (it will connect to Alice).
```sh
./target/release/node-template \
  --base-path /tmp/bob \
  --chain local \
  --bob \
  --port 30334 \
  --ws-port 9946 \
  --rpc-port 9934 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --validator \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

3. You can start more nodes by using the Charlie, Dave, Ferdie and Eve predefined accounts.
   Use the same command as when running Bob's node but change `--bob` with `--charlie` for example and use a different set of ports if you are running all nodes from the same computer.

## Consensus

We started by adding the Proof-of-Work consensus to the node (at `consensus/pow`).  
We then created an implementation of the Proof-of-Interaction consensus separated from the node (at `consensus/poi`).  

The PoW consensus is recognized and supported by the Substrate framework through an interface that needs to be implemented. Our last task was to integrate the PoI parts into the PoW consensus interface but the strict type system of the Rust programming language made it difficult to port our current PoI code into substrate.

## Node Structure

A Substrate project such as this consists of a number of components that are spread across a few directories.

### Node

A blockchain node is an application that allows users to participate in a blockchain network.
Substrate-based blockchain nodes expose a number of capabilities:

- Networking: Substrate nodes use the [`libp2p`](https://libp2p.io/) networking stack to allow the
  nodes in the network to communicate with one another.
- Consensus: Blockchains must have a way to come to [consensus](https://docs.substrate.io/fundamentals/consensus/) on the state of the network.
- RPC Server: A remote procedure call (RPC) server is used to interact with Substrate nodes.

There are several files in the `node` directory.
Take special note of the following:

- [`chain_spec.rs`](./node/src/chain_spec.rs): A [chain specification](https://docs.substrate.io/build/chain-spec/) is a source code file that defines a Substrate chain's initial (genesis) state.
- [`service.rs`](./node/src/service.rs): This file defines the node implementation.
  In particular, there are references to consensus-related topics, such as the [block finalization and forks](https://docs.substrate.io/fundamentals/consensus/#finalization-and-forks) and other [consensus mechanisms](https://docs.substrate.io/fundamentals/consensus/#default-consensus-models).

### Runtime

In Substrate, the terms "runtime" and "state transition function" are analogous.
Both terms refer to the core logic of the blockchain that is responsible for validating blocks and executing the state changes they define.
The Substrate project in this repository uses [FRAME](https://docs.substrate.io/fundamentals/runtime-development/#frame) to construct a blockchain runtime. FRAME allows runtime developers to declare domain-specific logic in modules called "pallets".

### Pallets

The runtime in this project is constructed using many FRAME pallets that ship with the [core Substrate repository](https://github.com/paritytech/substrate/tree/master/frame) and a template pallet that is [defined in the `pallets`](./pallets/template/src/lib.rs) directory.
