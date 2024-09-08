fn main() {
    let s: &str = "Hello world by me";
    s.to_string();
    println!("{s}");

    let mut stringinit = String::new();
    stringinit.push_str("my boi");
    stringinit.push('y');

    let new_str = s.to_string() + &stringinit;
    println!("{stringinit}");
    println!("{new_str}");

    // con
    let hello = "Здравствуйте";

    let sim = &hello[0..6];
    println!("{sim}");

    // using format!

    let ryan = String::from("Ryan");
    let game = String::from("is playing Cricket");
    let place = String::from("in the ground");

    let concat = format!("{ryan} {game} {place}");

    println!("{concat}");
}
