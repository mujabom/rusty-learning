fn main() {
    println!("Hello, world!");
    //get number from user
    println!("Enter a number: ");
    let num = get_number();

    if num % 2 == 0 {
        println!("{} is even", num);
    } else {
        println!("{} is odd", num);
    }
}

fn get_number() -> i32 {
    loop {
        let mut input_num = String::new();
        std::io::stdin().read_line(&mut input_num).unwrap_or_else(|_| {
            panic!("Failed to read line")
        });
        if input_num.trim() == "exit" {
            println!("Exiting program");
            std::process::exit(0);
        }
        match input_num.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        }
    }
}