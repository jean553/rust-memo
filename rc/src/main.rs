use std::rc::Rc;

fn main() {

    let first = Rc::new(10);
    println!("{}", Rc::strong_count(&first)); // 1

    let second = first.clone();
    println!("{}", Rc::strong_count(&first)); // 2

    let third = second.clone();
    println!("{}", Rc::strong_count(&third)); // 3

    println!("{}", *third); // 10
}
