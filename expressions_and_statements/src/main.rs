fn get() -> u8 {
    //error: statement returns nothing (semicolon): 10 + 5;
    10 + 5 // works: expression returns a value
}

/* declaring a function is a statement */
fn main() {

    /* declaring a variable is a statement */
    let value = 10;

    /* declaring a value is a statement,
       so it does not return any value,
       so it is impossible to use
       a statement to set a value */
    //error: let value = (let other = 10);

    /* a statement can contain an expression */
    let value = {
        let other = 10;
        other - 5
    };

    /* expressions have no semicolon at the end */
    {
        let other = 10;
    }

    /* expressions that return nothing returns () (unit type) */
    let a = {
        let other = 10;
    }; // a type is () (unit type)
}
