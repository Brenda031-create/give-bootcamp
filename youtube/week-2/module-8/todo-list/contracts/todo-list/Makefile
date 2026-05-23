-include .env

all: test

test: build
	cargo test

debug: build
	RUST_BACKTRACE=full cargo test

build:
	stellar contract build
	@ls -l ../../target/wasm32v1-none/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean

upload: build
	@echo "📦 Uploading contract..."
	@stellar contract install \
	  --network ${TESTNET} \
	  --source ${SOURCE_TESTNET} \
	  --wasm ../../target/wasm32v1-none/release/todo_list.wasm > upload.log 2>&1
	@tail -n 1 upload.log > wasm_hash.txt
	@echo "✅ Wasm hash saved to wasm_hash.txt"
	@cat wasm_hash.txt

deploy:
	stellar contract deploy \
	--wasm-hash ${WASM_HASH} \
	--network ${TESTNET} \
	--source-account ${SOURCE_TESTNET} \
	-- \

interact: 
	stellar contract invoke \
  --id ${CONTRACT_ADDRESS} \
  --source ${SOURCE_TESTNET} \
	--network ${TESTNET} \
  -- \
  get_todos \

list:
	stellar network ls --long

default:
	stellar network use mainnet-ankr

show:
	stellar keys show 

address:
	stellar keys address 

log:
	cat upload.log
