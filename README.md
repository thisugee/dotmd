# DotMD

DotMD is a simple, web-based Markdown editor built with Rust

![image](/public/screenshot.png)

### Installation

Firstly, install Rust: <https://www.rust-lang.org/tools/install>. The rest of this guide assumes a typical Rust installation which contains both `rustup` and Cargo.

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target installed. If you don't already have it, install it with the following command:

```bash
rustup target add wasm32-unknown-unknown
```

Now, install [Trunk]. For a more thorough explanation of Trunk and its features, please head over to the [repository][trunk]. Simply run the following command to install it:

```bash
cargo install trunk wasm-bindgen-cli
```

That's it, we're done!

### Running

```bash
trunk serve
```

Rebuilds the app whenever a change is detected and runs a local server to host it.

There's also the `trunk watch` command which does the same thing but without hosting it.

### Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`. You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

Unless overwritten, the output will be located in the `dist` directory.

[trunk]: https://github.com/thedodd/trunk
