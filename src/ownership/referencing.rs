fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a refence to a String
    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;
//     println!("{} and {}", r1, r2);
//     // variables r1 and r2 will not be used after this point

//     let r3 = &mut s;
//     println!("{}", r3);
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;
//     let r3 = &mut s;

//     println!("{}, {}, and {}", r1, r2 ,r3)
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2);
// }

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }


// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//     s.len()
// } // Here s goes out of scope. But because it does not have ownership of what
// // it refers tp, it is not dropped.
// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(s1);
//     println!("The length of '{}' is {}. ", s1, len);
// }
// fn calculate_length(s: String) -> (usize) {
//     let length = s.len();

//     length
// }