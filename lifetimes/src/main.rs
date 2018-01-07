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
}
