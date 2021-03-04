use ezemoji::{EZEmojis, EmojiGroups};
#[derive(Hash, Eq, PartialEq)]
struct Num;
fn main() {
    let e: EZEmojis<()> = EZEmojis::default();
    println!("{:?}", e.get_char(&EmojiGroups::Smile.into()));
}
