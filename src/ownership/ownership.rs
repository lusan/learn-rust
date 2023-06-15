fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}. ", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}



// fn main() {
//     let s1 = gives_ownership(); // gives_ownership moves its return
//                                 // value into s1
    
//     // s2 comes into scope
//     let s2 = String::from("hello");

//     // s2 is moved into
//     // takes_and_gives_back, which also moves its return value into s3
//     let s3 = takes_and_gives_back(s2);
// }
// fn gives_ownership() -> String {
//     // gives_ownership will move its return value into the function 
//     // that calls it

//     let some_string = String::from("yours"); // some_string comes into scope

//     // some_string is returned and moves out to the calling function
//     some_string
// }
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//     // scope

//     a_string  // a_string is returned and moves out to the calling function
// }

// fn main() {
//     let s = String::from("hello"); // s comes into scope

//     takes_ownership(s); // s's values moves into the function.. 
//                         // ... and so is longer valid here
//     let x = 5;          // x comes into scope
    
//     makes_copy(x);      // x world move into the function,
//                         // but i32 is Copy, so its okay to still
//                         // use x afterward
// }   // Here, x goes out of scope, then s. But because s's values was moved, nothing
//     // special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// }   // Here, some_string goes out of scope and 'drop' is called. The backing
//     // memory is freed

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }   // Here, some_integer goes out of scope. Nothing special happens

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {}, s2 = {}", s1, s2);
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{}, world!", s1);
// }

// fn main() {
//     {
//         // we call String::from, its implementation requests the memory it needs. 
//         // This is pretty much universal in programming languages.
//         // String needs to the allocator: when s goes out of scope. 
//         // When a variable goes out of scope, Rust calls a special function for us. 
//         // This function is called drop, and it’s where the author of String can put the code to return the memory. 
//         // Rust calls drop automatically at the closing curly bracket
//         let mut s = String::from("hello");

//         s.push_str(", world!");

//         println!("{}", s)
//     }
// }