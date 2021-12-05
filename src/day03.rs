use std::{error::Error, str::FromStr};

use crate::inputs::read_file_to_list;

pub fn run_day_03() {
    let filename = "data/day03.txt";
    let codes: Vec<String> = read_file_to_list(filename).unwrap();
    let num_codes = codes.len();
    let split_codes: Vec<Vec<i32>> = codes
        .into_iter()
        .map(|code| split_code(&code).unwrap())
        .collect();
    let counts = count_ones(&split_codes).unwrap();

    let gamma_rate = find_gamma_rate(&counts, num_codes);
    let epsilon_rate = find_epsilon_rate(gamma_rate, counts.len());
    println!(
        "Gamma and epsilon rates are {} and {}",
        gamma_rate, epsilon_rate
    );
    println!("And their product is {}", gamma_rate * epsilon_rate);

    let (oxygen, scrubber) = find_oxygen_and_scrubber_ratings(split_codes);
    println!("Oxygen and scrubber rates are {} and {}", oxygen, scrubber);
    println!("The checksum is {}", oxygen * scrubber);
}

fn find_gamma_rate(counts: &[i32], num_codes: usize) -> i32 {
    let mut rate = 0;
    let majority = {
        match num_codes % 2 {
            0 => num_codes / 2 + 1,
            1 => (num_codes + 1) / 2,
            _ => panic!(),
        }
    } as i32;
    for count in counts {
        if *count >= majority {
            rate += 1;
        }
        rate <<= 1;
    }
    rate >> 1
}

fn find_epsilon_rate(gamma_rate: i32, code_length: usize) -> i32 {
    let two_complement = (1 << code_length) - 1;
    two_complement - gamma_rate
}

fn split_code<T: FromStr>(code: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    <T as std::str::FromStr>::Err: std::error::Error + 'static,
{
    code.chars().map(|c| Ok(c.to_string().parse()?)).collect()
}

fn count_ones(code_vecs: &[Vec<i32>]) -> Option<Vec<i32>> {
    if code_vecs.is_empty() {
        return None;
    }
    let size = code_vecs[0].len();
    let mut acc = vec![0; size];
    for vec in code_vecs.iter() {
        if vec.len() != size {
            return None;
        }
        for i in 0..size {
            acc[i] += vec[i];
        }
    }
    Some(acc)
}

fn find_oxygen_and_scrubber_ratings(codes: Vec<Vec<i32>>) -> (i32, i32) {
    let (ones, zeros) = partition_by_position(codes, 0);
    let (mut oxygen, mut scrubber) = if ones.len() >= zeros.len() {
        (ones, zeros)
    } else {
        (zeros, ones)
    };
    let mut pos = 0;
    while oxygen.len() != 1 {
        pos += 1;
        oxygen = pick_most_common_partition_by_pos(oxygen, pos);
    }
    pos = 0;
    while scrubber.len() != 1 {
        pos += 1;
        scrubber = pick_least_common_partition_by_pos(scrubber, pos);
    }
    (vec_to_num(&oxygen[0]), vec_to_num(&scrubber[0]))
}

fn vec_to_num(vec: &[i32]) -> i32 {
    let mut acc = 0;
    for val in vec {
        acc += val;
        acc <<= 1;
    }
    acc >> 1
}

fn pick_most_common_partition_by_pos(codes: Vec<Vec<i32>>, pos: usize) -> Vec<Vec<i32>> {
    let (ones, zeros) = partition_by_position(codes, pos);
    if ones.len() >= zeros.len() {
        ones
    } else {
        zeros
    }
}

fn pick_least_common_partition_by_pos(codes: Vec<Vec<i32>>, pos: usize) -> Vec<Vec<i32>> {
    let (ones, zeros) = partition_by_position(codes, pos);
    if zeros.len() <= ones.len() {
        zeros
    } else {
        ones
    }
}

fn partition_by_position(codes: Vec<Vec<i32>>, pos: usize) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    codes.into_iter().partition(|vec| vec[pos] == 1)
}

#[cfg(test)]
mod tests {
    use crate::day03::find_oxygen_and_scrubber_ratings;
use crate::day03::partition_by_position;
    use crate::day03::vec_to_num;
    use crate::day03::{count_ones, find_epsilon_rate, find_gamma_rate};

    use super::split_code;

    #[test]
    fn it_splits_code_as_desired() {
        let input = "10010100";
        let expected = vec![1, 0, 0, 1, 0, 1, 0, 0];

        assert_eq!(expected, split_code(input).unwrap());
    }

    #[test]
    fn it_correctly_counts_ones() {
        let input = vec![vec![1, 0, 0], vec![0, 1, 1], vec![1, 0, 1]];
        let expected = vec![2, 1, 2];
        assert_eq!(expected, count_ones(&input).unwrap());
    }

    #[test]
    fn it_finds_simple_gamma_rate() {
        let counts = vec![3, 1, 4];
        let num_codes = 5;
        let expected = 0b101;
        assert_eq!(expected, find_gamma_rate(&counts, num_codes));
        let expected_epsilon = 0b010;
        assert_eq!(expected_epsilon, find_epsilon_rate(expected, 3));
    }

    #[test]
    fn it_partitions_nicely() {
        let input = vec![vec![1, 0, 0], vec![0, 1, 1], vec![1, 0, 1]];
        let (ones, zeros) = partition_by_position(input, 0);
        let expected = (vec![vec![1, 0, 0], vec![1, 0, 1]], vec![vec![0, 1, 1]]);
        assert_eq!(expected, (ones, zeros));
    }

    #[test]
    fn it_converts_to_binary() {
        assert_eq!(1 + 4 + 8 + 32, vec_to_num(&vec![1, 0, 1, 1, 0, 1]))
    }

    #[test]
    fn it_runs_the_example_test() {
        let input = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        let split_codes: Vec<Vec<i32>> = input.lines()
        .map(|code| split_code(&code).unwrap())
        .collect();
        let (oxygen, scrubber) = find_oxygen_and_scrubber_ratings(split_codes);
        assert_eq!((23, 10), (oxygen, scrubber));
    }
}
