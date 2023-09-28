// Fix the error with the use of define_x
fn main() {
    let x = "";
    let _x_hello = define_x(x);
    define_x(x);
    println!("{_x_hello} world!")
}

fn define_x(_x:&str) -> &str {
    let x = "hello";
    x
}
