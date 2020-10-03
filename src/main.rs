use std::io::stdin;
fn main() {
    loop {
        println!("Select conversion type:");
        println!("(1) Celsius to Fahrenheit\n(2) Fahrenheit to Celsius");

        let mut conversion_type = String::new();
        stdin()
            .read_line(&mut conversion_type)
            .expect("Failed to read line");
        let conversion_type: u8 = match conversion_type.trim().parse() {
            Ok(num) => match num {
                1 => {
                    println!("Selected (1)");
                    1
                }
                2 => {
                    println!("Selected (2)");
                    2
                }
                _ => {
                    println!("Enter 1 or 2");
                    continue;
                }
            },
            Err(_) => {
                println!("Enter a valid input");
                continue;
            }
        };

        let mut temp = String::new();
        let temp_converted: f32;
        println!("Enter temperature:");
        stdin().read_line(&mut temp).expect("Failed to read line");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a NUMBER");
                continue;
            }
        };

        match conversion_type {
            1 => {
                temp_converted = (temp * 1.800) + 32.00;
                println!("Temperature in fahrenheit: {}°F", temp_converted);
            }
            2 => {
                temp_converted = (temp - 32.00) / 1.8;
                println!("Temperature in Censius: {}°C", temp_converted);
            }
            _ => {
                continue;
            }
        }

        let mut choice = String::new();
        println!("Continue? y/n");
        stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim().to_ascii_lowercase().as_ref() {
            "y" => continue,
            "n" => break,
            _ => println!("Enter y or n"),
        };
    }
}
