use std::io;
fn main() {
    fn fibbonacci(n: i32) {
        let mut first: i32 = 0;
        let mut second: i32 = 1;

        for _ in 0..n {
            println!("{}", first);
            let temp: i32 = first;
            first = second + temp;
            second = temp;
        }
    }

    fibbonacci(10);

    fn temperature_converter() {
        println!("Please choose if you want to change the temperature from celcius or farenheit (C or F)");
        let mut choice: String = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        println!("Choose the temperature");
        let mut temperature: String = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                return;
            }
        };

        if choice.trim() == "C" {
            let farenheit: f64 = (temperature * 9.0/5.0) + 32.0;
            println!("{} C = {} F", temperature, farenheit);
        } else if choice.trim() == "F" {
            let celsius: f64 = (temperature - 32.0) * 5.0/9.0;
            println!("{} F = {} C", temperature, celsius);
        } else {
            println!("Invalid choice");
        }
    }

    temperature_converter();
}
