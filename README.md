# Rust Merkle Tree Demo

This app does three things:

1. Generate an initial Merkle tree from a bunch of static US election data in `data/`.
2. Serve that up over HTTP 
3. Keep the Merkle tree updated if files change (well, it should at least)

# Running it

```
cargo run -- ./data
```

## Running the test suite

To run its integration tests:

```
RUST_BACKTRACE=1 RUST_LOG=simple_merkle_tree_demo=debug cargo test -- --nocapture
```

## Older Python version

Is [here](https://github.com/paul-hammant/simple_Merkle_tree_demo). Thus Rust version is a port of that.

## Contributions

1. Ported from Python by [Ash Levy](https://gitlab.com/ashkitten)
2. Http server by VelocityRa.