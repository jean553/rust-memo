/* valid recursive structure declaration */
struct RecursiveStructure {
    next: Box<RecursiveStructure>,
    value: u8,
}

/* a current way to handle recursive type
   with an enumeration and different types as options */
enum RecursiveList {
    Next(
        u8,
        Box<RecursiveList>,
    ),
    End,
}

struct MyStructure {
    first: u32,
    second: bool,
}

#[derive(Clone)]
struct MyOtherStructure {
    first: u8,
    second: u8,
}

fn main() {

    /* 10 is allocated on the heap;
       there is no need to use the dereference
       pointer (*) to get the pointed data */
    let pointer = Box::new(10);
    println!("Value: {}", pointer);

    /* the structure is allocated on the stack */
    let object = MyStructure {
        first: 10,
        second: true,
    };
    println!("{}", object.first);
    println!("{}", object.second);

    /* the structure is allocated on the heap */
    let object = Box::new(
        MyStructure {
            first: 10,
            second: false,
        }
    );
    println!("{}", object.first);
    println!("{}", object.second);

    /* known way to create recursive list */
    let list = RecursiveList::Next(
        10,
        Box::new(
            RecursiveList::Next(
                20,
                Box::new(RecursiveList::End)
            )
        )
    );

    /* a Box<T> is moved by default */
    let first = Box::new(
        MyStructure {
            first: 100,
            second: false,
        }
    );
    let second = first;
    println!("{}", second.first);
    //println!("{}", first.first);

    /* a Box<T> can be copied only
       if the boxed structure implements Clone */
    let first = Box::new(
        MyOtherStructure {
            first: 10,
            second: 20,
        }
    );
    let second = first.clone();
    println!("{}", first.first);
    println!("{}", second.first);
}
