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

This Project is to make finding Emoji's and Char Groups a bit easier. (WIP)

EZEmojis is a Work in progress made for [rust-rain](https://rusty-rain.xyz) program
so just a warning that the api may change a lot in the coming updates.

## Example
```rust
use ezemoji::{EZEmoji, Crab};
fn main() {
    println!("{:?}", Crab.as_vec_char());
}
```


The `trait` `EZEmoji` is provided to glue all the types together.
You can implement it for your own type like

```rust
use ezemoji::EZEmoji;

struct MyEmojiGroup;
impl EZEmoji for MyEmojiGroup {
    fn as_vec_u32(&self) -> Vec<u32> {
        vec![96]
    }
}
```

### HELP
If you have any ideas for this crate make an issue and lets talk about it.
[github page](https://github.com/cowboy8625/ezemoji)
