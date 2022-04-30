
all: contracts/wormhole/target/wasm32-unknown-unknown/release/wormhole.wasm contracts/token_bridge/target/wasm32-unknown-unknown/release/token_bridge.wasm

contracts/wormhole/target/wasm32-unknown-unknown/release/wormhole.wasm: contracts/wormhole/src/*.rs
	(cd contracts/wormhole; cargo build --target wasm32-unknown-unknown --release)

contracts/token_bridge/target/wasm32-unknown-unknown/release/token_bridge.wasm: contracts/token_bridge/src/*.rs
	(cd contracts/token_bridge; cargo build --target wasm32-unknown-unknown --release)

test: all FORCE
	npx ts-node test/boot.ts

FORCE:
