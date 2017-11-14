#[cfg(test)]
mod part1_tests {
    #[test]
    fn it_passes_example1() {
        assert_eq!(0, super::part1("5 10 25"))
    }
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line|
            line
                .split_whitespace()
                .map(|value| value.parse::<u32>().unwrap() )
                .collect::<Vec<u32>>()
        )
        .filter(|vec| vec.len() == 3)
        .filter(|vec| (vec[0] + vec[1] > vec[2]))
        .count()
}