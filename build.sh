wasm-pack build --target bundler --features debug
wasm-opt -Os -o ./pkg/picasso_fingerprint_bg.wasm ./pkg/picasso_fingerprint_bg.wasm