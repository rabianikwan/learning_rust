// enum adalah serangkaian kemungkinan dari sebuah tipe data

use std::net::Shutdown::Write;

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// prelude
enum Option<T> {
    Some(T),
    None
}
impl Message {
    fn call(&self) {
        println!("Hello world")
    }
}
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

fn main() {
    let message: Message = Message::Write(String::from("Hello"));
    message.call();
    
    let x = 5;
    let y: Option<i8> = Some(5);
    let z = x + y.unwrap();
    println!("{}", z)
}
