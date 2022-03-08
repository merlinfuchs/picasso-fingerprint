# Picasso WASM

Lightweight Device Class Fingerprinting for Web Clients using the HTML5 Canvas.

This small library implements parts of the [Google Picasso white paper](https://storage.googleapis.com/pub-tools-public-publication-data/pdf/45581.pdf) in WebAssembly. 
It can be used to identify different device-classes and generate a unique fingerprint for them. 
It can't be used to identify individual devices.

For more information about how this can be used check the white paper.

## [Demo](https://merlinfuchs.github.io/picasso-fingerprint/)

## Usage

```shell
npm install picasso-wasm
```

```js
// these values could come from your server
const seed = 1234567890;
const rounds = 25;
const width = 500;
const height = 500;

import("picasso-wasm")
    .then(wasm => wasm.make_fingerprint(seed, rounds, height, width))
    .then(fingerprint => console.log(fingerprint))
```
