pub fn run(){
  let mut array: Vec<i32> = vec![1,2,3,4];

  println!("{:?}", array);

  for x in array.iter() {
    println!("{}", x)
  }

  for x in array.iter_mut() {
    *x *= 2;
  }
  println!("{:?}", array);
}