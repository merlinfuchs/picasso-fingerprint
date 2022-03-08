import * as wasm from "picasso-wasm";

const seed = 1234567890;

const fingerprint = wasm.make_fingerprint(seed, 25, 500, 500)
document.getElementById("fingerprint").innerText = fingerprint;
