
all: contracts/wormhole/target/wasm32-unknown-unknown/release/wormhole.wasm contracts/portal/target/wasm32-unknown-unknown/release/portal.wasm

contracts/wormhole/target/wasm32-unknown-unknown/release/wormhole.wasm: contracts/wormhole/src/*.rs
	(cd contracts/wormhole; cargo build --target wasm32-unknown-unknown --release)

contracts/portal/target/wasm32-unknown-unknown/release/portal.wasm: contracts/portal/src/*.rs
	(cd contracts/portal; cargo build --target wasm32-unknown-unknown --release)

test: all FORCE
	npx ts-node test/test.ts

FORCE:
