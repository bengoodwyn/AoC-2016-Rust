use aoc::input;

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

fn new_position(start_pos: Position, direction: char, keypad: &Keypad) -> Position {
    let new_pos = start_pos.moved(direction);
    if keypad.keys[new_pos.y][new_pos.x] == ' ' {
        start_pos
    } else {
        new_pos
    }
}

fn keypad(filename: &str, keypad: &Keypad) -> String {
    let input = input::read(&filename);
    input
        .lines()
        .map(|line| {
            let Position{ x, y } =
                line
                .chars()
                .fold(keypad.initial_position, |position, direction| {
                    new_position(position, direction, keypad)
                });
            keypad.keys[x][y]
        })
        .collect::<String>()
}

pub fn part1(filename: &str) -> String {
    keypad(filename, &BASIC_KEYPAD)
}

pub fn part2(filename: &str) -> String {
    keypad(filename, &CRAZY_KEYPAD)
}