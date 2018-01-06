fn first_function(string: String) {

    println!("first_function: {}", string);
}

fn second_function(value: i32) {

    println!("second_function: {}", value);
}

fn third_function() -> String {

    String::from("my string")
}

fn main() {

    /* "value" is the owner of the value 5 */
    let value = 5;

    /* "text" is the owner of the string,
       allocated on the heap;
       this object is automatically
       destroyed at the end of the scope */
    {
        let text = String::from("my text");

        // leaving scope: text is invalid from here
    }

    /* we move one string allocated on the heap;
       "text" is not callable anymore once its
       content has been moved into "other_text" */
    let text = String::from("my content");
    let other_text = text;
    //error: println!("{}", text);
    println!("{}", other_text);

    /* we create a copy "other_text" of the "text" variable;
       both of the heap and stack memory is copied
       and a brand new variable is created */
    let text = String::from("another_content");
    {
        let other_text = text.clone();
    }
    println!("{}", text); // "text" is still accessible

    /* i32 implements the Copy trait, so it is copied
       and not moved; it only lives on the stack,
       so there is no need for such kind of safety */
    let x = 5;
    let y = x;
    println!("{}", x);
    println!("{}", y);

    /* movement also occurs when a heap object
       is passed to a function; "string" is not
       callable anymore within this scope
       after calling "first_function" */
    let string = String::from("my content");
    first_function(string);
    //error: println!("{}", string);
    
    /* copy also occurs when a stack object
       is passed to a function */
    let value = 10;
    second_function(value);
    println!("{}", value);

    /* move occured from within the function,
       and moved its returned value */
    let value = third_function();
}
