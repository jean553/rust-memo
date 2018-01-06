struct Structure
{
    value: i32,
}

fn first_function(reference: &Structure) {

    println!("{}", reference.value);
}

fn second_function(reference: &mut Structure) {

    println!("{}", reference.value);
}

fn third_function(value: Structure) {

    println!("{}", value.value);
}

fn main() {

    /* an object can have exactly one mutable reference */
    let mut value = String::from("some text");
    let reference = &mut value;
    // error: already one mutable reference: let other_reference = &mut value;
    println!("{}", reference);

    /* an object can have many immutable references */
    let mut value = String::from("some text");
    let reference = &value;
    let other_reference = &value;
    let and_another_reference = &value;
    println!("{}", reference);
    println!("{}", other_reference);
    println!("{}", and_another_reference);

    /* an object cannot have both mutable and immutable references */
    let mut value = String::from("some text");
    let reference = &mut value;
    //error: let other_reference = &value;
    
    let variable = String::from("one string");
    let mut reference = &variable;
    let other_variable = String::from("one other string");
    // error: "other_variable" goes out of the scope: reference = &other_variable;

    /* when a variable is borrowed by a reference,
       only the reference can access it until the reference goes out of the scope */
    let mut variable = Structure {
        value: 10,
    };
    let reference = &mut variable;
    reference.value = 20;
    // error: "variable" is currently borrowed by "reference": variable.value = 10;

    /* if a variable is borrowed into a limited scope,
       the variable can be accessed again as soon as the borrowing reference scope is left */
    let mut variable = Structure {
        value: 10,
    };
    {
        let reference = &mut variable;
        reference.value = 30;
    }
    println!("{}", variable.value); // 30
    variable.value = 20;
    println!("{}", variable.value); // 20

    /* when a variable is borrowed, the reference can be passed to functions,
       but not the borrowed variable directly */
    let variable = Structure {
        value: 10,
    };
    let reference = &variable;
    first_function(&variable);
    //error: "variable" is borrowed by "reference": third_function(variable);

    /* if a variable is borrowed by a mutable reference,
       this is not possible to pass a mutable reference of the variable to a function,
       BUT this is possible to pass the current mutable reference itself */
    let mut variable = Structure {
        value: 10,
    };
    let reference = &mut variable;
    //error: "variable" is already borrowed: second_function(&mut variable);
    second_function(reference); // works
    reference.value = 20;
    //error: "variable" is still borrowed: variable.value = 30;
    
    /* if a variable is borrowed, this is not possible to move the variable to a function */
    let variable = Structure {
        value: 10,
    };
    let reference = &variable;
    //error: "variable" is already borrowed: third_function(variable);
}
