use std::{thread, time::Duration};
//task 2 
fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        // Your implementation here
        tracker +=1;
        println!("tracker increment: {}", tracker);
    };

    update();
    update();
}
 //task 3
fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    // Your implementation here
    vec.into_iter().map(f).collect()
}
 //task 4
struct ComputeCache<T>
where
    T: Fn() -> String,
{
    // Add fields here
    computation: T,
    cached_result: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        // Your implementation here
        ComputeCache{
            computation,
            cached_result: None,
        }
    }

    fn get_result(&mut self) -> String {
        // Your implementation here
        match &self.cached_result{
            Some(v) => {
                println!("Retrieved from cache instantly!");
                v.clone()
            }
            None => {
                println!("Computing(this will take 2 seconds)...");
                thread::sleep(Duration::from_secs(2));
                let v = (self.computation)();
                self.cached_result = Some(v.clone());
                v
            }
        }
    }
}
fn main() {
    println!("Task 1: Basic Closure");
    let operation = |a: i32, b: i32| {
        // Your implementation here
        a * b
    };

    println!("Result: {}", operation(10, 5));

    println!("Task 2: Environment Capture");
    track_changes();
     //task 3
    let numbers = vec![1, 2, 3];

    let doubled = process_vector(numbers.clone(), |x| {
        // Implement: multiply each number by 2
        x*2
    });

    let replaced = process_vector(numbers, |x| {
        // Implement: if number > 2, replace with 0, else keep number
        if x > 2
        {
            0
        }
        else
        {
            x
        }
    });
    println!("Task 3: Vector Transformation");
    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);

    println!("Task 4: Lazy Computation");
    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });
    
    println!("First call:");
    println!("Result: {}", cache.get_result());
    
    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}