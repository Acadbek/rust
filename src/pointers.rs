pub fn run() {
    let mut arr1 = [1, 2, 3];
    let mut arr2 = arr1;
    arr2[0] = 5;

    let vec1 = vec![1, 3, 4];
    let vec2 = &vec1;

    println!("{:?}", (vec2))
}
