#!/bin/sh
./png2src.sh
cargo build --release
wasm-opt target/wasm32-unknown-unknown/release/cart.wasm -o target/wasm32-unknown-unknown/release/cart-opt.wasm -Oz --strip-dwarf --strip-producers --zero-filled-memory
w4 bundle target/wasm32-unknown-unknown/release/cart-opt.wasm --title "Find the DogChicken" --html release/find-the-dogchicken-web.html --windows release/find-the-dogchicken-win32.exe --linux release/find-the-dogchicken-linux
node patchForArcade.js

