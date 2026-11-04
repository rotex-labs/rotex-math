# rotex-math

glam re-export plus spatial types and frustum culling.

> [!WARNING]
> **Beta, pre-1.0.** Rotex is under active development. APIs, behavior, and crate
> boundaries can change in any 0.x release. Do not depend on it for production work yet.

## Overview

`rotex-math` re-exports glam at the crate root and adds `Aabb` and `Frustum`. It is not part of the GPU command path.

## Responsibilities

- glam vectors, matrices, and transforms
- `Aabb` and `Frustum` intersection tests
- Optional `wide-simd` batch culling

## Dependencies

None.

## Status

Version 0.1.0. Without `wide-simd` the only math dependency is glam.

## Part of Rotex

Rotex is a graphics HAL workspace. See [rotex-dev](https://github.com/rotex-labs/rotex-dev) for the architecture and the other crates.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
