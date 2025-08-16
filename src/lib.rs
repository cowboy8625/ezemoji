//! # ezemoji
//!
//! `ezemoji` is a lightweight Rust library for working with Unicode character groups, including letters, numbers, symbols, and emojis.
//! It is always `no_std` and stores all data as `&'static [u32]`, making it suitable for embedded and minimal runtime environments.
//!
//! ## Usage
//! You can use the predefined character groups, or define your own custom groups.
//!
//! ### Using predefined groups
//!
//! ```rust
//! use ezemoji::{CharGroup, GroupKind};
//!
//! // Iterate over lowercase alphabet characters
//! let alpha_group = CharGroup::ALPHALOW;
//! for cp in alpha_group.iter() {
//!     // Print the Unicode code point
//!     println!("{}", cp);
//! }
//!
//! // Access a numeric group and check display width
//! let num_group = CharGroup::NUM;
//! assert_eq!(num_group.width(), 1); // ASCII numbers are single-width
//! ```
//!
//! ### Defining your own custom group
//!
//! ```rust
//! use ezemoji::{CharGroup, GroupKind, CharWidth, MultiRange};
//!
//! // Create a new custom group of just crab emojis
//! let crab_group = CharGroup::new(GroupKind::Custom("crab"), MultiRange::new(&[0x1F980..0x1F981]), CharWidth::Double);
//! assert_eq!(crab_group.width(), 2); // This emoji is double-width
//! ```
#![cfg_attr(not(test), no_std)]

use core::{ops::Range, str::FromStr};

pub const ALPHALOW_RANGE: Range<u32> = 97..123;
pub const ALPHANUM_RANGE: Range<u32> = 48..58;
pub const ALPHAUP_RANGE: Range<u32> = 65..91;
pub const ARROW_0_RANGE: Range<u32> = 129_024..129_036;
pub const ARROW_1_RANGE: Range<u32> = 129_040..129_096;
pub const ARROW_2_RANGE: Range<u32> = 129_104..129_113;
pub const BIN_RANGE: Range<u32> = 48..50;
pub const BRAILLE_RANGE: Range<u32> = 10_241..10_252;
pub const CARD_0_RANGE: Range<u32> = 127_130..127_145;
pub const CARD_1_RANGE: Range<u32> = 127_148..127_162;
pub const CARD_2_RANGE: Range<u32> = 127_163..127_178;
pub const CARD_3_RANGE: Range<u32> = 127_179..127_240;
pub const CLOCK_RANGE: Range<u32> = 128_336..128_360;
pub const CRAB_RANGE: Range<u32> = 129_408..129_409;
pub const DOMINOSH_RANGE: Range<u32> = 127_024..127_073;
pub const DOMINOSV_RANGE: Range<u32> = 127_074..127_123;
pub const EARTH_RANGE: Range<u32> = 127_757..127_760;
pub const EMOJIS_0_RANGE: Range<u32> = 129_292..129_401;
pub const EMOJIS_1_RANGE: Range<u32> = 129_402..129_483;
pub const EMOJIS_2_RANGE: Range<u32> = 129_484..129_536;
pub const JAP_RANGE: Range<u32> = 65_382..65_437;
pub const LARGELETTERS_RANGE: Range<u32> = 65_313..65_338;
pub const LETTEREDCUBES_RANGE: Range<u32> = 127_344..127_369;
pub const MOON_RANGE: Range<u32> = 127_761..127_773;
pub const NUMBEREDBALLS_RANGE: Range<u32> = 9_312..9_332;
pub const NUM_RANGE: Range<u32> = 48..57;
pub const PLANTS_RANGE: Range<u32> = 127_793..127_827;
pub const SHAPES_RANGE: Range<u32> = 128_992..129_003;
pub const SMILE_RANGE: Range<u32> = 128_512..128_518;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MultiRange {
    ranges: &'static [Range<u32>],
}

impl MultiRange {
    pub const fn new(ranges: &'static [Range<u32>]) -> Self {
        MultiRange { ranges }
    }

    pub fn iter(&self) -> MultiRangeIterator {
        MultiRangeIterator {
            ranges: self.ranges,
            current_range: 0,
            index_in_range: 0,
        }
    }
}

impl<'a> IntoIterator for &'a MultiRange {
    type Item = char;
    type IntoIter = MultiRangeIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug)]
pub struct MultiRangeIterator<'a> {
    ranges: &'a [Range<u32>],
    current_range: usize,
    index_in_range: u32,
}

impl<'a> Iterator for MultiRangeIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_range < self.ranges.len() {
            let r = &self.ranges[self.current_range];
            let value = r.start + self.index_in_range;

            if value >= r.end {
                self.current_range += 1;
                self.index_in_range = 0;
                continue;
            }

            self.index_in_range += 1;
            return char::from_u32(value);
        }
        None
    }
}

/// A named group of Unicode characters.
///
/// `CharGroup` holds a static reference to a slice of Unicode code points (`u32`)
/// and a name describing the group. It provides utility methods for iterating
/// over the characters and determining their maximum display width.
///
/// # Examples
///
/// ```rust
/// # use ezemoji::{CharGroup, GroupKind};
/// let alpha_group = CharGroup::ALPHALOW;
/// for c in alpha_group.iter() {
///     println!("{}", c);
/// }
/// println!("Max width: {}", alpha_group.width());
/// ```
#[derive(Debug, Clone)]
pub struct CharGroup {
    /// The name of this character group.
    pub name: GroupKind,

    /// A range of characters in this group.
    pub range: MultiRange,

    /// The maximum display width of characters in this group.
    pub char_width: CharWidth,

    pub len: usize,
}

impl CharGroup {
    pub const ALL: CharGroup = CharGroup::new(
        GroupKind::All,
        MultiRange::new(&[
            // REMOVE ALPHA_NUM_SLICE (it’s inside ALPHA_LOW_SLICE and ALPHA_UP_SLICE)
            ALPHALOW_RANGE,
            ALPHAUP_RANGE,
            NUM_RANGE,
            ARROW_0_RANGE,
            ARROW_1_RANGE,
            ARROW_2_RANGE,
            // REMOVED BIN_RANGE (it's inside NUM_SLICE)
            CARD_0_RANGE,
            CARD_1_RANGE,
            CARD_2_RANGE,
            CARD_3_RANGE,
            CLOCK_RANGE,
            // REMOVE CRAB_RANGE (it’s inside emojis)
            DOMINOSH_RANGE,
            DOMINOSV_RANGE,
            EMOJIS_0_RANGE,
            EMOJIS_1_RANGE,
            EMOJIS_2_RANGE,
            JAP_RANGE,
            LARGELETTERS_RANGE,
            // use de-overlapped moon slice
            MOON_RANGE,
            EARTH_RANGE,
            NUMBEREDBALLS_RANGE,
            LETTEREDCUBES_RANGE,
            PLANTS_RANGE,
            SMILE_RANGE,
            SHAPES_RANGE,
        ]),
        CharWidth::Double,
    );

    pub const ALPHALOW: Self = Self::new(
        GroupKind::AlphaLow,
        // a-z
        MultiRange::new(&[ALPHALOW_RANGE]),
        CharWidth::Single,
    );

    pub const ALPHAUP: Self = Self::new(
        GroupKind::AlphaUp,
        // A-Z
        MultiRange::new(&[ALPHAUP_RANGE]),
        CharWidth::Single,
    );

    pub const ALPHANUM: Self = Self::new(
        GroupKind::AlphaNum,
        // 0-9
        MultiRange::new(&[ALPHANUM_RANGE]),
        CharWidth::Single,
    );

    pub const ARROW: Self = CharGroup::new(
        GroupKind::Arrow,
        //         skip🠇  🠇                                                        🠇 skip 🠇
        // 🠀🠁🠂🠃🠄🠅🠆🠇🠈🠉🠊🠋🠌🠍🠎🠏🠐🠑🠒🠓🠔🠕🠖🠗🠘🠙🠚🠛🠜🠝🠞🠟🠠🠡🠢🠣🠤🠥🠦🠧🠨🠩🠪🠫🠬🠭🠮🠯🠰🠱🠲🠳🠴🠵🠶🠷🠸🠹🠺🠻🠼🠽🠾🠿🡀🡁🡂🡃🡄🡅🡆🡇🡈🡉🡊🡋🡌🡍🡎🡏🡐🡑🡒🡓🡔🡕🡖🡗🡘
        // skip 129_036 .. 129_040 skip 129_096 .. 129_104 ),
        MultiRange::new(&[ARROW_0_RANGE, ARROW_1_RANGE, ARROW_2_RANGE]),
        CharWidth::Double,
    );

    pub const BIN: Self = CharGroup::new(
        GroupKind::Bin,
        // 0 1
        MultiRange::new(&[BIN_RANGE]),
        CharWidth::Single,
    );

    pub const BRAILLE: Self = CharGroup::new(
        GroupKind::Braille,
        // ⠁⠂⠃⠄⠅⠆⠇⠈⠉⠊⠋⠌⠍⠎⠏⠐⠑⠒⠓⠔⠕⠖⠗⠘⠙⠚⠛⠜⠝⠞⠟⠠⠡⠢⠣⠤⠥⠦⠧⠨⠩⠪⠫⠬⠭⠮⠯⠰⠱⠲⠳⠴⠵⠶⠷
        // ⠸⠹⠺⠻⠼⠽⠾⠿⡀⡁⡂⡃⡄⡅⡆⡇⡈⡉⡊⡋⡌⡍⡎⡏⡐⡑⡒⡓⡔⡕⡖⡗⡘⡙⡚⡛⡜⡝⡞⡟⡠⡡⡢⡣⡤⡥⡦⡧⡨⡩⡪⡫⡬⡭⡮
        // ⡯⡰⡱⡲⡳⡴⡵⡶⡷⡸⡹⡺⡻⡼⡽⡾⡿⢀⢁⢂⢃⢄⢅⢆⢇⢈⢉⢊⢋⢌⢍⢎⢏⢐⢑⢒⢓⢔⢕⢖⢗⢘⢙⢚⢛⢜⢝⢞⢟⢠⢡⢢⢣⢤⢥
        // ⢦⢧⢨⢩⢪⢫⢬⢭⢮⢯⢰⢱⢲⢳⢴⢵⢶⢷⢸⢹⢺⢻⢼⢽⢾⢿⣀⣁⣂⣃⣄⣅⣆⣇⣈⣉⣊⣋⣌⣍⣎⣏⣐⣑⣒⣓⣔⣕⣖⣗⣘⣙⣚⣛⣜
        // ⣝⣞⣟⣠⣡⣢⣣⣤⣥⣦⣧⣨⣩⣪⣫⣬⣭⣮⣯⣰⣱⣲⣳⣴⣵⣶⣷⣸⣹⣺⣻⣼⣽⣾
        MultiRange::new(&[BRAILLE_RANGE]),
        CharWidth::Single,
    );

    pub const CARDS: Self = CharGroup::new(
        GroupKind::Cards,
        //                           skip🠇🠇                          skip🠇
        // 🂠🂡🂢🂣🂤🂥🂦🂧🂨🂩🂪🂫🂬🂭🂮🂯🂰🂱🂲🂳🂴🂵🂶🂷🂸🂹🂺🂻🂼🂽🂾🂿🃀🃁🃂🃃🃄🃅🃆🃇🃈🃉🃊🃋🃌🃍
        // skip🠇
        // 🃎🃏🃐🃑🃒🃓🃔🃕🃖🃗🃘🃙🃚🃛🃜🃝🃞🃟🃠🃡🃢🃣🃤🃥🃦🃧🃨🃩🃪🃫🃬🃭🃮🃯🃰🃱🃲🃳🃴🃵
        // skip 127_145 .. 127_148 skip 127_162 .. 127_163 skip 127_178 .. 127_179 ),
        MultiRange::new(&[CARD_0_RANGE, CARD_1_RANGE, CARD_2_RANGE, CARD_3_RANGE]),
        CharWidth::Double,
    );

    pub const CLOCK: Self = CharGroup::new(
        GroupKind::Clock,
        // 🕐🕑🕒🕓🕔🕕🕖🕗🕘🕙🕚🕛🕜🕝🕞🕟🕠🕡🕢🕣🕤🕥🕦🕧
        MultiRange::new(&[CLOCK_RANGE]),
        CharWidth::Double,
    );

    pub const CRAB: Self = CharGroup::new(
        GroupKind::Crab,
        // 🦀
        MultiRange::new(&[CRAB_RANGE]),
        CharWidth::Double,
    );

    pub const DOMINOSH: Self = CharGroup::new(
        GroupKind::Dominosh,
        // 🀰🀱🀲🀳🀴🀵🀶🀷🀸🀹🀺🀻🀼🀽🀾🀿🁀🁁🁂🁃🁄🁅🁆🁇🁈🁉🁊🁋🁌🁍🁎🁏🁐🁑🁒🁓🁔🁕🁖🁗🁘🁙🁚🁛🁜🁝🁞🁟🁠
        MultiRange::new(&[DOMINOSH_RANGE]),
        CharWidth::Double,
    );

    pub const DOMINOSV: Self = CharGroup::new(
        GroupKind::Dominosv,
        // 🁢🁣🁤🁥🁦🁧🁨🁩🁪🁫🁬🁭🁮🁯🁰🁱🁲🁳🁴🁵🁶🁷🁸🁹🁺🁻🁼🁽🁾🁿🂀🂁🂂🂃🂄🂅🂆🂇🂈🂉🂊🂋🂌🂍🂎🂏🂐🂑🂒
        MultiRange::new(&[DOMINOSV_RANGE]),
        CharWidth::Single,
    );

    pub const EARTH: Self = CharGroup::new(
        GroupKind::Earth,
        // 🌍🌎🌏
        MultiRange::new(&[EARTH_RANGE]),
        CharWidth::Double,
    );

    pub const EMOJIS: Self = CharGroup::new(
        GroupKind::Emojis,
        // 🤌🤍🤎🤏🤐🤑🤒🤓🤔🤕🤖🤗🤘🤙🤚🤛🤜🤝🤞🤟🤠🤡🤢🤣🤤🤥🤦🤧🤨🤩🤪🤫🤬🤭
        // 🤮🤯🤰🤱🤲🤳🤴🤵🤶🤷🤸🤹🤺🤻🤼🤽🤾🤿🥀🥁🥂🥃🥄🥅🥆🥇🥈🥉🥊🥋🥌🥍🥎🥏🥐
        // 🥑🥒🥓🥔🥕🥖🥗🥘🥙🥚🥛🥜🥝🥞🥟🥠🥡🥢🥣🥤🥥🥦🥧🥨🥩🥪🥫🥬🥭🥮🥯🥰🥱🥲
        // 🥳🥴🥵🥶🥷🥸🥺🥻🥼🥽🥾🥿🦀🦁🦂🦃🦄🦅🦆🦇🦈🦉🦊🦋🦌🦍🦎🦏🦐🦑🦒🦓🦔🦕
        // 🦖🦗🦘🦙🦚🦛🦜🦝🦞🦟🦠🦡🦢🦣🦤🦥🦦🦧🦨🦩🦪🦫🦬🦭🦮🦯🦰🦱🦲🦳🦴🦵🦶🦷
        // 🦸🦹🦺🦻🦼🦽🦾🦿🧀🧁🧂🧃🧄🧅🧆🧇🧈🧉🧊🧌🧍🧎🧏🧐🧑🧒🧓🧔🧕🧖🧗🧘🧙🧚
        // 🧛🧜🧝🧞🧟🧠🧡🧢🧣🧤🧥🧦🧧🧨🧩🧪🧫🧬🧭🧮🧯🧰🧱🧲🧳🧴🧵🧶🧷🧸🧹🧺🧻🧼
        // 🧽🧾
        MultiRange::new(&[EMOJIS_0_RANGE, EMOJIS_1_RANGE, EMOJIS_2_RANGE]),
        CharWidth::Double,
    );

    pub const JAP: Self = CharGroup::new(
        GroupKind::Jap,
        // NOTE: this messes with my editor so I fold it up
        // ｦｧｨｩｪｫｬｭｮｯｰｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜ
        MultiRange::new(&[JAP_RANGE]),
        CharWidth::Single,
    );

    pub const LARGELETTERS: Self = CharGroup::new(
        GroupKind::LargeLetters,
        // NOTE: I used to have
        // 🇦 🇧 🇨 🇩 🇪 🇫 🇬 🇭 🇮 🇯 🇰 🇱 🇲 🇳 🇴 🇵 🇶 🇷 🇸 🇹 🇺 🇻 🇼 🇽 🇾
        // But that makes the flags when they are next to each other
        // 🇺 + 🇸 = 🇺🇸
        // ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹ
        MultiRange::new(&[LARGELETTERS_RANGE]),
        CharWidth::Double,
    );

    pub const MOON: Self = CharGroup::new(
        GroupKind::Moon,
        // 🌑🌒🌓🌔🌕🌖🌗🌘🌙🌚🌛
        MultiRange::new(&[MOON_RANGE]),
        CharWidth::Double,
    );

    pub const NUM: Self = CharGroup::new(
        GroupKind::Num,
        // 0..9
        MultiRange::new(&[NUM_RANGE]),
        CharWidth::Single,
    );

    pub const NUMBEREDBALLS: Self = CharGroup::new(
        GroupKind::NumberedBalls,
        // ①②③④⑤⑥⑦⑧⑨⑩⑪⑫⑬⑭⑮⑯⑰⑱⑲⑳
        MultiRange::new(&[NUMBEREDBALLS_RANGE]),
        CharWidth::Double,
    );

    pub const LETTEREDCUBES: Self = CharGroup::new(
        GroupKind::LetteredCubes,
        // 🅰 🅱 🅲 🅳 🅴 🅵 🅶 🅷 🅸 🅹 🅺 🅻 🅼 🅽 🅾 🅿 🆀 🆁 🆂 🆃 🆄 🆅 🆆 🆇 🆈
        MultiRange::new(&[LETTEREDCUBES_RANGE]),
        CharWidth::Double,
    );

    pub const PLANTS: Self = CharGroup::new(
        GroupKind::Plants,
        // 🌱 🌲 🌳 🌴 🌵 🌶 🌷 🌸 🌹 🌺 🌻 🌼 🌽 🌾 🌿 🍀 🍁 🍂 🍃 🍄 🍅 🍆 🍇
        // 🍈 🍉 🍊 🍋 🍌 🍍 🍎 🍏 🍐 🍑 🍒
        MultiRange::new(&[PLANTS_RANGE]),
        CharWidth::Double,
    );

    pub const SMILE: Self = CharGroup::new(
        GroupKind::Smile,
        // 😀😁😂😃😄😅
        MultiRange::new(&[SMILE_RANGE]),
        CharWidth::Double,
    );

    pub const SHAPES: Self = CharGroup::new(
        GroupKind::Shapes,
        // 🟨🟩🟪
        MultiRange::new(&[SHAPES_RANGE]),
        CharWidth::Double,
    );

    pub const fn new(name: GroupKind, range: MultiRange, char_width: CharWidth) -> Self {
        let mut len = 0;
        let mut i = 0;
        while i < range.ranges.len() {
            len += (range.ranges[i].end - range.ranges[i].start) as usize;
            i += 1;
        }

        Self {
            name,
            range,
            char_width,
            len,
        }
    }

    #[inline]
    pub fn iter(&self) -> MultiRangeIterator {
        self.range.iter()
    }

    #[cfg(feature = "alloc")]
    #[inline]
    pub fn as_slice_u32(&self) -> &[u32] {
        self.range.iter().as_slice()
    }

    #[inline]
    pub const fn width(&self) -> u8 {
        self.char_width as u8
    }

    /// Returns the character at the given index.
    pub fn nth_char(&self, index: usize) -> Option<char> {
        let index = index as u32;
        let mut i = 0u32;
        for range in self.range.ranges.iter() {
            let step = range.end - range.start;
            if index >= i && index < i + step {
                let offset = index - i;
                return char::from_u32(range.start + offset);
            }
            i += step;
        }
        return None;
    }

    pub const fn len(&self) -> usize {
        self.len
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum CharWidth {
    Single = 1,
    Double = 2,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum GroupKind {
    All,
    AlphaLow,
    AlphaNum,
    AlphaUp,
    Arrow,
    Bin,
    Braille,
    Cards,
    Clock,
    Crab,
    Custom(&'static str),
    Dominosh,
    Dominosv,
    Earth,
    Emojis,
    Jap,
    LargeLetters,
    LetteredCubes,
    Moon,
    Num,
    NumberedBalls,
    Plants,
    Shapes,
    Smile,
}

impl FromStr for CharGroup {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(CharGroup::ALL),
            "alphalow" => Ok(CharGroup::ALPHALOW),
            "alphanum" => Ok(CharGroup::ALPHANUM),
            "alphaup" => Ok(CharGroup::ALPHAUP),
            "arrow" => Ok(CharGroup::ARROW),
            "bin" => Ok(CharGroup::BIN),
            "braille" => Ok(CharGroup::BRAILLE),
            "cards" => Ok(CharGroup::CARDS),
            "clock" => Ok(CharGroup::CLOCK),
            "crab" => Ok(CharGroup::CRAB),
            "dominosh" => Ok(CharGroup::DOMINOSH),
            "dominosv" => Ok(CharGroup::DOMINOSV),
            "earth" => Ok(CharGroup::EARTH),
            "emojis" => Ok(CharGroup::EMOJIS),
            "jap" => Ok(CharGroup::JAP),
            "large-letters" => Ok(CharGroup::LARGELETTERS),
            "lettered-cubes" => Ok(CharGroup::LETTEREDCUBES),
            "moon" => Ok(CharGroup::MOON),
            "num" => Ok(CharGroup::NUM),
            "numbered-balls" => Ok(CharGroup::NUMBEREDBALLS),
            "plants" => Ok(CharGroup::PLANTS),
            "shapes" => Ok(CharGroup::SHAPES),
            "smile" => Ok(CharGroup::SMILE),
            // custom => Ok(CharGroup::new(
            //     GroupKind::Custom(&custom),
            //     MultiRange::new(&[]),
            //     CharWidth::Single,
            // )),
            _ => Err("Invalid group kind"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_group_length() {
        let group = CharGroup::ALPHALOW;
        assert_eq!(group.len(), 26);
    }

    #[test]
    fn test_group_nth_char() {
        let group = CharGroup::ALPHALOW;
        assert_eq!(group.nth_char(0), Some('a'));
        assert_eq!(group.nth_char(25), Some('z'));
    }

    macro_rules! width_calculation {
        ($name:ident, $group:ident, $expected:literal) => {
            #[test]
            fn $name() {
                let ascii_group = CharGroup::$group;
                assert_eq!(ascii_group.width(), $expected);
            }
        };
    }
    width_calculation!(width_calculation_for_all, ALL, 2);
    width_calculation!(width_calculation_for_alphalow, ALPHALOW, 1);
    width_calculation!(width_calculation_for_alphaup, ALPHAUP, 1);
    width_calculation!(width_calculation_for_alphanum, ALPHANUM, 1);
    width_calculation!(width_calculation_for_arrow, ARROW, 2);
    width_calculation!(width_calculation_for_bin, BIN, 1);
    width_calculation!(width_calculation_for_braille, BRAILLE, 1);
    width_calculation!(width_calculation_for_cards, CARDS, 2);
    width_calculation!(width_calculation_for_clock, CLOCK, 2);
    width_calculation!(width_calculation_for_crab, CRAB, 2);
    width_calculation!(width_calculation_for_dominosh, DOMINOSH, 2);
    width_calculation!(width_calculation_for_dominosv, DOMINOSV, 1);
    width_calculation!(width_calculation_for_earth, EARTH, 2);
    width_calculation!(width_calculation_for_emojis, EMOJIS, 2);
    width_calculation!(width_calculation_for_jap, JAP, 1);
    width_calculation!(width_calculation_for_largeletters, LARGELETTERS, 2);
    width_calculation!(width_calculation_for_moon, MOON, 2);
    width_calculation!(width_calculation_for_num, NUM, 1);
    width_calculation!(width_calculation_for_numberedballs, NUMBEREDBALLS, 2);
    width_calculation!(width_calculation_for_lettered_cubes, LETTEREDCUBES, 2);
    width_calculation!(width_calculation_for_plants, PLANTS, 2);
    width_calculation!(width_calculation_for_smile, SMILE, 2);
    width_calculation!(width_calculation_for_shapes, SHAPES, 2);
}
