pub fn run() {
  let person: (i32, &str, i8) = (77, "Nosirjon", 22);
  println!("{} {} {}", person.0, person.1, person.2)
}