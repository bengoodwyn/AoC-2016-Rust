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

fn new_crazy_key(start_key: char, direction: char) -> char {
    match direction {
        'U' => match start_key {
            '3' => '1',
            '6' => '2',
            '7' => '3',
            '8' => '4',
            'A' => '6',
            'B' => '7',
            'C' => '8',
            'D' => 'B',
            _ => start_key
        },
        'D' => match start_key {
            '1' => '3',
            '2' => '6',
            '3' => '7',
            '4' => '8',
            '6' => 'A',
            '7' => 'B',
            '8' => 'C',
            'B' => 'D',
            _ => start_key
        },
        'L' => match start_key {
            '9' => '8',
            '4' => '3',
            '8' => '7',
            'C' => 'B',
            '3' => '2',
            '7' => '6',
            'B' => 'A',
            '6' => '5',
            _ => start_key
        },
        'R' => match start_key {
            '8' => '9',
            '3' => '4',
            '7' => '8',
            'B' => 'C',
            '2' => '3',
            '6' => '7',
            'A' => 'B',
            '5' => '6',
            _ => start_key
        },
        _ => panic!("Invalid direction {}", direction)
    }
}

fn keypad(filename: &str, keypad_fn: &Fn(char, char) -> char) -> String {
    let input = input::read(&filename);
    input
        .lines()
        .map(|line|{
            line
                .chars()
                    .fold('5', |key, direction| {
                        keypad_fn(key, direction)
                    })
        })
        .collect::<String>()
}

pub fn part1(filename: &str) -> String {
    keypad(filename, &new_key)
}

pub fn part2(filename: &str) -> String {
    keypad(filename, &new_crazy_key)
}