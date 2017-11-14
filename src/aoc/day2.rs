#[cfg(test)]
mod position_tests {
    use super::Position;

    #[test]
    fn it_decreases_y_when_moving_up() {
        let result = Position{x:3, y:3}.moved('U');

        assert_eq!(3, result.x);
        assert_eq!(2, result.y);
    }

    #[test]
    fn it_prevents_overflow_when_moving_up() {
        let result = Position{x:3, y:0}.moved('U');

        assert_eq!(3, result.x);
        assert_eq!(0, result.y);
    }

    #[test]
    fn it_increases_y_when_moving_down() {
        let result = Position{x:3, y:3}.moved('D');

        assert_eq!(3, result.x);
        assert_eq!(4, result.y);
    }

    #[test]
    fn it_prevents_overflow_when_moving_down() {
        let result = Position{x:3, y:4}.moved('D');

        assert_eq!(3, result.x);
        assert_eq!(4, result.y);
    }

    #[test]
    fn it_decreases_x_when_moving_left() {
        let result = Position{x:3, y:3}.moved('L');

        assert_eq!(2, result.x);
        assert_eq!(3, result.y);
    }

    #[test]
    fn it_prevents_overflow_when_moving_left() {
        let result = Position{x:0, y:3}.moved('L');

        assert_eq!(0, result.x);
        assert_eq!(3, result.y);
    }

    #[test]
    fn it_increases_x_when_moving_right() {
        let result = Position{x:3, y:3}.moved('R');

        assert_eq!(4, result.x);
        assert_eq!(3, result.y);
    }

    #[test]
    fn it_prevents_overflow_when_moving_right() {
        let result = Position{x:4, y:3}.moved('R');

        assert_eq!(4, result.x);
        assert_eq!(3, result.y);
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize
}

const KEYPAD_DIMENSION: usize = 5;

struct Keypad {
    initial_position: Position,
    keys: [[char; KEYPAD_DIMENSION]; KEYPAD_DIMENSION]
}

const BASIC_KEYPAD : Keypad = Keypad{
    initial_position: Position{x:2,y:2},
    keys:
[
[ ' ', ' ', ' ' ,' ', ' ' ],
[ ' ', '1', '2', '3', ' ' ],
[ ' ', '4', '5', '6', ' ' ],
[ ' ', '7', '8', '9', ' ' ],
[ ' ', ' ', ' ' ,' ', ' ' ]
]
};

const CRAZY_KEYPAD : Keypad = Keypad{
    initial_position: Position{x:0,y:2},
    keys:
[
[ ' ', ' ', '1' ,' ', ' ' ],
[ ' ', '2', '3' ,'4', ' ' ],
[ '5', '6', '7' ,'8', '9' ],
[ ' ', 'A', 'B' ,'C', ' ' ],
[ ' ', ' ', 'D' ,' ', ' ' ]
]
};

impl Position {
    fn moved(&self, direction: char) -> Position {
        match direction {
            'U' if self.y > 0 => Position{ x: self.x, y: self.y-1 },
            'D' if self.y < (KEYPAD_DIMENSION-1) => Position{ x: self.x, y: self.y+1 },
            'L' if self.x > 0 => Position{ x: self.x-1, y: self.y },
            'R' if self.x < (KEYPAD_DIMENSION-1) => Position{ x: self.x+1, y: self.y },
            _ => Position{ x: self.x, y: self.y }
        }
    }
}

#[cfg(test)]
mod new_position_tests {
    use super::Position;
    use super::BASIC_KEYPAD;

    #[test]
    fn it_returns_the_new_position_above_when_valid() {
        let result = super::new_position(Position{x:2, y:2}, 'U', &BASIC_KEYPAD);

        assert_eq!(2, result.x);
        assert_eq!(1, result.y);
    }

    #[test]
    fn it_returns_the_same_position_above_when_space() {
        let result = super::new_position(Position{x:3, y:1}, 'U', &BASIC_KEYPAD);

        assert_eq!(3, result.x);
        assert_eq!(1, result.y);
    }
}

fn new_position(start_pos: Position, direction: char, keypad: &Keypad) -> Position {
    let new_pos = start_pos.moved(direction);
    if keypad.keys[new_pos.y][new_pos.x] == ' ' {
        start_pos
    } else {
        new_pos
    }
}

fn keypad(input: &str, keypad: &Keypad) -> String {
    let mut x = keypad.initial_position.x;
    let mut y = keypad.initial_position.y;
    input
        .lines()
        .map(|line| {
            let end_position =
                line
                .chars()
                .fold(Position{x:x, y:y}, |position, direction| {
                    new_position(position, direction, keypad)
                });
            x = end_position.x;
            y = end_position.y;
            keypad.keys[y][x]
        })
        .collect::<String>()
}

#[cfg(test)]
mod part1_tests {
    #[test]
    fn it_handles_example1() {
        assert_eq!("1985", super::part1("ULL\nRRDDD\nLURDL\nUUUUD"));
    }
}

pub fn part1(input: &str) -> String {
    keypad(input, &BASIC_KEYPAD)
}

#[cfg(test)]
mod part2_tests {
    #[test]
    fn it_handles_example1() {
        assert_eq!("5DB3", super::part2("ULL\nRRDDD\nLURDL\nUUUUD"));
    }
}

pub fn part2(input: &str) -> String {
    keypad(input, &CRAZY_KEYPAD)
}