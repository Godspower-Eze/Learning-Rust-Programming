// Variables hold primitive data or reference to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Godspower";
    let age = 20;

    let mut mutable_age = 20;

    println!("My name is {} and I am {} years old and my mutable age is {}", name, age, mutable_age);

    mutable_age = 28;

    println!("My name is {} and I am {} years old and my mutable age is {}", name, age, mutable_age);

    //Define Constant
    // When using the const keyword you need to declare the type

    const ID: i32 = 001;

    println!("ID: {}", ID);

    //Assign multiple variables
    let (my_name, my_age) = ("Godspower", 20);

    println!("{} is {}", my_name, my_age);
}  