//! EZEmojis is a Work in progress made for [rust-rain](https://rusty-rain.xyz) program
//! so just a warning that the api may change a lot in the coming updates.
//!
//! If you have any suggestion feel free to open a issue on the
//! [github page](https://github.com/cowboy8625/ezemoji)

/// All Default Implemented Emojis Groups.
///
/// Some of these may not show up for you depending on your Font I think.
/// correct me if I am wrong.
///
/// ```rust
/// use ezemoji::EmojiGroups;
/// fn main() {
///     let group = EmojiGroups::All;
///     println!("{:?}", group.as_vec_char());
/// }
/// ```
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
    HorizontalDominos,
    VerticalDominos,
    Cards,
    NumberedBalls,
    NumberedCubes,
    LargeLetter,
    Crab,
    Numbers,
    Bin,
    LowerAlpha,
    UpperAlpha,
    AlphaNumeric,
    All,
}

impl EmojiGroups {
    /// Returns a `Vec<char>`
    /// ```rust
    /// use ezemoji::EmojiGroups;
    /// fn main() {
    ///     let group = EmojiGroups::Crab;
    ///     println!("{:?}", group.as_vec_char());
    /// }
    pub fn as_vec_char(&self) -> Vec<char> {
        self.as_vec_u32()
            .iter()
            .map(|num| std::char::from_u32(*num).unwrap_or(' '))
            .collect::<Vec<char>>()
    }

    /// Returns a `Vec<u32>`
    /// ```rust
    /// use ezemoji::EmojiGroups;
    /// fn main() {
    ///     let group = EmojiGroups::Crab;
    ///     println!("{:?}", group.as_vec_u32());
    /// }
    pub fn as_vec_u32(&self) -> Vec<u32> {
        let vec = match self {
            Self::Emojis => create_emojis(),
            Self::Japanese => create_jap(),
            Self::Smile => create_smile(),
            Self::Moon => create_moon(),
            Self::Earth => create_earth(),
            Self::Plant => create_plant(),
            Self::Clock => create_clock(),
            Self::Shape => create_shape(),
            Self::Arrow => create_arrows(),
            Self::HorizontalDominos => create_hdominos(),
            Self::VerticalDominos => create_vdominos(),
            Self::Cards => create_cards(),
            Self::NumberedBalls => create_letter_ball_solid(),
            Self::NumberedCubes => create_letter_cube_solid(),
            Self::LargeLetter => create_bold_large_letters(),
            Self::Crab => create_crab(),
            Self::Numbers => create_numbers(),
            Self::Bin => create_bin(),
            Self::LowerAlpha => create_lower_alpha(),
            Self::UpperAlpha => create_upper_alpha(),
            Self::AlphaNumeric => create_alpha_numeric(),
            Self::All => create_all(),
        };
        vec
    }
}

pub fn create_emojis() -> Vec<u32> {
    let mut e: Vec<_> = (129292..=129400).collect();
    e.append(&mut (129402..=129482).collect());
    e.append(&mut (129484..=129535).collect());
    e
}

pub fn create_smile() -> Vec<u32> {
    (128512..=128518).collect()
}

pub fn create_jap() -> Vec<u32> {
    (65382..=65437).collect()
}

pub fn create_moon() -> Vec<u32> {
    (127760..=127773).collect()
}

pub fn create_earth() -> Vec<u32> {
    (127757..=127760).collect()
}

pub fn create_plant() -> Vec<u32> {
    (127793..=127827).collect()
}

pub fn create_clock() -> Vec<u32> {
    (128336..=128359).collect()
}

pub fn create_shape() -> Vec<u32> {
    (128992..=129003).collect()
}

pub fn create_crab() -> Vec<u32> {
    vec![129408]
}

pub fn create_arrows() -> Vec<u32> {
    let mut a: Vec<u32> = (129024..=129035).collect();
    a.append(&mut (129040..=129095).collect());
    a.append(&mut (129168..=129195).collect());
    a.append(&mut (129168..=129195).collect());
    a.append(&mut (129104..=129113).collect());
    a
}

pub fn create_all() -> Vec<u32> {
    let mut a = create_moon();
    a.append(&mut create_earth());
    a.append(&mut create_plant());
    a.append(&mut create_clock());
    a.append(&mut create_shape());
    a.append(&mut create_arrows());
    a.append(&mut create_hdominos());
    a.append(&mut create_vdominos());
    a.append(&mut create_cards());
    a.append(&mut create_crab());
    a.append(&mut create_letter_ball_solid());
    a.append(&mut create_letter_cube_solid());
    a.append(&mut create_bold_large_letters());
    a.append(&mut create_alpha_numeric());
    a
}

pub fn create_hdominos() -> Vec<u32> {
    (127024..=127073).collect()
}

pub fn create_vdominos() -> Vec<u32> {
    (127074..=127123).collect()
}

pub fn create_cards_hearts() -> Vec<u32> {
    (127137..=127166).collect()
}

pub fn create_cards_diamonds() -> Vec<u32> {
    (127169..=127182).collect()
}

pub fn create_cards_clubs() -> Vec<u32> {
    (127185..=127198).collect()
}

pub fn create_cards() -> Vec<u32> {
    let mut h = create_cards_hearts();
    h.append(&mut create_cards_clubs());
    h.append(&mut create_cards_diamonds());
    h
}

pub fn create_letter_ball_solid() -> Vec<u32> {
    (127312..=127337).collect()
}

pub fn create_letter_cube_solid() -> Vec<u32> {
    (127344..=127369).collect()
}

pub fn create_bold_large_letters() -> Vec<u32> {
    (127462..=127487).collect()
}

pub fn create_numbers() -> Vec<u32> {
    (48..=57).collect()
}

pub fn create_bin() -> Vec<u32> {
    (48..=49).collect()
}

pub fn create_lower_alpha() -> Vec<u32> {
    (97..=122).collect()
}

pub fn create_upper_alpha() -> Vec<u32> {
    (65..=90).collect()
}

pub fn create_alpha_numeric() -> Vec<u32> {
    let mut a = create_numbers();
    a.append(&mut create_upper_alpha());
    a.append(&mut create_lower_alpha());
    a
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_numbers() {
        assert_eq!(EmojiGroups::Numbers.as_vec_char(), vec!['0','1','2','3','4','5','6','7','8','9']);
    }
    #[test]
    fn test_bin() {
        assert_eq!(EmojiGroups::Bin.as_vec_char(), vec!['0','1']);
    }
    #[test]
    fn test_lower_alpha() {
        assert_eq!(EmojiGroups::LowerAlpha.as_vec_char(), ('a'..='z').collect::<Vec<char>>());
    }
    #[test]
    fn test_upper_alpha() {
        assert_eq!(EmojiGroups::UpperAlpha.as_vec_char(), ('A'..='Z').collect::<Vec<char>>());
    }
    #[test]
    fn test_alpha_numeric() {
        let mut right = ('0'..='9').collect::<Vec<char>>();
        right.append(&mut ('A'..='Z').collect());
        right.append(&mut ('a'..='z').collect());
        assert_eq!(EmojiGroups::AlphaNumeric.as_vec_char(), right);
    }
}
