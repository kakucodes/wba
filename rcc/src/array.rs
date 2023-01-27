use std::mem::size_of_val;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    numbers[2] = 20;

    println!("{:?}", numbers);
    println!("single {}", numbers[0]);
    println!("len {}", numbers.len());

    println!("Array occupies {} bytes", size_of_val(&numbers));

    println!("Slice: {:?}", &numbers[0..2]);
}
