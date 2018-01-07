/* define a trait with functions declarations,
   "get_first" has to be defined by implementations,
   "info" might be redefined, but has a default definition */
trait MyTrait {

    fn get_first(&self) -> &u32;

    fn info(&self) {
        println!("Information message");
    }
}

/* specific traits can be set using "derive";
   it creates a basic definition of the feature
   for the given structure */
#[derive(Clone)]
struct MyStructure {
    first: u32,
}

/* implement MyTrait for MyStructure */
impl MyTrait for MyStructure {

    fn get_first(&self) -> &u32 {
        &self.first
    }
}

/* the first parameter of this function 
   is an object that implements MyTrait
   and Clone */
fn function<T>(value: &T) -> &u32 
    where T: MyTrait + Clone
{
    &value.get_first()
}

struct OtherStructure<T, U> {
    first: T,
    second: U,
}

impl<T: MyTrait, U: Clone> OtherStructure<T, U> {

    pub fn get_first_value(&self) -> &T {
        &self.first
    }

    pub fn get_second_value(&self) -> &U {
        &self.second
    }
}

fn main() {

    let object = MyStructure {
        first: 10,
    };
    println!("{}", function(&object));
    object.info();

    let other_object = OtherStructure {
        first: object,
        second: 15,
    };
    println!("{}", other_object.get_first_value().get_first());
    println!("{}", other_object.get_second_value());

    let third_object = OtherStructure {
        first: 15,
        second: true,
    };
    // error: types do not match for this implementation: third_object.get_first_value();
}
