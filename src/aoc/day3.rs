use std::vec::Vec;

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

#[cfg(test)]
mod part2_tests {
    #[test]
    fn it_translates_a_batch_of_nine() {
        let result = super::part2("5 25 3\n10 10 4\n25 5 5");
        assert_eq!(1, result);
    }
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| line.split_whitespace())
        .map(|value| value.parse::<u32>().unwrap() )
        .scan(Vec::new(), |state, value| {
            state.push(value);
            let result = state.clone();
            if state.len() == 9 {
                state.clear();
            }
            Some(result)
        })
        .filter(|vec| vec.len() == 9 )
        .flat_map(|vec| {
            [vec[0], vec[3], vec[6], vec[1], vec[4], vec[7], vec[2], vec[5], vec[8]].to_vec()
        })
        .scan( Vec::new(), | state, value| {
            state.push(value);
            let result = state.clone();
            if state.len() == 3 {
                state.clear();
            }
            Some(result)
        })
        .filter(|vec| vec.len() == 3 )
        .map(|unsorted| {
            let mut sorted = unsorted.clone();
            sorted.sort();
            sorted
        })
        .filter(|vec| (vec[0] + vec[1] > vec[2]))
        .count()
}