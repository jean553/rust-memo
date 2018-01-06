fn main() {

    /* the first value is shadowed by the creation 
       of a second variable with the same name */
    let first = 0;
    println!("{}", first); // 0
    let first = 10;
    println!("{}", first); // 10

    /* shadowing can be performed with variables
       of totally different types */
    let second: u8 = 10;
    let second: u64 = 20;
    println!("{}", second); // 20

    /* one benefit is to stop objects mutability */
    let mut third = 10;
    third = 20;
    let third = third;
    // error: third = 40;
    
    /* one benefit is to start objects mutability */
    let fourth = 10;
    let mut fourth = fourth;
    fourth = 30;
}
