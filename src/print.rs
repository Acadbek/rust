pub fn run() {
  println!("hello from print file");

  println!("hello im {0} and {0}", 22);

  println!("im {name}", name = "asad"); // Named args

  println!("{:?}", (1, true, false));

  println!("10 + 10 = {}", 10 + 10)
}