fn main() {
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}")



    // loop {
    //     println!("print me ");
    // }

    let mut x: Vec<i32> = Vec::new();
    let v = vec![1,2,3];

    x.push(4);
    x.push(5);

    let val = &x[1];
    let val1 = v.get(2);

    match val1 {
        Some(val1) => println!("{val1}"),
        None => println!("notiing")
    };

    println!("{}, is", val);
}
