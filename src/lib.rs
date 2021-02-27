//! EZEmojis is a Work in progress made for [rust-rain](https://rusty-rain.xyz) program
//! so just a warning that the api may change a lot in the coming updates.
use std::collections::HashMap;
use std::hash::Hash;

/// All Default Implemented Emojis Groups.
///
/// Some of these may not show up for you depending on your Font I think.
/// correct me if I am wrong.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EmojiGroups {
    Emojis,
    Japanese,
    Smile,
    Moon,
    Earth,
    Plant,
    Clock,
    Shape,
    Arrow,
    All,
}

/// CharGroups are a collection of u32 numbers that group like characters together.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CharGroups<T: Hash + Eq> {
    /// Default Implemented Groups
    Standard(EmojiGroups),
    /// Place your Custom Groups in this one.
    Custom(T),
}

impl<T: Hash + Eq> From<EmojiGroups> for CharGroups<T> {
    fn from(v: EmojiGroups) -> Self {Self::Standard(v)}
}

impl<T: Hash + Eq> CharGroups<T> {
    pub fn custom(v: T) -> Self {Self::Custom(v)}
}

/// Creates A `HashMap<EmojiGroups, Vec<u32>>` and returns it.
pub fn create_emoji_data<T: Hash + Eq>() -> HashMap<CharGroups<T>, Vec<u32>> {
        let mut hash = HashMap::new();
        hash.insert(EmojiGroups::Emojis.into(), create_emojis());
        // FIXME:
        hash.insert(EmojiGroups::Smile.into(), create_smile());

        hash.insert(EmojiGroups::Japanese.into(), create_jap());
        hash.insert(EmojiGroups::Moon.into(), create_moon());
        hash.insert(EmojiGroups::Earth.into(), create_earth());
        hash.insert(EmojiGroups::Plant.into(), create_plant());
        hash.insert(EmojiGroups::Clock.into(), create_clock());
        hash.insert(EmojiGroups::Shape.into(), create_shape());
        hash.insert(EmojiGroups::Arrow.into(), create_arrows());
        hash.insert(EmojiGroups::All.into(),  create_all());
        hash
}


pub struct EZEmojis<T: Hash + Eq> {
    list: HashMap<CharGroups<T>, Vec<u32>>,
}

impl<T: Hash + Eq> EZEmojis<T> {
    pub fn new() -> Self {
        Self { list: create_emoji_data() }
    }

    pub fn add(&mut self, key: CharGroups<T>, value: Vec<u32>) {
        self.list.insert(key, value);
    }

    pub fn get_char(&self, key: &CharGroups<T>) -> Option<Vec<char>> {
        self.list.get(key).map(
            |n| n.iter().map(|num| std::char::from_u32(*num).unwrap_or(' ')).collect()
            )
    }

    pub fn get_u32(&self, key: &CharGroups<T>) -> Option<&Vec<u32>> {
        self.list.get(key)
    }
}

fn create_emojis() -> Vec<u32> {
    (129292..=129535).collect()
}

fn create_smile() -> Vec<u32> {
    (128512..=128518).collect()
}

fn create_jap() -> Vec<u32> {
    (65382..=65437).collect()
}

fn create_moon() -> Vec<u32> {
    (127760..=127773).collect()
}

fn create_earth() -> Vec<u32> {
    (127757..=127760).collect()
}

fn create_plant() -> Vec<u32> {
    (127793..=127827).collect()
}

fn create_clock() -> Vec<u32> {
    (128336..=128359).collect()
}

fn create_shape() -> Vec<u32> {
    (128992..=129003).collect()
}

fn create_arrows() -> Vec<u32> {
    let mut a: Vec<u32> = (129024..=129035).collect();
    a.append(&mut (129040..=129095).collect());
    a.append(&mut (129168..=129195).collect());
    a.append(&mut (129168..=129195).collect());
    a.append(&mut (129104..=129113).collect());
    a
}

fn create_all() -> Vec<u32> {
    let mut a = create_moon();
    a.append(&mut create_earth());
    a.append(&mut create_plant());
    a.append(&mut create_clock());
    a.append(&mut create_shape());
    a.append(&mut create_arrows());
    a
}
