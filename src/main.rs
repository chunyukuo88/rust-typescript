enum Colors {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Colors {
    fn is_green(&self) -> bool {
        if let Colors::Green = self {
            return true;
        }
        return false;
    }

    fn is_green_parts(&self) -> bool {
        if let Colors::Blue = self {
            return true;
        }
        if let Colors::Yellow = self {
            return true;
        }
        return false;
    }
}

fn print_color(color: Colors) {
    match color {
        Colors::Red => println!("red"),
        Colors::Green => println!("green"),
        Colors::Blue => println!("blue"),
        Colors::Yellow => {}
    }
}

fn main() {
    print_color(Colors::Red);
    println!("{:?}", Colors::is_green(&Colors::Green));
    println!("{:?}", Colors::is_green(&Colors::Blue));
    println!("{:?}", Colors::is_green_parts(&Colors::Yellow));
    println!("{:?}", Colors::is_green_parts(&Colors::Red));
}