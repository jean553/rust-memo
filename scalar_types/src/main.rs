fn main() {

    /* scalar types */

    let value: u8 = 255; // between 0 and 255
    let value: i8 = -128; // between -128 and 127
    println!("{}", value);

    let value: u16 = 65535; // between 0 and 65535
    let value: i16 = -32758; // between -32768 and 32757
    println!("{}", value);

    let value: u32 = 4294967295; // between 0 and 4294967295
    let value: i32 = -2147483648; // between -2147483648 and 2147483647
    println!("{}", value);

    /* (idem for u64 and i64) */

    /* on a 32 bits architecture */
    let value: usize = 4294967295; // between 0 and 2^32 
    let value: isize = -2147483648;

    /* (idem as u64 and i64 on a 64 bits architecture) */

    /* if the type is not specified, the default type is i32 */
    let mut value = -2147483648;
    value = -40;

    /* floating-point variables */
    let value: f32 = 5.0;
    let value: f64 = 10.0;

    /* unicode scalar value */
    let value: char = 'a';
    println!("{}", value);
}
