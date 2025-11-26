/*
   -------------------------------------------------------------------------
   Exercise 1: Defining multiple enum members (without extra logic or value)
   -------------------------------------------------------------------------
*/

#[derive(Debug)]
enum SimpleMessage {
    Resize,
    Move,
    Echo,
    ChangeColor,
    Quit,
}

/*
   ----------------------------------
   Exercise 2: Enums related to types
   ----------------------------------
*/

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

/*
   ----------------------------------
   Exercise 3: Enums related to types
   ----------------------------------
*/

#[derive(Debug)]
struct State {
    width: u64,
    height: u64,
    position: Point,
    message: String,
    color: (u8, u8, u8), // RGB color
    quit: bool,
}

impl State {
    fn resize(&mut self, width: u64, heigh: u64) {
        self.width = width;
        self.height = heigh;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn process(&mut self, message: Message) {
        // general function to process all messages and call the correct logic
        match message {
            Message::Resize { width, height } => self.resize(width, height),
            Message::Move(point) => self.move_position(point),
            Message::Echo(s) => self.echo(s),
            Message::ChangeColor(r, g, b) => self.change_color(r, g, b),
            Message::Quit => self.quit(),
        }
    }
}

fn main() {
    // Exercise 1:
    println!("{:?}", SimpleMessage::Resize);
    println!("{:?}", SimpleMessage::Move);
    println!("{:?}", SimpleMessage::Echo);
    println!("{:?}", SimpleMessage::ChangeColor);
    println!("{:?}", SimpleMessage::Quit);

    // Exercise 2:
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("Hello World!")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];
    for message in &messages {
        message.call();
    }

    // Exercise 3:
    let mut state = State {
        width: 0,
        height: 0,
        position: Point { x: 0, y: 0 },
        message: String::from("Hello world"),
        color: (0, 0, 0),
        quit: false,
    };
    print!("Initial state:");
    dbg!(&state);

    state.process(Message::Resize {
        width: 10,
        height: 30,
    });
    state.process(Message::Move(Point { x: 10, y: 15 }));
    state.process(Message::Echo(String::from("HELLO WORLD!")));
    state.process(Message::ChangeColor(255, 0, 255));
    state.process(Message::Quit);
    print!("Modified state:");
    dbg!(&state);

    assert_eq!(state.width, 10);
    assert_eq!(state.height, 30);
    assert_eq!(state.position.x, 10);
    assert_eq!(state.position.y, 15);
    assert_eq!(state.message, "HELLO WORLD!");
    assert_eq!(state.color, (255, 0, 255));
    assert!(state.quit);

    println!("All test passed!");
}
