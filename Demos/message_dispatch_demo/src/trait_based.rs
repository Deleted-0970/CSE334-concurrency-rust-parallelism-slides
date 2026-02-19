trait Message {
    fn announce(&self) -> String;
}

struct TextMsg {
    text: String,
}

struct PositionMsg {
    x: i32,
    y: i32,
}

impl Message for TextMsg {
    fn announce(&self) -> String {
        format!("You've received a text: {}", self.text)
    }
}

impl Message for PositionMsg {
    fn announce(&self) -> String {
        format!("You've received coordinates x: {}, y: {}", self.x, self.y)
    }
}

/// Uses compile-time monomorphization.
/// (like C++ templates)
/// 
/// Compiler creates a version of this function for each
/// possible T, and chooses which one to call at compile time.
fn handle_message_1<T: Message>(msg: &T) {
    println!("{}", msg.announce());
}

/// Uses run-time dynamic dispatch.
/// (like Java polymorphism)
/// 
/// Compiles to a single function.
/// Chooses at runtime which `announce()` to call.
/// 
/// Msg is a fat pointer to a type that implements `Message`,
/// and a vtable that points to the right method for the concrete type.
fn handle_message_2(msg: &dyn Message) {
    println!("{}", msg.announce());
}

pub fn run() {
    let msg = TextMsg {
        text: String::from("hi"),
    };
    handle_message_1(&msg);

    let msg = PositionMsg { x: 6, y: 7 };
    handle_message_1(&msg);

    // Array of fat pointers to objects
    // that implement the message trait.
    let messages: Vec<Box<dyn Message>> = vec![
        Box::new(TextMsg {
            text: String::from("hi"),
        }),
        Box::new(PositionMsg { x: 6, y: 7 }),
    ];

    handle_message_2(&*messages[0]);
    handle_message_2(&*messages[1]);
}
