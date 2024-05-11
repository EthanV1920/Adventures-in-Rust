fn main() {
    // Declare the variable that I will need
    let mut number_one:String = String::new();
    let mut number_two:String = String::new();

    // Get the first user input with poor error management
    println!("Enter a number from 1-10: ");
    std::io::stdin().read_line(&mut number_one).unwrap();
    println!("This is what you entered: {number_one}");
 
    loop {
        println!("Enter a number from 1-10: ");
        /* std::io::stdin().read_line(&mut number_two).unwrap_or_else(|err| {
           println!("There was a problem parsing the input {err}");
           std::process::exit(1)
           }); */
        std::io::stdin() .read_line(&mut number_two).unwrap();

        // let a_number = Option::Some(10);
        // match a_number {
        //     Some(x) if x <= 5 => println!("0 to 5 num = {x}"),
        //     Some(x @ 6..=10) => println!("6 to 10 num = {x}"),
        //     None => panic!(),
        //     // all other numbers
        //     _ => panic!(),
        // }

        let num = Option::Some(number_two.trim().parse::<i32>());
        match num {
            Some(x @ 0..=10) => println!("You entered an i32 {}", x),
            // Err(..) => println!("You did not enter an i32"),
        };



        
    }
    println!("This is what you entered: {number_two}")
}
