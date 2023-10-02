// Make it work
use std::mem::size_of_val; //Size in bytes
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4); // 'a' = 4 in bytes

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4); // '中' = 4 in bytes

    println!("Success!");
}

