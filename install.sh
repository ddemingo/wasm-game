#!/bin/bash

# Install nvm and node
$ curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.31.3/install.sh | bash
$ nvm install 16

# Install rust
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install wasm-pack
$ curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 