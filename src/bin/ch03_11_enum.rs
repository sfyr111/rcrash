fn main() {
    // 1. Simple enum (no data)
    enum Direction {
        North,
        East,
        South,
        West,
    }

    let dir = Direction::East;
    // Manual display for Direction
    fn print_direction(d: &Direction) {
        match d {
            Direction::North => println!("Direction: North"),
            Direction::East => println!("Direction: East"),
            Direction::South => println!("Direction: South"),
            Direction::West => println!("Direction: West"),
        }
    }
    print_direction(&dir);

    // 2. Enum with explicit discriminant values
    enum Status {
        Ok = 200,
        NotFound = 404,
        InternalError = 500,
    }
    let st = Status::NotFound;
    // Manual display for Status
    fn print_status(s: Status) {
        let name = match s {
            Status::Ok => "Ok",
            Status::NotFound => "NotFound",
            Status::InternalError => "InternalError",
        };
        println!("Status: {} as number: {}", name, s as i32);
    }
    print_status(st);

    // 3. Enum with data (struct-like and tuple-like variants)
    enum Message {
        Quit,                         // No data
        Move { x: i32, y: i32 },      // Struct-like variant
        Write(String),                // Tuple-like variant
        ChangeColor(i32, i32, i32),   // Tuple-like variant
    }

    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(255, 0, 0);

    // Manual display for Message
    fn print_message(msg: &Message) {
        match msg {
            Message::Quit => println!("Message: Quit"),
            Message::Move { x, y } => println!("Message: Move to x={}, y={}", x, y),
            Message::Write(s) => println!("Message: Write '{}'", s),
            Message::ChangeColor(r, g, b) => println!("Message: ChangeColor({}, {}, {})", r, g, b),
            _ => println!("Message: Unknown"),
        }
    }
    print_message(&m1);
    print_message(&m2);
    print_message(&m3);
    print_message(&m4);
}
