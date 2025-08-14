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
//! for cp in alpha_group.as_slice_u32() {
//!     // Print the Unicode code point
//!     println!("U+{:04X}", cp);
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
//! use ezemoji::{CharGroup, GroupKind, CharWidth};
//!
//! // Create a group of just crab emojis
//! let crab_group = CharGroup::new(GroupKind::Crab, &[0x1F980, 0x1F981], CharWidth::Double);
//! assert_eq!(crab_group.width(), 2); // Emoji are double-width
//! ```
//!
//! ## Notes
//! - All character groups are compile-time `&'static [u32]` slices. Iterating them has zero runtime cost.
//! - `width()` returns 1 for ASCII characters, 2 for most emojis and wide symbols.
//! - You can define your own `CharGroup` by providing a name and a static slice of `u32` codepoints.

#![cfg_attr(not(test), no_std)]

macro_rules! static_range {
    ($name:ident, $start:literal, $end:literal) => {
        const $name: &[u32] = &{
            const LEN: usize = ($end - $start + 1) as usize;
            let mut arr = [0u32; LEN];
            let mut i = 0;
            while i < LEN {
                arr[i] = $start + i as u32;
                i += 1;
            }
            arr
        };
    };
}

macro_rules! all_slices {
    ($($slice:ident),+ $(,)?) => {{
        const LEN: usize = 0 $(+ $slice.len())+;

        let mut arr = [0u32; LEN];
        let mut i = 0;

        $(
            let mut j = 0;
            while j < $slice.len() {
                arr[i + j] = $slice[j];
                j += 1;
            }
            i += $slice.len();
            // Just to shut up the compiler
            let _ = i;
        )+

        arr
    }};
}

macro_rules! slice_skip_first {
    ($name:ident, $src:ident) => {
        const $name: &[u32] = &{
            const LEN: usize = if $src.len() > 0 { $src.len() - 1 } else { 0 };
            let mut arr = [0u32; LEN];
            let mut i = 0;
            while i < LEN {
                arr[i] = $src[i + 1];
                i += 1;
            }
            arr
        };
    };
}

static_range!(ALPHA_LOW_SLICE, 97, 122);
static_range!(ALPHA_UP_SLICE, 65, 90);
static_range!(NUM_SLICE, 48, 57);
static_range!(BIN_SLICE, 48, 49);
static_range!(CLOCK_SLICE, 128_336, 128_359);
static_range!(DOMINOSH_SLICE, 127_024, 127_073);
static_range!(DOMINOSV_SLICE, 127_074, 127_123);
static_range!(EARTH_SLICE, 127_757, 127_760);
static_range!(JAP_SLICE, 65_382, 65_437);
static_range!(LARGE_LETTERS_SLICE, 127_462, 127_487);
static_range!(MOON_SLICE, 127_760, 127_773);
static_range!(NUMBERED_BALLS_SLICE, 127_312, 127_337);
static_range!(NUMBERED_CUBES_SLICE, 127_344, 127_369);
static_range!(PLANTS_SLICE, 127_793, 127_827);
static_range!(SMILE_SLICE, 128_512, 128_518);
static_range!(SHAPES_SLICE, 128_992, 129_003);
slice_skip_first!(MOON_NO_EARTH_SLICE, MOON_SLICE);

const CRAB_SLICE: &[u32] = &[129_408];

const fn multi_range<const N: usize>(ranges: &[(u32, u32)]) -> [u32; N] {
    let mut out = [0u32; N];
    let mut idx = 0;
    let mut r = 0;
    while r < ranges.len() {
        let (start, end) = ranges[r];
        let mut v = start;
        while v <= end {
            out[idx] = v;
            idx += 1;
            v += 1;
        }
        r += 1;
    }
    out
}

const ARROW_SLICE: &[u32] = &multi_range::<
    {
        (129_035 - 129_024 + 1)
            + (129_095 - 129_040 + 1)
            + (129_195 - 129_168 + 1)
            + (129_113 - 129_104 + 1)
    },
>(&[
    (129_024, 129_035),
    (129_040, 129_095),
    (129_168, 129_195),
    (129_104, 129_113),
]);

const CARDS_SLICE: &[u32] = &multi_range::<
    { (127_166 - 127_137 + 1) + (127_182 - 127_169 + 1) + (127_198 - 127_185 + 1) },
>(&[(127_137, 127_166), (127_169, 127_182), (127_185, 127_198)]);

const EMOJIS_SLICE: &[u32] = &multi_range::<
    { (129_400 - 129_292 + 1) + (129_482 - 129_402 + 1) + (129_535 - 129_484 + 1) },
>(&[(129_292, 129_400), (129_402, 129_482), (129_484, 129_535)]);

const ALPHA_NUM_SLICE: &[u32] = &{
    const LEN: usize = ALPHA_LOW_SLICE.len() + ALPHA_UP_SLICE.len() + NUM_SLICE.len();
    let mut arr = [0u32; LEN];
    let mut i = 0;
    while i < ALPHA_LOW_SLICE.len() {
        arr[i] = ALPHA_LOW_SLICE[i];
        i += 1;
    }
    let mut j = 0;
    while j < ALPHA_UP_SLICE.len() {
        arr[i + j] = ALPHA_UP_SLICE[j];
        j += 1;
    }
    let mut k = 0;
    while k < NUM_SLICE.len() {
        arr[i + j + k] = NUM_SLICE[k];
        k += 1;
    }
    arr
};

const ALL_SLICE: &[u32] = &all_slices!(
    // REMOVE ALPHA_NUM_SLICE (it’s inside ALPHA_LOW_SLICE and ALPHA_UP_SLICE)
    ALPHA_LOW_SLICE,
    ALPHA_UP_SLICE,
    NUM_SLICE,
    ARROW_SLICE,
    // REMOVED BIN_SLICE (it's inside NUM_SLICE)
    CARDS_SLICE,
    CLOCK_SLICE,
    // REMOVE CRAB_SLICE (it’s inside emojis)
    DOMINOSH_SLICE,
    DOMINOSV_SLICE,
    EARTH_SLICE,
    EMOJIS_SLICE,
    JAP_SLICE,
    LARGE_LETTERS_SLICE,
    // use de-overlapped moon slice
    MOON_NO_EARTH_SLICE,
    NUMBERED_BALLS_SLICE,
    NUMBERED_CUBES_SLICE,
    PLANTS_SLICE,
    SMILE_SLICE,
    SHAPES_SLICE,
);

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
/// for c in alpha_group.as_slice_u32() {
///     println!("{}", c);
/// }
/// println!("Max width: {}", alpha_group.width());
/// ```
#[derive(Debug, Clone, Copy)]
pub struct CharGroup {
    /// The name of this character group.
    pub name: GroupKind,

    /// The characters in this group as Unicode code points.
    pub chars: &'static [u32],

    /// The maximum display width of characters in this group.
    pub char_width: CharWidth,
}

impl CharGroup {
    pub const ALL: Self = CharGroup::new(GroupKind::All, ALL_SLICE, CharWidth::Double);
    pub const ALPHALOW: Self =
        CharGroup::new(GroupKind::Alphalow, ALPHA_LOW_SLICE, CharWidth::Single);
    pub const ALPHAUP: Self = CharGroup::new(GroupKind::Alphaup, ALPHA_UP_SLICE, CharWidth::Single);
    pub const ALPHANUM: Self =
        CharGroup::new(GroupKind::AlphaNum, ALPHA_NUM_SLICE, CharWidth::Single);
    pub const ARROW: Self = CharGroup::new(GroupKind::Arrow, ARROW_SLICE, CharWidth::Double);
    pub const BIN: Self = CharGroup::new(GroupKind::Bin, BIN_SLICE, CharWidth::Single);
    pub const CARDS: Self = CharGroup::new(GroupKind::Cards, CARDS_SLICE, CharWidth::Double);
    pub const CLOCK: Self = CharGroup::new(GroupKind::Clock, CLOCK_SLICE, CharWidth::Double);
    pub const CRAB: Self = CharGroup::new(GroupKind::Crab, CRAB_SLICE, CharWidth::Double);
    pub const DOMINOSH: Self =
        CharGroup::new(GroupKind::Dominosh, DOMINOSH_SLICE, CharWidth::Double);
    pub const DOMINOSV: Self =
        CharGroup::new(GroupKind::Dominosv, DOMINOSV_SLICE, CharWidth::Single);
    pub const EARTH: Self = CharGroup::new(GroupKind::Earth, EARTH_SLICE, CharWidth::Double);
    pub const EMOJIS: Self = CharGroup::new(GroupKind::Emojis, EMOJIS_SLICE, CharWidth::Double);
    pub const JAP: Self = CharGroup::new(GroupKind::Jap, JAP_SLICE, CharWidth::Single);
    pub const LARGELETTERS: Self = CharGroup::new(
        GroupKind::LargeLetters,
        LARGE_LETTERS_SLICE,
        CharWidth::Double,
    );
    pub const MOON: Self = CharGroup::new(GroupKind::Moon, MOON_SLICE, CharWidth::Double);
    pub const NUM: Self = CharGroup::new(GroupKind::Num, NUM_SLICE, CharWidth::Single);
    pub const NUMBEREDBALLS: Self = CharGroup::new(
        GroupKind::NumberedBalls,
        NUMBERED_BALLS_SLICE,
        CharWidth::Double,
    );
    pub const NUMBEREDCUBES: Self = CharGroup::new(
        GroupKind::NumberedCubes,
        NUMBERED_CUBES_SLICE,
        CharWidth::Double,
    );
    pub const PLANTS: Self = CharGroup::new(GroupKind::Plants, PLANTS_SLICE, CharWidth::Double);
    pub const SMILE: Self = CharGroup::new(GroupKind::Smile, SMILE_SLICE, CharWidth::Double);
    pub const SHAPES: Self = CharGroup::new(GroupKind::Shapes, SHAPES_SLICE, CharWidth::Double);
    pub const fn new(name: GroupKind, chars: &'static [u32], char_width: CharWidth) -> Self {
        Self {
            name,
            chars,
            char_width,
        }
    }

    #[inline]
    pub fn as_slice_u32(&self) -> &[u32] {
        self.chars.iter().as_slice()
    }

    #[inline]
    pub const fn width(&self) -> u8 {
        self.char_width as u8
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
    Alphalow,
    Alphaup,
    AlphaNum,
    Arrow,
    Bin,
    Cards,
    Clock,
    Crab,
    Dominosh,
    Dominosv,
    Earth,
    Emojis,
    Jap,
    LargeLetters,
    Moon,
    Num,
    NumberedBalls,
    NumberedCubes,
    Plants,
    Smile,
    Shapes,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn no_unexpected_duplicates() {}

    #[test]
    fn static_range_creates_correct_values() {
        assert_eq!(ALPHA_LOW_SLICE, &(97u32..=122).collect::<Vec<_>>()[..]);
        assert_eq!(ALPHA_UP_SLICE, &(65u32..=90).collect::<Vec<_>>()[..]);
        assert_eq!(NUM_SLICE, &(48u32..=57).collect::<Vec<_>>()[..]);
    }

    #[test]
    fn multi_range_creates_correct_values() {
        let expected_arrows: Vec<u32> = (129_024..=129_035)
            .chain(129_040..=129_095)
            .chain(129_168..=129_195)
            .chain(129_104..=129_113)
            .collect();
        assert_eq!(ARROW_SLICE, &expected_arrows);

        let expected_cards: Vec<u32> = (127_137..=127_166)
            .chain(127_169..=127_182)
            .chain(127_185..=127_198)
            .collect();
        assert_eq!(CARDS_SLICE, &expected_cards);
    }

    #[test]
    fn no_unexpected_duplicates_in_all_slice() {
        let mut seen = std::collections::HashSet::new();
        for (i, &cp) in ALL_SLICE.iter().enumerate() {
            assert!(
                seen.insert(cp),
                "Duplicate codepoint found: U+{cp:04X} at index {i}"
            );
        }
    }

    #[test]
    fn group_consts_match_expected_slices() {
        assert_eq!(CharGroup::ALL.chars, ALL_SLICE);
        assert_eq!(CharGroup::ALPHALOW.chars, ALPHA_LOW_SLICE);
        assert_eq!(CharGroup::ALPHAUP.chars, ALPHA_UP_SLICE);
        assert_eq!(CharGroup::ALPHANUM.chars, ALPHA_NUM_SLICE);
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
    width_calculation!(width_calculation_for_numberedcubes, NUMBEREDCUBES, 2);
    width_calculation!(width_calculation_for_plants, PLANTS, 2);
    width_calculation!(width_calculation_for_smile, SMILE, 2);
    width_calculation!(width_calculation_for_shapes, SHAPES, 2);
}
