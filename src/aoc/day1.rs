use aoc::input;
use std::collections::HashSet;

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
    fn travel(&self, distance_to_travel: i32) -> Position {
        match self.direction {
            Direction::North => Position{x: self.x, y: self.y + distance_to_travel, direction: self.direction },
            Direction::South => Position{x: self.x, y: self.y - distance_to_travel, direction: self.direction },
            Direction::East => Position{x: self.x + distance_to_travel, y: self.y, direction: self.direction },
            Direction::West => Position{x: self.x - distance_to_travel, y: self.y, direction: self.direction }
        }
    }
    fn coords(&self) -> (i32,i32) {
        (self.x, self.y)
    }
}

pub fn part1(filename: &str) -> i32 {
    let input = input::read(&filename);
    let mut position = Position{x:0, y:0, direction: Direction::North};
    for command in input.split(", ") {
        let (direction_to_turn, distance_to_travel) = command.split_at(1);
        let distance_to_travel = distance_to_travel.parse::<i32>().unwrap();
        position = position.turn(direction_to_turn);
        position = position.travel(distance_to_travel);
    }
    position.x.abs() + position.y.abs()
}

pub fn part2(filename: &str) -> i32 {
    let mut visited = HashSet::new();
    let input = input::read(&filename);
    let mut position = Position{x:0, y:0, direction: Direction::North};
    for command in input.split(", ") {
        let (direction_to_turn, distance_to_travel) = command.split_at(1);
        let distance_to_travel = distance_to_travel.parse::<i32>().unwrap();
        position = position.turn(direction_to_turn);
        for _ in 0..distance_to_travel {
            position = position.travel(1);
            let coords = position.coords();
            if visited.contains(&coords) {
                return position.x.abs() + position.y.abs()
            }
            visited.insert(coords);
        }
    }
    panic!("Did not visit the same position twice")
}