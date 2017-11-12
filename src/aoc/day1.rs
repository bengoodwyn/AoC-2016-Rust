use aoc::input;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn turn(&self, direction_to_turn: &str) -> Direction {
        match direction_to_turn {
            "R" => match self {
                &Direction::North => Direction::East,
                &Direction::East => Direction::South,
                &Direction::South => Direction::West,
                &Direction::West => Direction::North,
            },
            "L" => match self {
                &Direction::North => Direction::West,
                &Direction::East => Direction::North,
                &Direction::South => Direction::East,
                &Direction::West => Direction::South,
            },
            &_ => panic!("Invalid turn {}", direction_to_turn)
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    direction: Direction
}

impl Position {
    fn turn(&self, direction_to_turn: &str) -> Position {
        Position{x: self.x, y: self.y, direction: self.direction.turn(direction_to_turn)}
    }
    fn travel(&self, distance_to_travel: &str) -> Position {
        let distance = distance_to_travel.parse::<i32>().unwrap();
        match self.direction {
            Direction::North => Position{x: self.x, y: self.y + distance, direction: self.direction },
            Direction::South => Position{x: self.x, y: self.y - distance, direction: self.direction },
            Direction::East => Position{x: self.x + distance, y: self.y, direction: self.direction },
            Direction::West => Position{x: self.x - distance, y: self.y, direction: self.direction }
        }
    }
}

pub fn part1(filename: &str) -> i32 {
    let input = input::read(&filename);
    let mut position = Position{x:0, y:0, direction: Direction::North};
    for command in input.split(", ") {
        let (direction_to_turn, distance_to_travel) = command.split_at(1);
        position = position.turn(direction_to_turn);
        position = position.travel(distance_to_travel);
    }
    position.x.abs() + position.y.abs()
}