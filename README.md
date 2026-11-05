<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/github/rotex-readme-banner-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/github/rotex-readme-banner-light.svg">
    <img alt="Rotex: One API. Every GPU. Zero cost." src="https://raw.githubusercontent.com/rotex-labs/rotex-assets/55fa2474cb788a445c33b4ccf615eb8bbb9dbe64/brand/github/rotex-readme-banner-light.svg" width="1200">
  </picture>
</p>

# rotex-math

glam, spatial types, and color conversions. These crates stay off the GPU command path.

> [!WARNING]
> **Beta, pre-1.0.** Rotex is under active development. APIs, behavior, and crate
> boundaries can change in any 0.x release. Do not depend on it for production work yet.

## Overview

This repository holds the math crates that change together and do not import a GPU API. `rotex-color` converts color values. It does not upload textures or choose a GPU format. `rotex-math` re-exports glam and adds bounds and frustum tests. Neither crate depends on `rotex-types`.

## Crates

| Package | Description | Version |
|---------|-------------|---------|
| [`rotex-math`](crates/rotex-math/README.md) | glam re-export plus spatial types and frustum culling | 0.1.0 |
| [`rotex-color`](crates/rotex-color/README.md) | Color-space types and conversions | 0.1.0 |

## Dependencies

`rotex-color` depends on `rotex-math` by path plus version inside this repository. A requirement that leaves the slice stays a crates.io version. Each package keeps its own version and its own release tag (`rotex-math-v0.1.0`, `rotex-color-v0.1.0`).

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](crates/rotex-math/LICENSE-APACHE))
* MIT license ([LICENSE-MIT](crates/rotex-math/LICENSE-MIT))

at your option. Each crate directory carries the same two files.
