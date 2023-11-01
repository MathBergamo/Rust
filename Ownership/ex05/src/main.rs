// Don't use clone ,use copy instead
fn main() {
    let x = (1, 2, (), "hello".to_string());
    let y = &x;
    println!("{:?}, {:?}", x, y);

    test()
}

fn test() {
    let x = (1, 2, (), "hello"); //only values of stack memory, fixed size.
    let y = x;
    println!("{:?}, {:?}", x, y);
}