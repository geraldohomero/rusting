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

    let mut p = 5;
    let p = p * 2;
    // p = 60; // cannot assign twice to immutable variable

    //shadowing its type

    let q = 32;
    let q = "thirty two";
    println!("The value of q is: {}", q); // thirty two
    // this can be done multiple times

    // the scope of the shadowed variable is limited to the block in which it is declared
    let mut r = 65;
    {
        let r = 32;
        println!("The value of r inside the block is: {}", r); // 32
    }
    println!("The value of r outside the block is: {}", r); // 65
    // but if we remove the let keyword from the r inside the block, the value of r outside the block will be 32
    // because the r inside the block is a new variable that shadows the r outside the block
    let mut f = 60;
    {
        f = 33; // let removed
        println!("2 The value of f inside the block is: {}", f); // 33
    }
    println!("2 The value of f outside the block is: {}", f); // 33

    // Constants
    // are always immutable
    // Difference between constants and immutable variables
    //          1. You are not allowed to use mut with constants, they [constants] are always immutable
    //          2. You will declare a constant using the const keyword instead of the let keyword and the type of the value must be annotated
    //             The compiler will NOT infer and add the type of the value for you

    const MAX_POINTS: u32 = 100_000; // the type of the value must be annotated
                                     // the name of the constant must be in uppercase

}
