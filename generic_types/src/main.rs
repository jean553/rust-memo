/* generic data types can be used to define functions */
fn get_sum<T>(
    choice: bool,
    first: T,
    second: T,
) -> T
{
    if choice {
        first
    } else {
        second
    }
}

/* structure attributes can have generic types */
struct MyStructure<T, U> {
    first: T,
    second: U,
    third: bool,
}

/* implementations can use generic types */
impl<T, U> MyStructure<T, U> {

    pub fn get_first(&self) -> &T {
        &self.first
    }
}

struct MyOtherStructure<T, U> {
    first: T,
    second: U,
    third: bool,
}

/* implementation can be different according to passed types,
   the two functions signature is the same,
   but the structure attributes types will decide which function to use */
impl MyOtherStructure<u32, bool> {

    pub fn get_first(&self) -> &u32 {
        &self.first
    }
}
impl MyOtherStructure<bool, u32> {

    pub fn get_first(&self) -> &u32 {
        &self.second
    }
}

/* generic types can be passed to enumerations */
enum MyEnumeration<T> {
    FirstValue(u32),
    SecondValue(bool),
    ThirdValue(T),
    Nothing,
}

fn main() {

    /* passed parameters types define what type to use */
    let result = get_sum(
        true,
        10,
        5,
    );
    let float_result = get_sum(
        false,
        10.0,
        5.0,
    );

    /* used parameters when creating the structure defines what types to use */
    let object = MyStructure {
        first: 10,
        second: true,
        third: false,
    };
    let reference: &u32 = object.get_first();
    println!("{}", reference);

    let other_object = MyStructure {
        first: true,
        second: false,
        third: false,
    };
    println!("{}", other_object.first);

    /* generic types can be used into enumerations */
    let mut value: MyEnumeration<f64> = MyEnumeration::Nothing;
    value = MyEnumeration::ThirdValue(10.0);
    value = MyEnumeration::SecondValue(true);

    let other_value = MyEnumeration::ThirdValue(false);

    /* the chosen implementation depends on the passed attributes types */
    let object = MyOtherStructure {
        first: 10 as u32,
        second: false,
        third: false,
    };
    println!("{}", object.get_first());

    let other_object = MyOtherStructure {
        first: false,
        second: 25 as u32,
        third: false,
    };
    println!("{}", other_object.get_first());
}
