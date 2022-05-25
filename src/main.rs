use std::io::{ self, Write };

fn main() {
    print!("Input a number: ");
    io::stdout().flush().unwrap();
    
    let mut num: String = String::new();
    
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
        
    let num: u32 = num.trim().parse().expect("Input a number you dumbass! I'm sorry...");
    
    let fib_num: u32 = fib(num);

	print!("\n");
    println!("Fibonacci number is: {}", fib_num);
}

fn fib(num: u32) -> u32 {
    if num <= 1 {
        return num;
    }
    
    return fib(num - 1) + fib(num - 2)
}
