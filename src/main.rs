mod aoc;
use aoc::input;

fn main() {
    println!("Day 1 Part 1: {}", aoc::day1::part1(&input::read("inputs/1.txt")));
    println!("Day 1 Part 2: {}", aoc::day1::part2(&input::read("inputs/1.txt")));
    println!("Day 2 Part 1: {}", aoc::day2::part1(&input::read("inputs/2.txt")));
    println!("Day 2 Part 2: {}", aoc::day2::part2(&input::read("inputs/2.txt")));
    println!("Day 3 Part 1: {}", aoc::day3::part1(&input::read("inputs/3.txt")));
}
