use std::cell::RefCell;

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
}
