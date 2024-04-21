//! # EZEmoji
//!
//! This Project is to make finding Emoji's and Char Groups a bit easier. (WIP)
//!
//! EZEmojis is a Work in progress made for use in
//! **rusty-rain** which is a **CMatrix** clone
//! #### Rusty Rain
//! - [Website](https://rusty-rain.xyz)
//! - [Github](https://github.com/cowboy/rusty-rain)
//! - [Crates.io](https://crates.io/crates/rusty-rain)
//!
//! #### Warning
//! The api may change a lot in the coming updates.
//!
//! ## Example
//! ```rust
//! # use ezemoji::{EZEmoji, Crab};
//! # fn main() {
//!     println!("{:?}", Crab.as_vec_char());
//! # }
//! ```
//!
//!
//! The `trait` `EZEmoji` is provided to glue all the types together.
//! You can implement it for your own type like
//!
//! ```rust
//! use ezemoji::EZEmoji;
//!
//! struct MyEmojiGroup;
//! impl EZEmoji for MyEmojiGroup {
//!     fn as_vec_u32(&self) -> Vec<u32> {
//!         vec![96]
//!     }
//! }
//! ```
//!
//! ### HELP
//! If you have any ideas for this crate make an [issue](https://github.com/cowboy8625/ezemoji/issues) and lets talk about it.

/// A trait to turn the Type into a `Vec`
pub trait EZEmoji {
    /// Returns a `Vec<u32>`
    fn as_vec_u32(&self) -> Vec<u32>;

    /// Returns a `Vec<char>`
    fn as_vec_char(&self) -> Vec<char> {
        self.as_vec_u32()
            .iter()
            .map(|num| std::char::from_u32(*num).unwrap_or(' '))
            .collect::<Vec<char>>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Emojis;
impl EZEmoji for Emojis {
    fn as_vec_u32(&self) -> Vec<u32> {
        let mut emojis: Vec<_> = (129292..=129400).collect();
        emojis.extend_from_slice(&(129402..=129482).collect::<Vec<u32>>());
        emojis.extend_from_slice(&(129484..=129535).collect::<Vec<u32>>());
        emojis
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Japanese;
impl EZEmoji for Japanese {
    fn as_vec_u32(&self) -> Vec<u32> {
        (65382..=65437).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Smile;
impl EZEmoji for Smile {
    fn as_vec_u32(&self) -> Vec<u32> {
        (128512..=128518).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Moon;
impl EZEmoji for Moon {
    fn as_vec_u32(&self) -> Vec<u32> {
        (127760..=127773).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Earth;
impl EZEmoji for Earth {
    fn as_vec_u32(&self) -> Vec<u32> {
        (127757..=127760).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Plant;
impl EZEmoji for Plant {
    fn as_vec_u32(&self) -> Vec<u32> {
        (127793..=127827).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Clock;
impl EZEmoji for Clock {
    fn as_vec_u32(&self) -> Vec<u32> {
        (128336..=128359).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shape;
impl EZEmoji for Shape {
    fn as_vec_u32(&self) -> Vec<u32> {
        (128992..=129003).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Arrow;
impl EZEmoji for Arrow {
    fn as_vec_u32(&self) -> Vec<u32> {
        let mut arrow = (129024..=129035).collect::<Vec<u32>>();
        arrow.extend_from_slice(&(129040..=129095).collect::<Vec<u32>>());
        arrow.extend_from_slice(&(129168..=129195).collect::<Vec<u32>>());
        arrow.extend_from_slice(&(129168..=129195).collect::<Vec<u32>>());
        arrow.extend_from_slice(&(129104..=129113).collect::<Vec<u32>>());
        arrow
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HorizontalDominos;
impl EZEmoji for HorizontalDominos {
    fn as_vec_u32(&self) -> Vec<u32> {
        (127024..=127073).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerticalDominos;
impl EZEmoji for VerticalDominos {
    fn as_vec_u32(&self) -> Vec<u32> {
        (127074..=127123).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cards;
impl EZEmoji for Cards {
    fn as_vec_u32(&self) -> Vec<u32> {
        let mut hearts = (127137..=127166).collect::<Vec<u32>>();
        let diamonds = (127169..=127182).collect::<Vec<u32>>();
        let clubs = (127185..=127198).collect::<Vec<u32>>();
        hearts.extend_from_slice(&diamonds);
        hearts.extend_from_slice(&clubs);
        hearts
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberedBalls;
impl EZEmoji for NumberedBalls {
    fn as_vec_u32(&self) -> Vec<u32> {
        (127312..=127337).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberedCubes;
impl EZEmoji for NumberedCubes {
    fn as_vec_u32(&self) -> Vec<u32> {
        (127344..=127369).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LargeLetter;
impl EZEmoji for LargeLetter {
    fn as_vec_u32(&self) -> Vec<u32> {
        (127462..=127487).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Crab;
impl EZEmoji for Crab {
    fn as_vec_u32(&self) -> Vec<u32> {
        vec![129408]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Numbers;
impl EZEmoji for Numbers {
    fn as_vec_u32(&self) -> Vec<u32> {
        (48..=57).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bin;
impl EZEmoji for Bin {
    fn as_vec_u32(&self) -> Vec<u32> {
        (48..=49).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LowerAlpha;
impl EZEmoji for LowerAlpha {
    fn as_vec_u32(&self) -> Vec<u32> {
        (97..=122).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpperAlpha;
impl EZEmoji for UpperAlpha {
    fn as_vec_u32(&self) -> Vec<u32> {
        (65..=90).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlphaNumeric;
impl EZEmoji for AlphaNumeric {
    fn as_vec_u32(&self) -> Vec<u32> {
        let mut alpha_numeric = Numbers.as_vec_u32();
        alpha_numeric.extend_from_slice(&UpperAlpha.as_vec_u32());
        alpha_numeric.extend_from_slice(&LowerAlpha.as_vec_u32());
        alpha_numeric
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenSource;
impl EZEmoji for OpenSource {
    fn as_vec_u32(&self) -> Vec<u32> {
        let mut open_source = (62208..=62325).collect::<Vec<u32>>(); // All Font Logos
        open_source.retain(|&x| x != 62210); // Remove Apple logo
        open_source.extend_from_slice(&[59205, 59257]); // Devicons
        open_source.extend_from_slice(&[58930, 58931, 59054]); // Nerd Fonts custom icons
        open_source.extend_from_slice(&[58975]); // Seti-UI
        open_source.extend_from_slice(&[983211, 983714, 984444]); // Material Design Icons
        open_source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllEmojis;
impl EZEmoji for AllEmojis {
    fn as_vec_u32(&self) -> Vec<u32> {
        let mut all = Moon.as_vec_u32();
        all.extend_from_slice(&Earth.as_vec_u32());
        all.extend_from_slice(&Plant.as_vec_u32());
        all.extend_from_slice(&Clock.as_vec_u32());
        all.extend_from_slice(&Shape.as_vec_u32());
        all.extend_from_slice(&Arrow.as_vec_u32());
        all.extend_from_slice(&HorizontalDominos.as_vec_u32());
        all.extend_from_slice(&VerticalDominos.as_vec_u32());
        all.extend_from_slice(&Cards.as_vec_u32());
        all.extend_from_slice(&Crab.as_vec_u32());
        all.extend_from_slice(&NumberedBalls.as_vec_u32());
        all.extend_from_slice(&NumberedCubes.as_vec_u32());
        all.extend_from_slice(&LargeLetter.as_vec_u32());
        all.extend_from_slice(&AlphaNumeric.as_vec_u32());
        all.extend_from_slice(&OpenSource.as_vec_u32());
        all
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_numbers() {
        assert_eq!(
            Numbers.as_vec_char(),
            vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
        );
    }
    #[test]
    fn test_bin() {
        assert_eq!(Bin.as_vec_char(), vec!['0', '1']);
    }
    #[test]
    fn test_lower_alpha() {
        assert_eq!(LowerAlpha.as_vec_char(), ('a'..='z').collect::<Vec<char>>());
    }
    #[test]
    fn test_upper_alpha() {
        assert_eq!(UpperAlpha.as_vec_char(), ('A'..='Z').collect::<Vec<char>>());
    }
    #[test]
    fn test_alpha_numeric() {
        let mut right = ('0'..='9').collect::<Vec<char>>();
        right.append(&mut ('A'..='Z').collect());
        right.append(&mut ('a'..='z').collect());
        assert_eq!(AlphaNumeric.as_vec_char(), right);
    }
}
