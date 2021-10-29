use std::env;

pub fn run(){
     let args: Vec<String> = env::args().collect();
     let name = "Godspower";

     let command = args[1].clone();

     if command == "hello"{
         println!("Hi {}, how are you?", name);
     }

}