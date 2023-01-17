enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right"),
    }
}

pub fn run() {
    let avatar1 = Movement::Down;
    let avatar2 = Movement::Left;
    let avatar3 = Movement::Up;
    let avatar4 = Movement::Right;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
