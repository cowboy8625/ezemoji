use ezemoji::EZEmojis;
fn main() {
    let e = EZEmojis::new();
    println!("{:?}", e.get_char("shape"));
}
