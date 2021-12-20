use std::io;

fn main() {
    println!("Welcome to Temperature Converter!\n");

    loop {
        println!("Enter 'ftc' to convert from Fahrenheit to Celsius.");
        println!("Enter 'ctf' to convert from Celsius to Fahrenheit.");
        println!("Enter 'q' to quit.");
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        match option.trim() {
            "ftc" => run_conversion(true),
            "ctf" => run_conversion(false),
            "q" => break,
            _ => continue,
        }
    }

    println!("Thanks for using Temperature Converter!");
}

fn run_conversion(fahrenheit_to_celsius: bool) {
    // Get temperature from user input
    let original_unit = if fahrenheit_to_celsius { "°F" } else { "°C" };
    let temperature = loop {
        println!("Please enter your temperature in {}", original_unit);
        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read temperature");
        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break temperature;
    };
    let converted_unit = if fahrenheit_to_celsius { "°C" } else { "°F" };
    let converted = if fahrenheit_to_celsius {
        (temperature - 32.0) * (5.0 / 9.0)
    } else {
        temperature * (5.0 / 9.0) + 32.0
    };

    println!(
        "\nResult: {}{} in {} is {}{}\n",
        temperature, original_unit, converted_unit, converted, converted_unit
    );
}
