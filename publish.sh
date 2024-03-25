#!bin/bash

## publish rust
cargo install rust-to-npm-cli

# login to cargo 
# cargo login 

# login to npm
npm adduser

# build the cli and bpulish
rust-to-npm-cli deploy -b -n @opeolluwa/utils


cargo publish -p utils-cli-migration 


cargo publish -p utils-cli-utils


cargo publish 