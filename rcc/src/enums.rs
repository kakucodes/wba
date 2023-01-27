enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("moving on up"),
        Movement::Down => println!("moving donw"),
        Movement::Left => println!("moving  left"),
        Movement::Right => println!("moving right"),
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    move_avatar(Movement::Right);
}
