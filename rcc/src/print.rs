pub fn run() {
    println!("basic print");

    println!("with a {}", "param");

    println!("with {0} {1}", "many", "params");

    println!("named {params}", params = "things");

    println!("binary {:b}", 100);

    println!("debug {:?}", vec!["first", "second"]);
}
