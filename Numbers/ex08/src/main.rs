fn main() {
    assert!(0.1_f32+0.2_f32==0.3 as f32); //f32 was added as a type to make the values less precisely
    // By default, the numbers 0.1, 0.2 and 0.3 was f64, making the real value super precisely (something like 0.20000002.)


    println!("Success!");
}

