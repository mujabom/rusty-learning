fn main() {
    println!("Hello, world!");
    let a = (5, String::from("hello"));
    //get number from user
    println!("Enter a number: {} ", a.1);
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
                println!("Invalid input. Please enter a number or type 'exit' to quit");
            }
        }
    }
}