fn main() {

    /* tuple with implicit types */
    let tuple = (10, 10.5, 'a');
    println!("{}", tuple.0);
    println!("{}", tuple.1);
    println!("{}", tuple.2);

    /* tuple with explicit types */
    let tuple: (u8, f32, char) = (10, 10.5, 'a');
    println!("{}", tuple.0);
    println!("{}", tuple.1);
    println!("{}", tuple.2);

    /* create new variable from tuple */
    let a = tuple.0;
    let b = tuple.1;
    let c = tuple.2;
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);

    /* tuple is affected my mutability */
    let tuple = (10, 20);
    //error: tuple.0 = 5;

    let mut tuple = (10, 20);
    tuple.0 = 50;
    println!("{}", tuple.0);

    /* declare a chunk of variables on the stack and access them */
    let array = [1, 2, 3, 4];
    println!("{}", array[0]);
    println!("{}", array[1]);

    /* arrays are affected by mutability */
    let array = [1, 2];
    //error: array[0] = 10;

    let mut array = [1, 2];
    array[0] = 10;
    println!("{}", array[0]);

    /* explicit array types (contains f32 and size is 3) */
    let array: [f32; 3] = [1.0, 2.0, 3.0];
    println!("{}", array[0]);
}
