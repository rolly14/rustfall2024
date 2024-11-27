use std::thread;
use std::time::Duration;

fn main() {
   //use mut vector to store threads
   let mut handles = vec![];
    //let handle = thread::spawn(||
    for i in 1..4{
        let handle = thread::spawn(move ||{
        println!("Thread {}",i);
        thread::sleep(Duration::from_millis(1));
        });
        handles.push(handle);
    }
   for handle in handles{
    handle.join().unwrap();
   }
    //handle.join().unwrap();
    println!("All threads completed");
}
