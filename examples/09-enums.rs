// Define a basic enum and an enum with associated data
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
enum Command {
    Move { x: f32, y: f32 },
    Quit,
}
impl Direction {
    fn print(&self) {
        match self {
            Direction::Up => println!("Up"),
            Direction::Down => println!("Down"),
            Direction::Left => println!("Left"),
            Direction::Right => println!("Right"),
        }
    }
}
fn main() {
    // Use the enums
    let up = Direction::Up;
    let cmd = Command::Move { x: 1.0, y: 2.0 };
    match cmd {
        Command::Move { x, y } => println!("Move to {}, {}", x, y),
        Command::Quit => {
            println!("Quit");
            return;
        }
    }
}
