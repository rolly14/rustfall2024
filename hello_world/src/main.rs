const FAHRENEHIT_TEMPERATURE: u32 = 32;

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
   
   (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64
{
   (fahrenheit - 32.0) * 5.0/9.0
}
//divisiable
fn is_even(n: i32) -> bool
{
   n % 2 == 0
}
//guess
use std::io;

fn check_guess(guess: i32, secret: i32) -> i32 
{
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    println!("
    
    Assignment 1: Temperature Converter
    
    ");
   let mut fahrenheit_temp = 75.0; 

   let celsius_temperature = fahrenheit_to_celsius(fahrenheit_temp);

   println!("Fahrenheit to Celsius: {} ", celsius_temperature);
   //loop 
   let mut fahren_temp = 94.0;
   let cel_temp = fahrenheit_to_celsius(fahren_temp);

   loop {
      fahren_temp+=1.0;
       println!("The loop of Fahrenheit convert to Celsius: {}",fahrenheit_to_celsius(fahren_temp));
       if fahren_temp>=99.0{
           break;
       }
   }
   println!("
   
   Assignment 2: Number Analyzer
   
   ");
   //divaislbe
   let arr = [3,2,54,6,13,76,45,89,65,49];

   for &num in arr.iter()
   {
      //print even or odd
      if is_even(num)
      {
         println!("{} is even ", num);
      }
      else
      {
         println!("{} is odd ", num);
      }
//divisible by 3
      if num % 3 ==0
      {
         println!("Fizz");
      }
      //divisible by 5
      else if num % 5 ==0
      {
         println!("Buzz");
      }
      // divisible by 5 and 3
      else if num % 3 ==0 && num % 5 ==0
      {
         println!("FizzBuzz");
      }
   }
   //sum of all number
   let mut index =0;
   let mut sum =0;
   while index < arr.len()
   {
      sum += arr[index];
      index+=1;
      
   }
   println!("the sum of all numbers is {}", sum);
   //largest number
   let mut largest = arr[0];

   for &number in arr.iter() {
       if number > largest {
           largest = number;
       }
   }
   println!("The largest number in the array is {}", largest);

   println!("
   
   Assignment 3: Guessing Game
   
   ");
   //guess
   let secret_number = 134; 
    let mut guess_count = 0;

    loop {
        
        let mut input = String::new();
        println!("Please enter your guess:");

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };
        guess_count += 1;
        
        match check_guess(guess, secret_number) {
            0 => {
                println!("You did it. You guessed the correct number.");
                break; // Exit the loop if the guess is correct
            }
            1 => println!("Too high! Try again."),
            -1 => println!("Too low! Try again."),
            _ => unreachable!(), 
        }
    }
    println!("It took you {} guesses.", guess_count);
}

