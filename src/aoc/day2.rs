use aoc::input;

fn new_key(start_key: char, direction: char) -> char {
    match direction {
        'U' => match start_key {
            '4' => '1',
            '5' => '2',
            '6' => '3',
            '7' => '4',
            '8' => '5',
            '9' => '6',
            _ => start_key
        },
        'D' => match start_key {
            '1' => '4',
            '2' => '5',
            '3' => '6',
            '4' => '7',
            '5' => '8',
            '6' => '9',
            _ => start_key
        },
        'L' => match start_key {
            '2' => '1',
            '3' => '2',
            '5' => '4',
            '6' => '5',
            '8' => '7',
            '9' => '8',
            _ => start_key
        },
        'R' => match start_key {
            '1' => '2',
            '2' => '3',
            '4' => '5',
            '5' => '6',
            '7' => '8',
            '8' => '9',
            _ => start_key
        },
        _ => panic!("Invalid direction {}", direction)
    }
}

pub fn part1(filename: &str) -> String {
    let input = input::read(&filename);
    let mut key = '5';
    let mut code = String::new();
    for line in input.split("\n") {
        for direction in line.chars() {
            key = new_key(key, direction)
        }
        code.push(key)
    }
    code
}