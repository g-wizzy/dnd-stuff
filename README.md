# DND STUFF

This project aims to provide a searchable repository of DND 5.5e spells, that runs entirely on your device.

The `dndlib` rust library is compiled to web assembly that the web app then uses.

## Getting started

### Prerequisites

You will need to [install `rust `and` cargo`](https://rustup.rs/) and have `npm `installed as well.

You will need to install wasm-pack like this: 

```bash
cargo install wasm-pack
```

And you need to make sure that cargo-installed applications are on your `$PATH` (they usually are installed in `$HOME/.cargo/bin`).

### Setting up the app

To prepare the web assembly library, run the following:

```bash
cargo run --bin load-spells -- --url "https://www.aidedd.org/spell/fr/"
wasm-pack build --target web
```

You can then run `npm run dev` to have your app up and running !
