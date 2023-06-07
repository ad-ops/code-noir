# Code Noir
Simple demo of using a image library ([photon-rs](https://silvia-odwyer.github.io/photon/)) written in Rust in WebAssembly using [Wasm-Pack](https://rustwasm.github.io/wasm-pack/)

## Build
```sh
wasm-pack build --release --target web
```

## Run
Open `index.html` after it has been built and upload an image. You can use [VS Code Live Server](https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer) extension to try.