use aoc::input;
use std::collections::HashSet;
use std::iter;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn turn(&self, direction_to_turn: char) -> Direction {
        match direction_to_turn {
            'R' => match self {
                &Direction::North => Direction::East,
                &Direction::East => Direction::South,
                &Direction::South => Direction::West,
                &Direction::West => Direction::North,
            },
            'L' => match self {
                &Direction::North => Direction::West,
                &Direction::East => Direction::North,
                &Direction::South => Direction::East,
                &Direction::West => Direction::South,
            },
            _ => panic!("Invalid turn {}", direction_to_turn)
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
    fn turn(&self, direction_to_turn: char) -> Position {
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
    let init_position = Position{x:0, y:0, direction: Direction::North};
    let final_position =
        input
            .split(", ")
            .map(|command| {
                command.split_at(1)
            })
            .map(|(direction_to_turn, distance_to_travel)| {
                (direction_to_turn.chars().nth(0).unwrap(), distance_to_travel.parse::<i32>().unwrap())
            })
            .fold(init_position, |position, (direction_to_turn, distance_to_travel)| {
                position.turn(direction_to_turn).travel(distance_to_travel)
            });
    final_position.x.abs() + final_position.y.abs()
}

pub fn part2(filename: &str) -> i32 {
    let input = input::read(&filename);
    let init_position = Position{x:0, y:0, direction: Direction::North};
    let result =
        input
            .split(", ")
            .map(|command| {
                command.split_at(1)
            })
            .map(|(direction_to_turn, distance_to_travel)| {
                (direction_to_turn.chars().nth(0).unwrap(), distance_to_travel.parse::<usize>().unwrap())
            })
            .flat_map(|(direction_to_turn, distance_to_travel)| {
                iter::repeat(direction_to_turn).take(distance_to_travel).enumerate()
            })
            .scan(init_position, |position, (step, direction_to_turn)| {
                if 0 == step {
                    *position = position.turn(direction_to_turn).travel(1)
                } else {
                    *position = position.travel(1)
                }
                Some(*position)
            })
            .scan(HashSet::new(), |visited, position| {
                if visited.contains(&position.coords()) {
                    Some((true,position))
                } else {
                    visited.insert(position.coords());
                    Some((false,position))
                }
            })
            .filter(|&(done, _)| {
                done
            })
            .next();
    match result {
        Some((true, final_position)) => final_position.x.abs() + final_position.y.abs(),
        _ => panic!("No duplicate positions")
    }
}