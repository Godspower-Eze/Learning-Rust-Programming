//Array: Fixed list of same data types

use std::mem;

pub fn run() {

    // i32 is the datatype and 5 is the length 
     let mut numbers:  [i32; 5] = [1,2,3,4,5];

     //Re-assign

     numbers[2] = 20;

     println!("{:?}", numbers);

     //Get single value
     println!("Single Value: {}", numbers[0]);

     //Get array length
     println!("Array length: {}", numbers.len());

     //Arrays of stack allocated
     println!("Array occupies {} bytes", mem::size_of_val(&numbers));

     //Get slice
     let slice: &[i32] = &numbers;
     println!("Slice: {:?}", slice);

     let slice: &[i32] = &numbers[0..2];
     println!("Slice: {:?}", slice);
}