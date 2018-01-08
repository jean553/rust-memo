struct MyStructure {
    value: u8,
}

impl Drop for MyStructure {

    fn drop(&mut self) {
        println!("the object goes out of the scope");
    }
}

fn main() {

    {
        let object = MyStructure { value: 10 };
        // print here
    }

    let object = MyStructure { value: 20 };
    // print here
}
