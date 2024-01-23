fn main() {
    println!("Hello World");
    println!("The value is {}", 10);
    //println!(10) will not work
    println!("");

    print!("this is going to be
        Printed on Multiple 
        lines");

    println!("\n\n This is going to be printed after one line");
    println!("\t This will have some empty space at the begining");
    println!("This is some text which will be overwritten \r this text will only appear on the screen");
    println!("\n\n the first slash n is going to be parte of the text");
    println!("This will print single quote \' and this double quote \"");
    println!("This will print only one back slash \\");
    println!("\n doing {2} from {1} years and i {0} it", "love", 1.5, "programming");

    println!("{language} is a system programming language whick is cool to {activity} in.", activity = "Code", language = "Rust");

    println!("The summation of 25 + 10 = {}", 25+10);

}

// comment

/*
Multiple lines
*/


