// enums1.rs
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Quit = 1,
    Echo = 2,
    Move = 4,
    ChangeColor = 8,
    // TODO: define a few types of messages as used below
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
