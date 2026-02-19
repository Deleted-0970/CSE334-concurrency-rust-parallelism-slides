enum Message {
    Text(String),
    Position { x: i32, y: i32 },
}

fn handle_message(msg: &Message) {
    match msg {
        Message::Text(text) => {
            println!("You've received a text: {}", text)
        }
        Message::Position { x, y } => {
            println!("You've received coordinates x: {}, y: {}", x, y);
        }
    }
}

pub fn run() {
    let msg = Message::Text(String::from("hi"));
    handle_message(&msg);

    let msg = Message::Position { x: 6, y: 7 };
    handle_message(&msg);
}
