use std::io;

fn main() {
    let x = i32::MAX as i64 + 1; 
    let y = 10_u8;  

    let z = x % y as i64;
    println!("Hello, world! x:{} y:{} z:{}", x, y, z);

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error reading");
    let int_input: i64 = input.trim().parse().expect("Error converting string");

    println!("{} + 2 = {}", input.trim(), int_input + 2);


}
