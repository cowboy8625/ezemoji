use std::collections::HashMap;

// Turn this into a trait
pub struct EZEmojis {
    list: HashMap<&'static str, Vec<u32>>,
}

impl EZEmojis {
    pub fn new() -> Self {
        let mut hash = HashMap::new();
        // FIXME:
        hash.insert("smile", (127512..=128591).collect());

        hash.insert("moon", create_moon());
        hash.insert("earth", create_earth());
        hash.insert("plant", create_plant());
        hash.insert("clock", create_clock());
        hash.insert("shape", create_shape());
        hash.insert("arrow", create_arrows());
        hash.insert("all",  create_all());
        Self { list: hash }
    }

    pub fn add(&mut self, key: &'static str, value: Vec<u32>) {
        self.list.insert(key, value);
    }

    pub fn get_char(&self, key: &str) -> Option<Vec<char>> {
        self.list.get(key).map(
            |n| n.iter().map(|num| std::char::from_u32(*num).unwrap_or(' ')).collect()
            )
    }

    pub fn get_u32(&self, key: &str) -> Option<&Vec<u32>> {
        self.list.get(key)
    }
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let e = EZEmojis::new();
        assert_eq!(e.get_u32("shape"), Some(&vec![128992, 128993, 128994, 128995, 128996, 128997, 128998, 128999, 129000, 129001, 129002, 129003]));
    }
}
