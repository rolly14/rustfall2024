use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Debug)]
struct Car {
    name:String,
    car: String,
    color: String,
    year: u32,
}

#[derive(Debug)]
struct Config
{
    car: Car,
}

fn create_and_write_to_file() {
    //let mut file = File::create("user_info.txt").unwrap();
    let mut buffer = String::new();

    print!("What's is name?");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();

    print!("What's car do you have?");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let car_type = buffer.trim().to_string();
    buffer.clear();

    print!("What color is your car?");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let color = buffer.trim().to_string();
    buffer.clear();

    print!("What year is your car?");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year: u32 = buffer.trim().parse().unwrap();

    let car_info = Car { name, car:car_type, color, year };
    let your_car_info = Config {car: car_info};

    let mut file = File::create("user_info.txt").unwrap();
    writeln!(file, "{:?}", your_car_info).unwrap();

    println!("Hi {}, you're car is {} and the color is {} and it's year is {}!", your_car_info.car.name, your_car_info.car.car, your_car_info.car.color, your_car_info.car.year);
}

fn read_and_print_file() 
{
    let path = "user_info.txt";
    let mut contents = String::new();

    if Path::new(path).exists()
    {
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut contents).unwrap();
        println!("Your car information is:\n{}", contents);
    };
    
}

fn main() 
{
    create_and_write_to_file();
    read_and_print_file();
}