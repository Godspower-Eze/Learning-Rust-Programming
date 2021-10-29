// Primitive str = Immutable fized-length string somewhere in memory
// String = Growable, heap allocated data structure - Use when you need modify or own string data

pub fn run(){

    // Immutable and fixed-length
    let hello = "Hello";

    // Mutable 
    let mut hello = String::from("Hello ");

    //Get length
    println!("The length of hello is {}",hello.len());

    //Add Characters to hello. Push takes char datatypes
    hello.push('w');

    //Push String
    hello.push_str("orld");

    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //Check if empty
    println!("Check if empty: {}", hello.is_empty());

    //Contains
    println!("Contains 'world': {}", hello.contains("World"));

    //Replace 
    println!("Replace: {}", hello.replace("world", "There"));

    //Loop through string
    for word in hello.chars(){
        println!("{}", word)
    }

    //Loop through string by whitespace
    for word in hello.split_whitespace(){
        println!("{}", word)
    }

    //Create string with capacity
    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');

    println!("s: {}", s);

    //Assertion testing
    assert_eq!(2, s.len());

}