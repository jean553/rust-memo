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
}
