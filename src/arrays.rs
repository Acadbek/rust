use std::mem;

pub fn run() {
	let nums: [i32; 4] = [1,2,3,4];
	println!("{:?}", nums);

	// occupies arr
	println!("{} bytes", mem::size_of_val(&nums));

	// slice
	let two = 2;
	let slice: &[i32] = &nums[0..two];
	
	println!("1-{:?} 2-{:?} 1-{:?} 2-{:?}", 
		nums, slice, mem::size_of_val(&nums), mem::size_of_val(&slice));
}