use std::thread;
//use std::time::Duration;
fn share_counter_data_between_thread() { 
    use std::sync::{Arc, Mutex}; // Arc and Mutex

    println!("Intro to Concurrency");
    let counter =  Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _i in 0..5{
        
    // //let thread = {
         let counter_clone = Arc::clone(&counter);
         let handle = thread::spawn(move ||{
            
            for _j in 0..10 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
                //println!("Thread {} - Incremented counter to {}", i, *num);
            }
    //         while *steps.lock().unwrap() > 0 {
    //             std::thread::sleep(std::time::Duration::from_secs(1));
    //             println!("Thread step {}", steps.lock().unwrap());
    //             *steps.lock().unwrap() -= 1; // mutating the value inside Mutex
    //         }
    //         "Goodbye!" // thread could return values
         });
         handles.push(handle);
    // // };
    }
    for handle in handles{
        handle.join().unwrap();
    }

    // println!("Spawned a thread!");

    // std::thread::sleep(std::time::Duration::from_secs(3));
    // println!("Main thread slept for 3 seconds");

    let result = *counter.lock().unwrap(); 
    println!("Counter value is {:?}", result);
}
fn main()
{
    share_counter_data_between_thread()
}