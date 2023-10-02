// Modify `4` in assert to make it work
use std::mem::size_of_val;
fn main() {
    let unit: () = (); //unit = 0 bytes
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}
