# Parsify Core Helpers

> Bunch of @parsify/core helpers, powered by Rust and WASM.

[![Build Status](https://travis-ci.com/parsify-dev/core-helpers.svg?branch=master)](https://travis-ci.com/parsify-dev/core-helpers)

## Development

### Prerequisites

- [Node.js](https://nodejs.org/en/)
- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

``` bash
# Build for Node.js
$ wasm-pack build --target nodejs

# Run tests, formatter and linter
$ cargo fmt --all -- --check && cargo clippy && cargo test
```

## License

MIT