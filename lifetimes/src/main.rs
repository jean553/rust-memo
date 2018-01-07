fn get_longest_string(
    first: &str,
) -> &str
{
    first
}

fn get_highest<'a, 'b>(
    first: &'a i32,
    second: &'a i32,
) -> &'a i32
{
    if *first == 0 {
        first
    } else {
        second
    }
}

fn get_one_reference<'a>(
    first: &'a i32,
    second: &i32,
) -> &'a i32
{
    first
}

struct MyStructure<'a> {
    reference: &'a i32,
    other_reference: &'a i32,
}

impl<'a> MyStructure<'a> {

    pub fn get_reference(&self) -> &'a i32 {

        if (true) {
            self.other_reference
        } else {
            self.reference
        }
    }
}

fn main() {

    /* error occurs if the reference lifetime is higher than its value */
    //{
    //    let reference;
    //    {
    //        let value = 10;
    //        // error: value does not live long enough: reference = &value;
    //    }
    //}

    /* "first" and "second" have a different lifetime,
       so the returned lifetime cannot be equal to both at the same time,
       this code may fail if a reference to "second" is returned
       as "second" lifetime is shorter than "reference" lifetime,
       the compilation fails */
    let first = 10;
    let mut reference;
    reference = &first;
    {
        let second = 20;
        // error: reference = get_highest(
        //     &first,
        //     &second,
        // );
    }
    // error if we call "reference" from here
    // and this is a reference to "second"
    // that does not exist anymore

    /* "first" and "second" have the same lifetime,
       this lifetime is the same one as the returned reference,
       the code above compiles */
    let first = 10;
    let second = 20;
    let first_reference = &first;
    let second_reference = &second;
    let reference = get_highest(
        first_reference,
        second_reference,
    );

    /* get_one_reference always return a reference of the same lifetime
       than "first", so for sure "reference" will always have the appropriate lifetime */
    let first = 10;
    let mut reference;
    {
        let second = 20;
        reference = get_one_reference(
            &first,
            &second,
        );
    }

    /* if structure attributes references have the same lifetime,
       then trying to set references with different lifetimes
       results into an error */
    let value = 10;
    let other_value = 20;
    let mut object = MyStructure {
        reference: &value,
        other_reference: &other_value,
    };
    {
        let other_value = 20;
        // error: "other_value" does not live long enough:  object.reference = &other_value;
    }

    /* 'static lifetime cannot be used on a variable that does not live the entire program
       execution */
    let value = 10;
    // error: value does not live for the entire program execution: let reference: &'static i32 = &value;
}
