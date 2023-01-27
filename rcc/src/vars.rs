pub fn run() {
    let name = "kaku";
    let mut age = 155;

    println!("my name is {name} and i'm {age}", name = name, age = age);
    age = 156;
    println!("my name is {name} and i'm {age}", name = name, age = age);

    const ID: i8 = 1;
    println!("id: {}", ID);

    let (my_name, my_age) = (name, age);

    println!("my name {} my age {}", my_name, my_age);
}
