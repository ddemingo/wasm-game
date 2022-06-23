#!/bin/bash

if ! hash nvm 2>/dev/null; then
    curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.31.3/install.sh | bash
fi

if ! hash node 2>/dev/null; then
    nvm install 16
fi

if ! hash cargo 2>/dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi

if ! hash wasm-pack 2>/dev/null; then
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 
fi

npm install

