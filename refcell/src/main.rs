use std::cell::{
    RefCell,
    RefMut,
};
use std::rc::Rc;

struct StructureWithoutRefCell {
    value: u8,
}

impl StructureWithoutRefCell {

    /// Constant method, modify the attributes is forbidden
    pub fn try_update(&self) {
        // forbidden: self.value = 20;
    }
}

struct StructureWithRefCell {
    value: RefCell<u8>,
}

impl StructureWithRefCell {

    /// Constant method, modify the attributes is forbidden
    pub fn try_update(&self) {
        *self.value.borrow_mut() = 20;
    }
}

fn main() {

    /* not having a RefCell attribute does not allow modification (inherited mutability) */
    let mut immutable_structure = StructureWithoutRefCell {
        value: 10,
    };
    immutable_structure.try_update();

    /* having RefCell attribute allows modification (interior mutability) */
    let mut mutable_structure = StructureWithRefCell {
        value: RefCell::new(10),
    };
    mutable_structure.try_update();
    println!("{}", mutable_structure.value.borrow()); // 20

    /* with normal references, the multiple immutable references check is done at compile time */

    let mut value: u8 = 20;
    let first_immutable_reference = &mut value;
    // compilation fails: let second_immutable_reference = &mut value;
    
    /* with RefCell<T>, the multiple immutable references check is done at execution time */

    let mut value: RefCell<u8> = RefCell::new(10);

    let mut first_immutable_reference: std::cell::RefMut<u8> = value.borrow_mut();
    // execution fails: let mut second_immutable_reference: std::cell::RefMut<u8> = value.borrow_mut();

    /* using RefCell<T> with Rc<T> in order to modify the wrapped content */

    let mut value = Rc::new(10);
    //compilation error: *value = 20; (cannot borrow as mutable)

    let mut value = Rc::new(RefCell::new(10));
    {
        let mut reference: RefMut<u8> = value.borrow_mut();
        *reference = 20;
    }
    println!("{}", *value.borrow());
}
