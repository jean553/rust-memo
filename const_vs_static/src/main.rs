/* constant stored into the code directly for future usage */
const STORED_IN_THE_CODE: i32 = 10;

/* variable that exists in memory and that can be accessed from anywhere */
static mut STORED_IN_MEMORY: i32 = 20;

static IMMUTABLE_VALUE: i32 = 50;

fn print_mutable_value() {

    /* static globals can be accessed from anywhere */
    unsafe {
        println!("{}", STORED_IN_MEMORY);
    }

    /* immutable static variables does not need to be called into unsafe block */
    println!("{}", IMMUTABLE_VALUE);
}

fn main() {

    /* "static" and "static mut" variables that exists once
       for the entire program execution can be declared
       outside or within a function */
    static mut MUTABLE_VALUE: i32 = 30;
    static VALUE: i32 = 20;

    /* as many threads can access the variable at any time,
       the modification of a mutable static variable
       has to be performed into an unsafe block */
    unsafe {
        STORED_IN_MEMORY = 30;
    }

    print_mutable_value();
}
