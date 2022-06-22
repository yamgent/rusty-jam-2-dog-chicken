# rusty_jam_2_dog_chicken

A game written in Rust for the [WASM-4](https://wasm4.org) fantasy console.

Written for the [Rusty Jam 2](https://itch.io/jam/rusty-jam-2) game jam. The theme is a picture of a dog combined with chicken.

## Game Info

The objective is to combine items, and get the necessary items in order to eventually combine into a dogchicken.

Controls:

- `Z`: Select the first item to combine
- `X`: Select the second item to combine
- `<>^v`: Move the cursor around the list of items

## Building

Ensure that you have w4 CLI installed (skip this step if you already have w4):

```shell
yarn global add wasm4
```

Update assets with the png2src script:

```shell
./png2src.sh
```

Build the cart by running:

```shell
cargo build --release
```

Then run it with:

```shell
w4 run target/wasm32-unknown-unknown/release/cart.wasm
```

For more info about setting up WASM-4, see the [quickstart guide](https://wasm4.org/docs/getting-started/setup?code-lang=rust#quickstart).

## Release

Ensure that you have binaryen optimizer installed: [binaryen releases](https://github.com/WebAssembly/binaryen/releases).

(NOTE: wasm-opt version must be > 98 to have `--zero-filled-memory` option).

Then, run the following:

```shell
./release.sh
```

## Tests

```shell
./tests.sh
```

## Links

- [Documentation](https://wasm4.org/docs): Learn more about WASM-4.
- [Snake Tutorial](https://wasm4.org/docs/tutorials/snake/goal): Learn how to build a complete game
  with a step-by-step tutorial.
- [GitHub](https://github.com/aduros/wasm4): Submit an issue or PR. Contributions are welcome!
