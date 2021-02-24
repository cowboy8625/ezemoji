use std::collections::HashMap;
#[derive(PartialEq, Eq, Hash)]
pub enum EmojiTypes {
    Smile,
    Moon,
    Earth,
    Plant,
    Clock,
    Shape,
    Arrow,
}
// Turn this into a trait
pub struct EZEmojis {
    list: HashMap<EmojiTypes, Vec<u32>>,
}

impl EZEmojis {
    pub fn new() -> Self {
        let mut hash = HashMap::new();
        // FIXME:
        hash.insert(EmojiTypes::Smile, (127512..=128591).collect());

        hash.insert(EmojiTypes::Moon, (127760..=127773).collect());
        hash.insert(EmojiTypes::Earth, (127757..=127760).collect());
        hash.insert(EmojiTypes::Plant, (127793..=127827).collect());
        hash.insert(EmojiTypes::Clock, (128336..=128359).collect());
        hash.insert(EmojiTypes::Shape, (128992..=129003).collect());
        hash.insert(EmojiTypes::Arrow, create_arrows());
        Self { list: hash }
    }

    pub fn get_char(&self, key: &EmojiTypes) -> Option<Vec<char>> {
        self.list.get(key).map(
            |n| n.iter().map(|num| std::char::from_u32(*num).unwrap_or(' ')).collect()
            )
    }

    pub fn get_u32(&self, key: &EmojiTypes) -> Option<&Vec<u32>> {
        self.list.get(key)
    }
}

fn create_arrows() -> Vec<u32> {
    let mut a: Vec<u32> = (129024..=129035).collect();
    a.append(&mut (129040..=129095).collect());
    a.append(&mut (129168..=129195).collect());
    a.append(&mut (129168..=129195).collect());
    a.append(&mut (129104..=129113).collect());
    a
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
