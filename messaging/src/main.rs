/* mpsc = multiple producer single consumer */
use std::sync::mpsc;

use std::thread;

fn main() {

    /* create one transmitter and one receiver for a common channel */
    let (
        transmitter,
        receiver,
    ) = mpsc::channel();

    let thread = thread::spawn(move || {
        let value = 10;

        /* value is moved to the transmitter */
        transmitter.send(value).unwrap();

        /* value cannot be used here */
        println!("{}", value);
    });

    let received = receiver.recv().unwrap();
    println!("{}", received);

    /* example of channel transmitter clone;
       send data through the two transmitters
       send to the same receiver */

    let (
        other_transmitter,
        other_receiver,
    ) = mpsc::channel();

    let other_transmitter_copy = other_transmitter.clone();

    let first_thread = thread::spawn(move || {
        for index in 0..10 {
            other_transmitter.send(index).unwrap();
        }
    });

    let second_thread = thread::spawn(move || {
        for index in 100..110 {
            other_transmitter_copy.send(index).unwrap();
        }
    });

    for received in other_receiver {
        println!("{}", received);
    }
}
