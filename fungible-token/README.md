# erc20

ERC-20 準拠（動作確認用コピー）

Issue your own token (ERC20)
https://docs.near.org/docs/tutorials/near-studio/token

## test

cargo test --package nep21-basic -- --nocapture

## build

env 'RUSTFLAGS=-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

## login

near login

## deploy

near deploy --wasmFile target/wasm32-unknown-unknown/release/nep21_basic.wasm --accountId <<ACCOUNT_ID.net>>

## exec

### トークン発行

near call <<ACCOUNT_ID.net>> new '{"owner_id":"<<ACCOUNT_ID.net>>", "total_supply":"<<トークン発行枚数>>"}' --accountId <<ACCOUNT_ID.net>>

### 総発行枚数のチェック

near view <<ACCOUNT_ID.net>> get_total_supply --accountId <<ACCOUNT_ID.net>>
