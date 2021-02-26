use ezemoji::{EZEmojis,EmojiGroups};
#[derive(Hash, Eq, PartialEq)]
struct Num;
fn main() {
    let e: EZEmojis<()> = EZEmojis::new();
    println!("{:?}", e.get_char(&EmojiGroups::Smile.into()));
}

enum MyGroups {
    Numbers,
    LowerAlpha,
    UpperAlpha,
}
