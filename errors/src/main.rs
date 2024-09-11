use std::fs::File;
use std::io::{self, ErrorKind, Read};
fn main() {
    // println!("Hello, world!");
    // // panic!("crash and burn");

    // let v = vec![1, 2, 3];

    // v[99];

    let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(e) => panic!("Problem create a file: {e:?}"),
            },
            other_error => panic!("Problem opening file: {other_error:?}"),
        },
    };

    // unwrap method to handle Result and give default panic message

    let another_file = File::open("priyanshu.txt").unwrap();

    // expect method to handle Result and developer can give custom message for more clarity, in prod most often expect is used

    let new_file = File::open("newfile.txt").expect("Somethng went wrong while opening the file");

    // error propagation

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(error) => return Err(error),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        };
    }
}
