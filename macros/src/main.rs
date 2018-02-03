/* defining a macro (set of repeatedly used code),
   with one expression */
macro_rules! my_macro {
    ($p:expr) => {
        println!("{}", $p);
    }
}

/* defining a macro with a more specific pattern */
macro_rules! my_other_macro {
    (required => $p:expr) => {
        println!("{}", $p);
    }
}

/* we can define more than one pattern per macro */
macro_rules! multiple_patterns {
    (first) => (println!("first"));
    (second - $p:expr) => (println!("second"));
}

/* a macro that takes no parameter */
macro_rules! no_param {
    () => {
        println!("No parameter")
    }
}

/* a macro with repetitions */
macro_rules! list {
    ($($p:expr),*) => {{
        $(
            println!("{}", $p);
        )*
    }}
}

fn main() {

    /* simple macro call */
    my_macro!(10);
    my_macro!("hello");

    my_other_macro!(required => 100);

    /* simple macro with multiple valid patterns */
    multiple_patterns!(first); // first
    multiple_patterns!(second - 30); // second

    no_param!();

    list!(15, 20, 30);
}
