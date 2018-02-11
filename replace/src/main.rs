use std::mem;

struct MyStructure {
    array: Vec<u8>,
}

impl MyStructure {

    /// Returns a reference of an object attribute
    pub fn get_array_reference(&mut self) -> &Vec<u8> {
        &self.array
    }

    /// Move out an object attribute, the attribute must be replaced
    pub fn move_out_array(&mut self) -> Vec<u8> {
        // error: the attributed is owned by the object, as solution would be a copy/clone: self.array

        /* returns the previous array and replace it with a new one */
        mem::replace(
            &mut self.array,
            vec![100, 200],
        )
    }

    /// Move out an object attribute, the attribute is replaced by another one
    pub fn move_out_and_replace(
        &mut self,
        new_array: Vec<u8>,
    ) -> Vec<u8> {
        
        mem::replace(
            &mut self.array,
            new_array,
        )
    }
}

fn main() {

    /* getting a reference to an attribute of an object is totally possible */

    let mut obj = MyStructure {
        array: vec![1, 2, 3],
    };
    let vector: &Vec<u8> = obj.get_array_reference();

    /* moving out an object attribute is of course impossible,
       but this is possible to "replace" it by another one */

    let mut obj = MyStructure {
        array: vec![10, 20],
    };
    let vector: Vec<u8> = obj.move_out_array();

    println!("{}", vector[0]); // 10
    println!("{}", obj.array[0]); // 100

    /* moving out an object attribute can be done by replacing it with a custom one */

    let mut obj = MyStructure {
        array: vec![15, 25],
    };
    let new_array: Vec<u8> = vec![150, 250];
    let vector: Vec<u8> = obj.move_out_and_replace(new_array);
}
