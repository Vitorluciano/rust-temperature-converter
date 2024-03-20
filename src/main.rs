use std::io;

fn main() {
    println!("Temperature converter");

    display_options();
    let mut option: char = read_option();
    while option != 'q' {

        if option == 'f' || option == 'c' {
            let temperature: f64 = read_temperature();
            let (conversion_value, conversion_unit): (f64, char) = convert_temperature(option, temperature);
            println!("{temperature} °{option} = {conversion_value} °{conversion_unit}");

        } else {
            println!("Invalid option");
        }
        
        display_options();
        option = read_option();
    }
}

fn convert_temperature(source_unit: char, value: f64) -> (f64, char) {
    if source_unit == 'f' {
        return ((value - 32.0) * (5.0 / 9.0), 'c')
    } 
    (1.8 * value + 32.0, 'f')
}

fn display_options() {
    println!("[f] Convert from °F to °C");
    println!("[c] Convert from °C to °F");
    println!("[q] Quit the application");
}

fn read_option() -> char {
    loop {
        println!("Please enter your choice:");

        let mut input = String::new();
    
        // read option as string
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        // parse option as char
        match input.trim().parse() {
            Ok(option) => return option,
            Err(_) => {
                println!("Invalid input. Input must be a char.");
                continue;
            }
        };
    }
}

fn read_temperature() -> f64 {
    loop {
        println!("Please enter a valid temperature: ");

        let mut input = String::new();

        //read temperature as string
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // parse option as f64
        match input.trim().parse() {
            Ok(temperature) => return temperature,
            Err(_) => {
                println!("Invalid input. Input must be a floating-point number.");
                continue;
            }
        };
    }
}
