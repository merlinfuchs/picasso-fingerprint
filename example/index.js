import * as wasm from "picasso-wasm";

const seed = Math.floor(Math.random() * 1000)

console.log(wasm.make_fingerprint(seed, 25, 500, 500));
