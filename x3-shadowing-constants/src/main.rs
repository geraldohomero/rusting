fn main() {
    let (first_number, second_number) = (1, 2.343);
    let large_number = 1_000_000;

    // let over_flow_number: u8 = 256;

    println!("first_number: {}", first_number);
    println!("second_number: {}", second_number);
    println!("large_number: {}", large_number);

    let x = 255;
    println!("The value of the variable in octal is: {:o} and the hexadecimal is: {:x} and the binary is: {:b}", x, x, x);

    let n1 = 12;
    let n2 = 65.4321;
    let n3 = n1 as f32 + n2; // need to convert n1 to f32 to add with n2 becouse n2 is a float number and rust don't allow to add float with integer
    println!("n3: {}", n3);

    // shadowing
    // is when you declare a new variable with the same name as a previous variable and the new variable shadows the previous variable
    // the first variable is beeing shadowed by the second, which means that the second variable's value is what the program sees 

    let s = 5;
    let s = 5*5;

    println!("The value of s is: {}", s); // 25
}
