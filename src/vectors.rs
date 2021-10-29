//Vectors: Resizable arrays

use std::mem;

pub fn run() {

    // i32 is the datatype and 5 is the length 
     let mut numbers:  Vec<i32> = vec![1,2,3,4,5];

     //Re-assign

     numbers[2] = 20;

     println!("{:?}", numbers);

     //Add on to vectors
     numbers.push(5);
     numbers.push(6);

     //Pop off last values
     numbers.pop();

     //Get single value
     println!("Single Value: {}", numbers[0]);

     //Get vector length
     println!("Vector length: {}", numbers.len());

     //Vectors are  stack allocated
     println!("Array occupies {} bytes", mem::size_of_val(&numbers));

     //Get slice
     let slice: &[i32] = &numbers;
     println!("Slice: {:?}", slice);

     let slice: &[i32] = &numbers[0..2];
     println!("Slice: {:?}", slice);

     //Loop through vector items
     for x in numbers.iter(){
         println!("Numbers: {}", x)
     }

    //Loop through vector items
    for x in numbers.iter_mut(){
        *x *= 2
    }

    println!("Number vectors: {:?}", numbers);
}