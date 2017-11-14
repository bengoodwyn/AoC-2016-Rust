use std::collections::HashSet;
use std::iter;
use std::iter::FromIterator;

#[cfg(test)]
mod direction_tests {
    use super::Direction;

    const DIRECTIONS: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West
    ];

    #[test]
    fn it_can_turn_to_the_right() {
        for i in 0..DIRECTIONS.len() {
            let start_direction = DIRECTIONS[i];
            let end_direction = start_direction.turn('R');

            assert_eq!(DIRECTIONS[(i + 1) % DIRECTIONS.len()], end_direction);
        }
    }

    #[test]
    fn it_can_turn_to_the_left() {
        for i in 0..DIRECTIONS.len() {
            let start_direction = DIRECTIONS[i];
            let end_direction = start_direction.turn('L');

            assert_eq!(DIRECTIONS[(i + DIRECTIONS.len() - 1) % DIRECTIONS.len()], end_direction);
        }
    }

    #[test]
    #[should_panic]
    fn it_panics_for_an_invalid_direction_to_turn() {
        let direction = Direction::North;
        direction.turn('U');
    }
}

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

#[cfg(test)]
mod position_tests {
    use super::Position;
    use super::Direction;

    #[test]
    fn it_increases_y_to_travel_north() {
        let result = Position{x:5,y:5}.travel(2, Direction::North);

        assert_eq!(result.x, 5);
        assert_eq!(result.y, 7);
    }

    #[test]
    fn it_decreases_y_to_travel_south() {
        let result = Position{x:5,y:5}.travel(2, Direction::South);

        assert_eq!(result.x, 5);
        assert_eq!(result.y, 3);
    }

    #[test]
    fn it_increases_x_to_travel_east() {
        let result = Position{x:5,y:5}.travel(2, Direction::East);

        assert_eq!(result.x, 7);
        assert_eq!(result.y, 5);
    }

    #[test]
    fn it_decreases_x_to_travel_west() {
        let result = Position{x:5,y:5}.travel(2, Direction::West);

        assert_eq!(result.x, 3);
        assert_eq!(result.y, 5);
    }
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

#[cfg(test)]
mod part1_tests {
    #[test]
    fn it_handles_example1() {
        assert_eq!(5, super::part1("R2, L3"));
    }

    #[test]
    fn it_handles_example2() {
        assert_eq!(2, super::part1("R2, R2, R2"));
    }

    #[test]
    fn it_handles_example3() {
        assert_eq!(12, super::part1("R5, L5, R5, R3"));
    }

    #[test]
    fn uselss_move_is_0_length() {
        assert_eq!(0, super::part1("R0"));
    }

    #[test]
    fn simple_move_is_1_length() {
        assert_eq!(1, super::part1("R1"));
    }

    #[test]
    fn simple_move_is_10_length() {
        assert_eq!(10, super::part1("R10"));
    }

    #[test]
    fn can_add_two_moves() {
        assert_eq!(10, super::part1("R5, L5"));
    }

    #[test]
    fn can_get_back_to_origin() {
        assert_eq!(0, super::part1("R3, L3, L3, L3"));
    }
}

pub fn part1(input: &str) -> i32 {
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

#[cfg(test)]
mod part2_tests {
    #[test]
    fn it_handles_example1() {
        assert_eq!(4, super::part2("R8, R4, R4, R8"));
    }

    #[test]
    #[should_panic]
    fn it_panics_without_a_repeat() {
        super::part2("R5, L5, R5, L5, R5, L5");
    }

    #[test]
    fn it_stops_if_we_get_back_to_origin() {
        assert_eq!(0, super::part2("R3, L3, L3, L3, R3"));
    }

    #[test]
    fn it_stops_if_we_get_back_to_a_previous_point() {
        assert_eq!(3, super::part2("L3, R3, L3, L3, L3, R3"));
    }

    #[test]
    fn it_stops_if_we_cross_a_previous_point_without_intending_to_stop_on_it() {
        assert_eq!(5, super::part2("L5, R3, L3, L3, L300, R3"));
    }
}

pub fn part2(input: &str) -> i32 {
    let initial = PositionAndDirection{ position: Position{x:0, y:0}, direction: Direction::North };
    let initial_visited : HashSet<Position> = HashSet::from_iter(iter::once(Position{x:0, y:0}));
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
            .scan(initial_visited, |visited, current| {
                if visited.insert(current.position) {
                    Some((false,current))
                } else {
                    Some((true,current))
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