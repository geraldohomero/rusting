//
// Strings
//   -&str (string slice) & means that it is basically a reference
//   -String (heap allocated string)
// Are immutable by default
// Fix length
// Can not grow in size or change its value

fn main() {
    let one_string = "Fixed length string"; // &str its the default type for strings

    let mut growable_string = String::from("This string will grow"); // String is a growable string
    println!("The string is: \"{}\" ", growable_string);

    growable_string.push('s'); // add a character to the end of the string
    println!("The string is: \"{}\" ", growable_string);

    growable_string.pop(); // remove the last character
    println!("The string is: \"{}\" ", growable_string);

    growable_string.push_str("Which i will use"); 
    println!("Basics functions on Strings,
    is_empty(): {},
    length(): {},
    Bytes: {},
    Contains 'use': {}",
    growable_string.is_empty(),
    growable_string.len(),
    growable_string.capacity(),
    growable_string.contains("use"));

    growable_string.push_str("        ");
    // remove the white spaces from the end of the string
    let str_len: usize = growable_string.trim().len(); // multiple functions can be chained
    println!("The string is: \"{}\" ", str_len);
}