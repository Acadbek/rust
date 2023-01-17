// Traditional struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// Tuple struct

// struct Color(u16, u8, u8);

struct Person {
    name: String,
    last_name: String,
}

impl Person {
    fn new(n: &str, l: &str) -> Person {
        Person {
            name: n.to_string(),
            last_name: l.to_string(),
        }
    }
    // Get full name
    fn get_full_name(&self) -> String {
        format!("{} {}", self.name, self.last_name)
    }

    // name to tuple
    fn toTuple (&self)
}

pub fn run() {
    // let c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };
    // println!("{} {} {}", c.red, c.green, c.blue);

    // c.red = 000;

    // let mut c = Color(255, 0, 0);

    // c.0 = 900;
    // println!("{} {} {}", c.0, c.1, c.2);

    let p = Person::new("Asad", "Nosir");
    let p2 = Person::new("Abdulloh", "Zakkokiy");
    println!("{}", (p2.get_full_name()));
}
