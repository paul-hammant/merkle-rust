## Rust Merkle Tree Demos

Generate a Merkle tree from a bunch of static US election data in `data/`. 

```
cargo run -- ./data
```

## Tests

To run its integration tests:

```
RUST_BACKTRACE=1 RUST_LOG=simple_merkle_tree_demo=debug cargo test -- --nocapture
```

## Older Python version

Is [here](https://github.com/paul-hammant/simple_Merkle_tree_demo)

## Contributions

Ported from Python by [Ash Levy](https://gitlab.com/ashkitten)
