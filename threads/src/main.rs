use std::thread;

fn main() {

    let mut thread;

    {
        let value = 10;

        /* this thread and the loop below are executed simultaneously;
           the ownership of "value" is passed to the thread */
        thread = thread::spawn(move || {
            for i in 0..100 {
                println!("{}", value);
            }
        });

        /* "value" is not accessible from here anymore, so there is no
           risk of wrong modification of the variable that might
           affect the execution of the running thread "thread" */
    }

    for i in 0..10 {
        println!("x");
    }

    /* wait for the thread "thread" to be finished before going on */
    thread.join();

    println!("the two threads are finished");
}
