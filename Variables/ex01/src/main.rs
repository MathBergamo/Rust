// Fix the error below with least amount of modification to the code
//A variable can be used only if it has been initialized.
fn main() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let y: i32 = 5; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, y);
    println!("Success!");
}
