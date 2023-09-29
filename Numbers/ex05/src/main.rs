// Fix errors and panics to make it work
fn main() {
    let v1 = 251_u16 + 8; //v1 was u8
    let v2 = i16::checked_add(151, 8).unwrap(); //v2 was i8
    println!("{},{}",v1,v2);
}
