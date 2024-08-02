// use std::io;

// fn main() {
//     find_c();
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

//ownership 

// fn main() {
//    let x = 5;
//    let y = x;

//     println!("{}",  x==y)
// }

// fn main() {
//     let s = String::from("hello"); 

//     takes_ownership(s);          

//     let x = 5;                   

//     makes_copy(x);  
    

// } 
// fn takes_ownership(some_string: String) { 
//     println!("{some_string}");
// }

// fn makes_copy(some_integer: i32) { 
//     println!("{some_integer}");
// }

// fn main() {
//     let s1 = String::from("hello");

//     let (val, len) = get_len_val(s1);

//     println!("{}, {}", val, len)
// }

// fn get_len_val(val: String) -> (String, usize) {
//     let len = val.len();

//     (val, len)
// }

//refernce

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("{}, {}", len, s1)
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let mut s = String::from("hello");
    
//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello world");

//     let r1 = &mut s;

//     println!("{}", r1)
// }


// fn main() {
//     let mut s = String::from("hello world");
//     println!("{}", first_word(&s));

//     s.clear();
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

//slices

fn main() {
    let string = String::from("hello world");

    let first_part_of_str = &string[0..5];
    let second_part_of_str = &string[6..11];

    println!("{} {}", first_part_of_str, second_part_of_str)
}