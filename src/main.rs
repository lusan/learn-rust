fn main()  {
    let my_string = String::from("hello world");

    // first_word works on slices of Strings whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals are string slices already
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

// fn main() {
//     let mut s = String::from("hello world");

//     let word = first_word(&s); // word will get the value 5

//     s.clear(); // This empties the String, making it equal to ""

//     // word still has the value 5 here, but theres no more string that
//     // we could meaningfully use the value 5 with. word is not totally invalid!
// }
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i , &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i , &item) in bytes.iter().enumerate() {
//         if item ==  b' ' {
//             return i;
//         }
//     }

//     s.len()
// }