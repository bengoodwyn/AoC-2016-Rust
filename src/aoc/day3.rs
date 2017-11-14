#[cfg(test)]
mod part1_tests {
    #[test]
    fn it_passes_example1() {
        assert_eq!(0, super::part1("5 10 25"));
    }

    #[test]
    fn it_passes_an_invalid_triangle_in_reverse_order() {
        assert_eq!(0, super::part1( "25 10 5"));
    }

    #[test]
    fn it_passes_a_valid_triangle() {
        assert_eq!(1, super::part1( "3 4 5"));
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
        .map(|unsorted| {
            let mut sorted = unsorted.clone();
            sorted.sort();
            sorted
        })
        .filter(|vec| vec.len() == 3)
        .filter(|vec| (vec[0] + vec[1] > vec[2]))
        .count()
}