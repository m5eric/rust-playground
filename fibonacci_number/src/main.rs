use std::io;

fn main() {
    println!("Generate Fibonacci number with index:");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim().parse::<u64>();

        match input {
            Ok(input) => {
                println!(
                    "Calling with index {}, -> {}",
                    input,
                    fibonacci_num_by_idx(input)
                );
            }
            Err(_) => {
                println!("Please enter an positive integer");
            }
        }
    }
}

fn fibonacci_num_by_idx(number: u64) -> u64 {
    match number {
        0 => 0,
        1 => 1,
        _ => fibonacci_num_by_idx(number - 1) + fibonacci_num_by_idx(number - 2),
    }
}
