[![Try on playground](https://img.shields.io/badge/Playground-Node_Template-brightgreen?logo=Parity%20Substrate)](https://docs.substrate.io/playground/) [![Matrix](https://img.shields.io/matrix/substrate-technical:matrix.org)](https://matrix.to/#/#substrate-technical:matrix.org)

# Substrate Node for Diffy chat messenger

The aim of this project is to develop a secured decentralized messenger that doesn’t store data on a centralized backend and uses personal Polkadot wallet credentials for chatting initiation and messaging.

P2p channels between users are set using WebRTC. This Substrate pallet is used for exchanging SDP offers. For address discovery of NAT users any public STUN server can be used. All messages between users are encrypted with user’s public keys. This pallet also includes a “contacts” feature: a user is able to tie names to wallet addresses and organize his contacts in a common way.

![image](https://user-images.githubusercontent.com/126072104/220610232-0b9a4033-97cc-44ba-8948-a610b2b0c4bf.png)

# Substrate Node Template

A fresh FRAME-based [Substrate](https://www.substrate.io/) node, ready for hacking :rocket:

## Getting Started

Follow the steps below to get started with the Node Template, or get it up and running right from
your browser in just a few clicks using
the [Substrate Playground](https://docs.substrate.io/playground/) :hammer_and_wrench:

### Using Nix

Install [nix](https://nixos.org/) and optionally [direnv](https://github.com/direnv/direnv) and
[lorri](https://github.com/nix-community/lorri) for a fully plug and play experience for setting up
the development environment. To get all the correct dependencies activate direnv `direnv allow` and
lorri `lorri shell`.

### Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### Run

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/diffychat -h
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with non-persistent state:

```bash
./target/release/diffychat --dev
```

Purge the development chain's state:

```bash
./target/release/diffychat purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_BACKTRACE=1 ./target/release/diffychat -ldebug --dev
```

> Development chain means that the state of our chain will be in a tmp folder while the nodes are
> running. Also, **alice** account will be authority and sudo account as declared in the
> [genesis state](https://github.com/substrate-developer-hub/substrate-diffychat/blob/main/node/src/chain_spec.rs#L49).
> At the same time the following accounts will be pre-funded:
> - Alice
> - Bob
> - Alice//stash
> - Bob//stash

In case of being interested in maintaining the chain' state between runs a base path must be added
so the db can be stored in the provided folder instead of a temporal one. We could use this folder
to store different chain databases, as a different folder will be created per different chain that
is ran. The following commands shows how to use a newly created folder as our db base path.

```bash
// Create a folder to use as the db base path
$ mkdir my-chain-state

// Use of that folder to store the chain state
$ ./target/release/diffychat --dev --base-path ./my-chain-state/

// Check the folder structure created inside the base path after running the chain
$ ls ./my-chain-state
chains
$ ls ./my-chain-state/chains/
dev
$ ls ./my-chain-state/chains/dev
db keystore network
```


### Connect with Polkadot-JS Apps Front-end

Once the node template is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your
local node template.

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, refer to our
[Simulate a network tutorial](https://docs.substrate.io/tutorials/get-started/simulate-network/).

### Substrate Builder Docker Image

Go to folder scripts/docker.

The Docker image in this folder is a `builder` image. It is self contained and allows users to build the binaries themselves.
There is no requirement on having Rust or any other toolchain installed but a working Docker environment.

Unlike the `parity/polkadot` image which contains a single binary (`polkadot`!) used by default, the image in this folder builds and contains several binaries and you need to provide the name of the binary to be called.

You should refer to the [.Dockerfile](./substrate_builder.Dockerfile) for the actual list. At the time of editing, the list of included binaries is:

- substrate
- subkey
- node-template
- chain-spec-builder

First, install [Docker](https://docs.docker.com/get-docker/).

Then to generate the latest parity/substrate image. Please run:
```sh
./build.sh
```

> If you wish to create a debug build rather than a production build, then you may modify the [.Dockerfile](./substrate_builder.Dockerfile) replacing `cargo build --locked --release` with just `cargo build --locked` and replacing `target/release` with `target/debug`. 
> If you get an error that a tcp port address is already in use then find an available port to use for the host port in the [.Dockerfile](./substrate_builder.Dockerfile).
The image can be used by passing the selected binary followed by the appropriate tags for this binary.

Your best guess to get started is to pass the `--help flag`. Here are a few examples:

- `./run.sh substrate --version`
- `./run.sh subkey --help`
- `./run.sh diffychat --version`
- `./run.sh chain-spec-builder --help`

Then try running the following command to start a single node development chain using the Substrate Node Template binary `diffychat`:

```sh
./run.sh diffychat --dev --ws-external
```

Note: It is recommended to provide a custom `--base-path` to store the chain database. For example:

```sh
# Run Substrate Node Template without re-compiling
./run.sh diffychat --dev --ws-external --base-path=/data
```

> To print logs follow the [Substrate debugging instructions](https://docs.substrate.io/test/debug/).
```sh
# Purge the local dev chain
./run.sh diffychat purge-chain --dev --base-path=/data -y
```

## Related repos

- [Diffy chat DOTRTC library](https://github.com/Belsoft-rs/diffychat-dotrtc)
