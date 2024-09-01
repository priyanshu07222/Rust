// fn main() {
//     let x = -34;
//     let y = 45;
//     let z: f32 = 68.8998;
//     let _pri = "priyanshu";
//     let greeting = String::from("Good moringin");
//     println!("{}", greeting);

//     print!("x:{}, y: {}, z: {}", x, y, z);
// }

// fn main() {
//     let istrue = true;

//     if istrue {
//         print!("istrue is true");
//     } else {
//         print!("istrue is false");
//     }

//     for i in 0..10 {
//         print!("{}", i)
//     }
// }

// fn main() {
//     let sentence = String::from("my name is priyanshu");
//     let first_word = get_first_word(sentence);
//     print!("first word is {}",first_word);
// }

// fn get_first_word(sentence:String) -> String {
//     let mut ans = String::from("");
//     for char in sentence.chars() {
//         // print!("the char is: {}", char );
//         ans.push_str(char.to_string().as_str());
//         if char == ' ' {
//             break;
//         }
//     }
//     return ans;
// }

// struct User {
//     name: String,
//     age: u32,
//     isactive: bool,
// }

// fn main(){
//     let user = User {
//         name: String::from("Priyanshu"),
//         age: 20,
//         isactive: true,
//     };
// }

fn main() {
    println!("Hello Guys, I'm learning Learning Rust")
}
