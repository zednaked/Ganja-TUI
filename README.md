# Ganja-TUI

A cannabis grow simulation that runs in your terminal, written in Rust with
[ratatui](https://ratatui.rs). Plant a seed, watch it through seven growth
stages over ninety in-game days, and harvest it — rendered entirely in ASCII.

## What's modelled

**Genetics.** 35 real strains, each with its own indica/sativa split, THC and
CBD ranges, flowering time, dominant terpenes, aroma and effects. Every plant
rolls its own phenotype inside the strain's ranges, so two Purple Kush seeds do
not grow into the same plant.

**Growth.** Seed → germination → seedling → vegetative → pre-flower → flowering
→ ready to harvest, on a day counter with light cycles.

**Care and consequence.** Each plant carries a resilience trait, a quality
ceiling, and a care history. Stress events are recorded with a cause and a
severity, and they follow the plant to harvest — a neglected grow yields less
and worse, and the numbers say why.

## Run

```sh
cargo run --release
```

Your grow is saved to `~/.local/share/ganjatui/save.json` and picked up again on
the next launch. Truecolor is used when the terminal advertises support, with a
256-colour fallback.

## Keys

| Key | |
| --- | --- |
| `1` | Growing room |
| `2` / `s` | Stats |
| `v` | Cycle visual mode |
| `a` | Toggle auto-harvest |
| `h` | Harvest (when the plant is ready) |
| `q` | Quit |

## Build

Rust 2021, no system dependencies beyond a terminal. `cargo build --release`.

## Licence

MIT.
