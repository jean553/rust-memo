use std::sync::{
    Mutex,
    Arc,
};
use std::{
    thread,
    time,
};

fn main() {

    /* simple mutex explanation */

    let mut value = 10;
    /* accessing "value" mutex cannot be done simultaneously */
    let mutex = Mutex::new(value);
    /* concurrent lock of the mutex is forbidden from here */
    let first_lock = mutex.lock().unwrap();
    //error: this line would wait for ever, as "first_lock" is never terminated; let other_value = mutex.lock().unwrap();
    println!("{}", first_lock);

    /* multiple locks of the same mutex */

    let mut value = 20;
    let mutex = Mutex::new(value);
    {
        let first_lock = mutex.lock().unwrap();

        // "first_lock" terminates here, the value is released for other threads
    }
    let second_lock = mutex.lock().unwrap();
    println!("{}", second_lock);

    /* pass a mutex to a thread using Arc<T> */

    let mut value = 30;
    let mutex = Mutex::new(value);
    let mutex_arc = Arc::new(mutex);

    let mutex_arc_clone = mutex_arc.clone();
    let thread = thread::spawn(move || {
        let value = mutex_arc_clone.lock().unwrap();
        thread::sleep(time::Duration::from_millis(2000));
        println!("{}", value);
    });

    {
        let value = mutex_arc.lock().unwrap();
        println!("First thread value {}", value);
    }

    thread.join();
}
