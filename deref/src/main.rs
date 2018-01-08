use std::ops::{
    Deref,
    DerefMut,
};

struct CustomBox<T> {
    param: T,
}

impl<T> CustomBox<T> {

    pub fn new(param: T) -> CustomBox<T> {
        CustomBox {
            param: param, 
        }
    }
}

/* the function "deref" defines what happens when
   the type is de-referenced using *,
   it is implemented by the std::ops::Deref trait
   and must be implemented by the structure
   that needs it */
impl<T> Deref for CustomBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.param
    }
}

struct OtherCustomBox<T> {
    param: T,
}

impl<T> OtherCustomBox<T> {

    pub fn new(param: T) -> OtherCustomBox<T> {
        OtherCustomBox {
            param: param,
        }
    }
}

struct ThirdCustomBox<T> {
    param: T,
}

impl<T> ThirdCustomBox<T> {

    pub fn new(param: T) -> ThirdCustomBox<T> {
        ThirdCustomBox {
            param: param,
        }
    }
}

/* if a type implements DerefMut, it must implements Deref too */

impl<T> Deref for ThirdCustomBox<T> {
    type Target = T;

    /* this is the required signature of the Deref object deref */
    fn deref(&self) -> &T {
        &self.param
    }
}

impl<T> DerefMut for ThirdCustomBox<T> {

    /* this is the required signature of the DerefMut object deref_mut */
    fn deref_mut(&mut self) -> &mut T {
        &mut self.param
    }
}

fn print_digit(digit: &u8) {
    println!("{}", digit);
}

fn print_mut_digit(digit: &u8) {
    println!("{}", digit);
}

fn main() {

    /* CustomBox implements Deref
       and has its own definition of deref,
       so the following code compiles well
       and the variable can be dereferenced */
    let value = CustomBox::new(10);
    println!("{}", *value);

    /* OtherCustomBox does not implement Deref,
       so using the dereferencing operator
       on it does not work */
    let value = OtherCustomBox::new(20);
    //println!("{}", *value);
    
    /* deref coercion is a mechanism
       that calls "deref()" when the object
       is passed by reference */
    let digit: u8 = 20;
    let value = CustomBox::new(digit);
    print_digit(&value);

    /* deref coercion also works with mutable references,
       the trait to use is DerefMut and the function to
       define "deref()" */
    let digit: u8 = 30;
    let mut value = ThirdCustomBox::new(digit);
    print_mut_digit(&mut value);
}
