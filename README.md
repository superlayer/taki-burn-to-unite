# Download solana & install solana

https://docs.solanalabs.com/cli/install

# Install rust

https://www.rust-lang.org/tools/install

# Start local network

To start the local solana network, needs to be on _WSL_

1. `cd ~`
2. `solana-test-validator`

You can reset the validator by using the following command

`solana-test-validator -r`

Build

`cargo build-bpf`

Deploy

`solana program deploy ./target/deploy/solana_burn_token.so`

Update Solana

`solana-install update`
