use self::Action::*;
use self::Direction::*;

enum Action {
    Drive,
    Turn(Direction),
    Stop,
}

enum Direction {
    Left,
    Right,
}

fn print_action(a: &Action) {
    match a {
        Drive => println!("Drive forward!"),
        Turn(Left) => println!("Turn left"),
        Turn(Right) => println!("Turn right"),
        Stop => println!("Stop!"),
    }
}

fn main() {

    let program = vec![
        Drive,
        Turn(Left),
        Drive,
        Stop
    ];

    for action in program {
        print_action(&action)
    }
}
