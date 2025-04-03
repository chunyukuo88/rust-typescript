enum Colors {
    Red,
    Green,
    Blue,
}

fn print_color(color: Colors) {
    match color {
        Colors::Red => println!("red"),
        Colors::Green => println!("green"),
        Colors::Blue => println!("blue"),
    }
}

fn main() {
    print_color(Colors::Red);
    print_color(Colors::Green);
    print_color(Colors::Blue);
}