# paramtest

near-cli パラメータ例

## test

cargo test --package param-test -- --nocapture

## build

env 'RUSTFLAGS=-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

## login

near login

## deploy

near deploy --wasmFile target/wasm32-unknown-unknown/release/param_test.wasm --accountId <<ACCOUNT_ID.net>>

## exec

near view <<ACCOUNT_ID.net>> get_num --accountId <<ACCOUNT_ID.net>>
near view <<ACCOUNT_ID.net>> boolin '{"flg":true}' --accountId <<ACCOUNT_ID.net>>
near view tobi_s1.testnet calc '{"abc":4}' --accountId tobi_s1.testnet
