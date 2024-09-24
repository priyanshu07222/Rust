// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use std::List::{Cons, Nil};

structs MyBox<T>(T);



use std::ops::Deref;

impl <T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target{
        &self.0
    }
}

impl<T> MyBox<T>{
    fn new(x: T) ->  MyBox<T>{
        MyBox(x)
    }
}

fn main() {
    // let b = Box::new(5);
    // println!("Hello, world!, {b}");

    // let list = Cons(1, Box::new(Cons(2, Box::new(3, Box::new(Nil)))));


    let x = 6;
    let y = MyBox::new(x);

    assert_eq!(6, x);
    assert_eq!(6, *y);
}
