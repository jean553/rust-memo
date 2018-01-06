struct Structure
{
    attribute: u8,
}

fn main() {

    /* this variable is immutable */
    let primitive_type_variable = 0;

    /* this object is immutable,
       we cannot modify its attributes */
    let object_variable = Structure {
        attribute: 0,
    };

    /* this array is immutable,
       we cannot modify its values */
    let array = [1, 2, 3, 4, 5];

    /* this variable is mutable */
    let mut primitive_type_mutable_variable = 0;
    primitive_type_mutable_variable = 10;

    /* this object is mutable */
    let mut mutable_object_variable = Structure {
        attribute: 0,
    };
    mutable_object_variable.attribute = 10;

    /* this array is mutable */
    let mut mutable_array = [1, 2, 3, 4, 5];
    mutable_array[0] = 10;
}
