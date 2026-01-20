enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {

    let go = Direction::Left;

    match go {
        Direction::Up => println!("go up"),
        Direction::Down => println!("go down"),
        Direction::Left => println!("go left"),
        Direction::Right => println!("go right"),
    }
}
