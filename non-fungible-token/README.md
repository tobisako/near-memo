# non-fungible-token

NFT クローン

## test

cargo test --package nep4-rs -- --nocapture

## build

env 'RUSTFLAGS=-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

## login

near login

## deploy

near deploy --wasmFile target/wasm32-unknown-unknown/release/nep4_rs.wasm --accountId <<ACCOUNT_ID.net>>

## exec

### トークンコントラクトのインスタンスを作る

near call <<ACCOUNT_ID.net>> new '{"owner_id":"<<ACCOUNT_ID.net>>"}' --accountId <<ACCOUNT_ID.net>>

### 鋳造(mint)

near call <<ACCOUNT_ID.net>> mint_token '{"owner_id":"<<ACCOUNT_ID.net>>","token_id":<<ID_u64>>}' --accountId <<ACCOUNT_ID.net>>

### 転送

near call <<ACCOUNT_ID.net>> transfer_from '{"owner_id":"<<FROM_ACCOUNT_ID.net>>","new_owner_id":"<<TO_ACCOUNT_ID.net>>","token_id":<<ID_u64>>}' --accountId <<ACCOUNT_ID.net>>

### トークン所有者の確認

near view <<ACCOUNT_ID.net>> get_token_owner '{"token_id":1}' --accountId <<ACCOUNT_ID.net>>
