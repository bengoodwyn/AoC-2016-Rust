use aoc::input;
use std::collections::HashSet;
use std::iter;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn turn(self, direction_to_turn: char) -> Direction {
        match direction_to_turn {
            'R' => match self {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
            'L' => match self {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            },
            _ => panic!("Invalid turn {}", direction_to_turn)
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn travel(&self, distance_to_travel: i32, direction_to_travel: Direction) -> Position {
        match direction_to_travel {
            Direction::North => Position{x: self.x, y: self.y + distance_to_travel },
            Direction::South => Position{x: self.x, y: self.y - distance_to_travel },
            Direction::East => Position{x: self.x + distance_to_travel, y: self.y },
            Direction::West => Position{x: self.x - distance_to_travel, y: self.y }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct PositionAndDirection {
    position: Position,
    direction: Direction
}

pub fn part1(filename: &str) -> i32 {
    let input = input::read(&filename);
    let initial = PositionAndDirection{ position: Position{x:0, y:0}, direction: Direction::North };
    let result =
        input
            .split(", ")
            .map(|command| {
                command.split_at(1)
            })
            .map(|(direction_to_turn, distance_to_travel)| {
                (direction_to_turn.chars().nth(0).unwrap(), distance_to_travel.parse::<i32>().unwrap())
            })
            .fold(initial, |mut current, (direction_to_turn, distance_to_travel)| {
                current.direction = current.direction.turn(direction_to_turn);
                current.position = current.position.travel(distance_to_travel, current.direction);
                current
            });
    result.position.x.abs() + result.position.y.abs()
}

pub fn part2(filename: &str) -> i32 {
    let input = input::read(&filename);
    let initial = PositionAndDirection{ position: Position{x:0, y:0}, direction: Direction::North };
    let final_position =
        input
            .split(", ")
            .map(|command| (&command[..1], &command[1..]) )
            .map(|(direction_to_turn, distance_to_travel)|
                (direction_to_turn.chars().next().unwrap(),
                 distance_to_travel.parse::<usize>().unwrap())
            )
            .flat_map(|(direction_to_turn, distance_to_travel)|
                iter::repeat(direction_to_turn).take(distance_to_travel).enumerate()
            )
            .scan(initial, |current, (step, direction_to_turn)| {
                if 0 == step {
                    current.direction = current.direction.turn(direction_to_turn)
                }
                current.position = current.position.travel(1, current.direction);
                Some(*current)
            })
            .scan(HashSet::new(), |visited, current| {
                if visited.contains(&current.position) {
                    Some((true,current))
                } else {
                    visited.insert(current.position);
                    Some((false,current))
                }
            })
            .filter_map(|(done, position_and_direction)| {
                if done {
                    Some(position_and_direction.position)
                } else {
                    None
                }
            } )
            .next()
            .unwrap();
    final_position.x.abs() + final_position.y.abs()
}