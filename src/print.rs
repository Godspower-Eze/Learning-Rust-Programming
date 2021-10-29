pub fn run() {
    // Print to console
    println!("Hello from print.rs file");

    // Passing in a number
    println!("Number {}", 1);

    //Passing in multiple values
    println!("{} is from {}", "Godspower", "Anambra");

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Godspower", "Anambra", "Code");

    //Named Arguments
    println!("{name} likes to {activity}", name="Godspower", activity = "Code");

    //Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 8);

    //Placeholder for debug traits
    println!("{:?}",(12, true, "Godspower"));

    //Basic maths
    println!("10 + 10 = {}", 10 + 10);
}