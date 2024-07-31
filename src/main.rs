// use std::io;

// fn main() {
//     find_celsius();
// }

// fn find_celsius() {
//     let mut input = String::new();

//     println!("Enter Celsius:");
    
//     io::stdin()
//     .read_line(&mut input)
//     .expect("Falied to read line !.");

//     let num: i32 = input.trim().parse().expect("Invalid input");

//      let celsius_temprature = (num * 9/5) + 32;

//      println!("celsius degree: {}", celsius_temprature)

// }

//fibonacci 

// fn main() {
//     println!("fibonacci_sequence : {}", fibonacci_sequence(6));
// }

// fn fibonacci_sequence(num: i32) -> i32 {
//     if num <= 1 {
//       return num;
//     }
//     fibonacci_sequence(num-1) + fibonacci_sequence(num-2)
// }

