pub fn run() {
    greeting("Hello", "Asad");
    println!("{}", sum(5, 5));

    // Closure
    let n3: i32 = 2;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum: {}", add_nums(2, 2));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {} nice to meet you", greet, name,);
}

fn sum(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
