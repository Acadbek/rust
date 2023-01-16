pub fn run() {
    let mut count: i32 = 0;

    // Infinite loop

    loop {
        count += 1;
        println!("{}", count);
        if count == 20 {
            break;
        };
    }

    // While loop (FizzBuzz)

    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz")
        } else if count % 3 == 0 {
            println!("fizz")
        } else if count % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", count)
        }
        count += 1;
    }

    // for range

    for x in 0..100 {
        if x % 15 == 0 {
            println!("FizzBuzz")
        } else if x % 3 == 0 {
            println!("fizz")
        } else if x % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", x)
        }
    }
}
