#![feature(const_fn)]

/* constants can be declared to be part of the global scope */
const GLOBAL_CONSTANT: u8 = 0;

/* const before fn means this function is evaluated at compile-time */
const fn get_constant() -> u8 {

    /* this function cannot contains let declarations */

    10
}

fn main() {

    /* this is a constant variable,
       the type is mandatory */
    const CONSTANT: u8 = 0;

    /* a constant can be built from another constant */
    const OTHER_CONSTANT: u8 = CONSTANT;

    /* a constant be be built from an operation on constants and raw values */
    const ANOTHER_CONSTANT: u8 = CONSTANT + OTHER_CONSTANT + 25;

    /* a constant cannot be built from a variable */
    let first_variable = 10;
    //error: const RESULT: u8 = first_variable;
    
    /* a constant cannot be set a second time */
    const VALUE: u8 = 20;
    //error: VALUE = 30;
    
    /* constant can be built from a function
       if this function result can be evaluated
       at compile time */
    const FROM_FUNCTION: u8 = get_constant();
}
