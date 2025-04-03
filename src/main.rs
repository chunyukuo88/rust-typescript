enum Colors {
    Red,
    Green,
    Blue,
}

fn main() {
    match Colors::Red  {
        _ => println!("{}", "Red"),
    }
    match Colors::Green {
        _ => println!("{}", "Green"),
    }
    match Colors::Blue {
        _ => println!("{}", "Blue"),
    }
}