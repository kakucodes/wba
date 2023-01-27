pub fn run() {
    let mut hello = String::from("Hello ");

    println!("Length: {}", hello.len());
    hello.push('W');
    hello.push_str("orld!");
    println!("{}", hello);
    println!(
        "is empty? {}. contains ello? {}",
        hello.is_empty(),
        hello.contains("ello")
    );

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}
