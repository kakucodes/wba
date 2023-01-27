pub fn run() {

    let age = 18;
    let check_id = false;
    let knows_person_of_age = true

    if age > 21 && check_id || knows_person_of_age {
        println("B: What would you like to drink");
    } else if (age < 21 && check_id) {
        println("B:Sorry you have to leave");
    } else {
        println!("B: I'll need to see your ID");
    }

    let is_of_age = if age >= 21 {true} else {false}
    println!("is of age {}", is_of_age);
}