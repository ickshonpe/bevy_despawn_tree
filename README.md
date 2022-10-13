# bevy_despawn_tree

[![crates.io](https://img.shields.io/crates/v/bevy_despawn_tree)](https://crates.io/crates/bevy_despawn_tree)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/ickshonpe/bevy_despawn_tree)
[![crates.io](https://img.shields.io/crates/d/bevy_despawn_tree)](https://crates.io/crates/bevy_despawn_tree)

An extension method for Bevy's `EntityCommands` that despawns an entire parent-child entity hierarchy tree
from an EntityCommands for any entity at any depth in the hierarchy.

Supports Bevy 0.8

## Usage

Add the dependency to your project's `Cargo.toml`:

```toml
bevy_despawn_tree = "0.1.0"
```

Then to despawn the tree containing the entity `leaf`:

```rust
use bevy_despawn_tree::*;

commands.entity(leaf).despawn_tree();
```

## Example

Spawns two marked trees, despawns one from a queried child without touching the other.

```
cargo run --example example
```