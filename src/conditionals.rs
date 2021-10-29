pub fn run(){
    let age: u8 = 21;
    let check_id: bool = true;
    let knows_person_of_age: bool = true;

    //If/Else

    if age >= 21 && check_id || knows_person_of_age{
        println!("Bartender: What would you like to drink?")
    }else if age < 21 && check_id{
        println!("Bartender: please you have to leave")   
    }else{
        println!("Bartender: please, i would need to see your ID")
    }

    //Shorthand IF
    let is_of_age = if age >= 21 {true} else {false};

    println!("Value of is_of_age: {}", is_of_age);
}