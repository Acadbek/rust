pub fn run() {
    // if else
    let age: i32 = 18;
    let check_id: bool = false;

    if age > 18 && check_id {
        println!("what would you like to drink?")
    } else if age < 21 && check_id {
        println!("21")
    } else {
        println!("-18")
    }

    // shorthand
    let is_of_age: bool = if age >= 31 { true } else { false };
    println!("Is Of Age {}", is_of_age);
}
