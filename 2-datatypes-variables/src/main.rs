fn main() {
    let mut x: i32 = 15;
    let y: f32 = 15.5;
    println!("The value of x = {} and the y = {}", x, y);

    // cannot change the value of a variable in rust, it is immutable, but you can especify it as mutable with the keyword mut

    x = 20;

    println!("The value of x now is = {}", x);

    // Rules for naming variables
    // Only can start with a letter or underscore
    // Can contain numbers but not at the beginning
    // Can use uppercase and lowercase
    // Can use underscore to separate words
    // Cannot use special characters
    // Case sensitive

    let b = 5;
    let c = 10 * 10;
    println!("The value of b is: {} and the value of c is: {}", b, c);

    // Data types
    //      Scalar types
    //          Integers
    //              Signed
    //                  i8, i16, i32, i64
    //              Unsigned
    //                  u8, u16, u32, u64

    println!("The max value of i8 is: {}", std::i8::MAX);
    println!("The max value of i16 is: {}", std::i16::MAX);
    println!("The max value of i32 is: {}", std::i32::MAX);
    println!("The max value of i64 is: {}", std::i64::MAX);

    let j: i8 = -120;

    println!("The max value of u8 is: {}", std::u8::MAX);
    println!("The max value of u16 is: {}", std::u16::MAX);
    println!("The max value of u32 is: {}", std::u32::MAX);

    let k: u8 = 120;

    //          Floating point
    //              f32, f64

    let g: f32 = 3.54;

    println!("The max value of f32 is: {}", std::f32::MAX);

    //          Boolean
    //              bool

    let status: bool = false;
    println!(
        "The value of some of our variables are: {:?}",
        (j, k, g, status)
    );
    // :? is used to display complex types of data that are not
    // simple scalar values. Not needed when using separeted

    let not_equal: bool = 10 != 10; // 10 is different than 10? false
    println!("The value of not_equal is: {}", not_equal);

    //          Character
    //              char

    let c1: char = 'a';
    let c2: char = '😻';
    println!("The value of c1 is: {}", c1);
    println!("The value of c2 is: {}", c2);
    
}
