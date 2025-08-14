<h1 align="center">
EZEmoji
</h1>

<p align="center">
  <a href="https://crates.io/crates/ezemoji"><img alt="crates.io" src="https://img.shields.io/crates/v/ezemoji.svg"></a>
  <a><img alt="lastupdated" src="https://img.shields.io/github/last-commit/cowboy8625/ezemoji"></a>
  <a><img alt="GitHub repo size" src="https://img.shields.io/github/repo-size/cowboy8625/ezemoji"></a>
  <a><img alt="issuse" src="https://img.shields.io/github/issues/cowboy8625/ezemoji"></a>
  <a><img alt="Lines of Code" src="https://img.shields.io/tokei/lines/github/cowboy8625/ezemoji"></a>
  <a><img alt="License" src="https://img.shields.io/badge/License-MIT-blue.svg"></a>
  <a href="https://discord.gg/KwnGX8P"><img alt="Discord Chat" src="https://img.shields.io/discord/509849754155614230"></a>
</p>

`ezemoji` is a **compile-time Rust library** that provides convenient access to groups of Unicode characters, including emojis, symbols, letters, numbers, and more. All character groups are generated at **compile time**, making it extremely efficient and suitable for `no_std` and embedded environments, with optional `alloc` support.

## Features

- **Compile-time character groups**: All slices of characters are generated at compile time for zero runtime cost.
- Access predefined groups like letters, numbers, emojis, shapes, and arrows.
- Iterate over characters as `u32` code points.
- Determine character display width (`Single` or `Double`) at runtime.
- **Custom extendable groups**: Users can define their own character sets at compile time.
- Fully `no_std` compatible, with optional `alloc` support for dynamic structures.

## Installation

Add `ezemoji` to your `Cargo.toml`:

```toml
[dependencies]
ezemoji = "0.2"
```

## Used in

- [Rusty Rain](https://github.com/cowboy/rusty-rain)

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

MIT / Apache-2.0
