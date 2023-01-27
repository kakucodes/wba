struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct TColor(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}
impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string()
    }

    fn to_tuple(&self) -> (String, String) {
        (self.first_name.to_string(), self.last_name.to_string())
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    c.red = 200;
    println!("color: {} {} {}", c.red, c.green, c.blue);

    let mut c = TColor(255, 0, 0);
    println!("color: {} {} {}", c.0, c.1, c.2);

    let mut p = Person::new("Jon", "Dough");
    println!("Person {} ", p.full_name());

    p.set_last_name("Doh");
    println!("Person {:?}", p.to_tuple());
}
