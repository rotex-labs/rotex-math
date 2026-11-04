# rotex-color

Color-space types and conversions.

> [!WARNING]
> **Beta, pre-1.0.** Rotex is under active development. APIs, behavior, and crate
> boundaries can change in any 0.x release. Do not depend on it for production work yet.

## Overview

`rotex-color` holds color values and the conversions between them. It does not upload textures or choose a GPU format.

## Responsibilities

- `Srgba`, `LinearRgba`, and `Oklcha`
- `Hsva`, `Cmyka`, and `Xyza`
- Conversions between those representations

## Dependencies

`rotex-math`.

## Status

Version 0.1.0.

## Part of Rotex

Rotex is a graphics HAL workspace. See [rotex-dev](https://github.com/rotex-labs/rotex-dev) for the architecture and the other crates.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
