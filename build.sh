wasm-pack build --target bundler
wasm-opt -Os -o ./pkg/picasso_fingerprint_bg.wasm ./pkg/picasso_fingerprint_bg.wasm