use std::io;

fn main (){
    println!("Choose one of the options: \n 1) Celsius to Fahrenheit \n 2) Fahrenheit to Celsius");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim(){
            "1" => {
                let temperature =
                    prompt_and_read_temperature("Enter temperature in Celsius:");
                println!("{temperature:.2} C = {:.2} F", convert_celsius_to_fahrenheit(temperature));
                break;
            }
            "2" => {
                let temperature =
                    prompt_and_read_temperature("Enter temperature in Fahrenheit:");
                println!("{temperature:.2} F = {:.2} C", convert_fahrenheit_to_celsius(temperature));
                break;
            }
            _ => println!("Not one of the options"),
        }
    }
}

fn prompt_and_read_temperature(prompt :&str) -> f32{
    println!("{}", prompt);
    loop{
        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature).expect("Failed to read input");

        match temperature.trim().parse(){
            Ok(value) => return value,
            Err(_) => println!("Not a valid number"),
        }
    }
}

fn convert_celsius_to_fahrenheit (celsius: f32) -> f32{
    (celsius * 1.8) + 32.0
}

fn convert_fahrenheit_to_celsius (fahrenheit: f32) -> f32{
    (fahrenheit - 32.0) / 1.8
}