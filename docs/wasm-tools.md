# WASM Tools

## Install wasmtime (project likely uses it already)
curl https://wasmtime.dev/install.sh -sSf | bash
source ~/.bashrc

## Validate
wasm-tools validate add.wasm && echo "✅ Valid WASM"

## See what functions are exported
wasm-tools print add.wasm

## Test execution
wasmtime run --invoke add add.wasm 5 7