#!/bin/sh

export PATH=${HOME}/.local/share/dfx/bin:${PATH}

make build
dfx start --background
dfx deploy
dfx canister info codelta_backend
dfx stop
