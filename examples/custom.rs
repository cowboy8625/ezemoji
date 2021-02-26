use ezemoji::{EZEmojis,EmojiGroups, CharGroups, };
#[derive(Hash, Eq, PartialEq)]
struct Num;
fn main() {
    let mut e: EZEmojis<MyGroups> = EZEmojis::new();
    e.add(CharGroups::custom(MyGroups::Numbers), (49..57).collect());
    e.add(CharGroups::custom(MyGroups::LowerAlpha), (97..122).collect());
    e.add(CharGroups::custom(MyGroups::UpperAlpha), (65..90).collect());
    println!("{:?}", e.get_char(&EmojiGroups::Smile.into()));
    println!("{:?}", e.get_char(&CharGroups::custom(MyGroups::Numbers)));
    println!("{:?}", e.get_char(&CharGroups::custom(MyGroups::LowerAlpha)));
    println!("{:?}", e.get_char(&CharGroups::custom(MyGroups::UpperAlpha)));
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum MyGroups {
    Numbers,
    LowerAlpha,
    UpperAlpha,
}
