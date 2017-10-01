## Rust Merkle Tree Demo

A simple Rust Merkle tree (ported from Python by [Ash Levy](https://gitlab.com/ashkitten) from Python). To run it:

```
cargo run -- ./data
```

To run its integration tests:

```
RUST_BACKTRACE=1 RUST_LOG=simple_merkle_tree_demo=debug cargo test -- --nocapture
```

## Older Python version

Is [here](https://github.com/paul-hammant/simple_Merkle_tree_demo)
