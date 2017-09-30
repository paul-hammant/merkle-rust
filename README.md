## Rust Merkle Tree Version

A simple Rust Merkle tree (ported by [Ash Levy](https://gitlab.com/ashkitten) from Python). To run it:

```
cargo run -- ./data
```

To run its integration tests:

```
RUST_BACKTRACE=1 RUST_LOG=simple_merkle_tree_demo=debug cargo test -- --nocapture
```
