#![allow(dead_code)]

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    Move (Point),
    Echo(String),
    ChangeColor(i32,i32,i32),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
