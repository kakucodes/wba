pub fn run() {
    greeting("Hello", "kaku");

    println!("sum {}", add(5, 7));

    let z = 2;
    println!("C sum: {}", (|x: i32, y: i32| x + y + z)(1, 2));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
