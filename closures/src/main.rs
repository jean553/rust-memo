fn main() {

    /* the current context can be consumed by the closure,
       "param" is borrowed as immutable (the closure implements "Fn" trait) */
    let param = 10;
    let closure = |p| {
        p == param
    };
    println!("{}", param); // "param" is still usuable
    println!("{}", closure(10));

    /* the closure modifies one variable of the current context,
       here "param" is borrowed as mutable,
       the closture implements the "FnMut" trait;
       "param" is not accessible anymore outside of the closure */
    let mut param = 10;
    let mut closure = |p| {
        param += 5;
        p == param
    };
    println!("{}", closure(10));

    /* one solution to the problem just before is to wrap
       the closure into a specific context */
    let mut param = 10;
    {
        let mut closure = |p| {
            param += 5;
            p == param
        };
        println!("{}", closure(5));
    }
    println!("{}", param);

    /* here "my_string" is moved, so it is not accessible
       anymore outside of the closure;
       the closure implements the trait "FnOnce",
       note that without "move" keyword, the string
       would have been copied (String implements Clone) */
    let mut my_string = String::from("my string");
    let mut closure = move |p| {
        println!("{}", my_string);
    };
    // error: "my_string" has been moved: println!("{}", my_string);
    closure(10);
    // error: "my_string" has been moved: println!("{}", my_string);
}
