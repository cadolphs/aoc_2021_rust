use crate::inputs::read_file_to_list;
use itertools::Itertools;

pub fn run_day_01() {
    let filename = "data/day01.txt";
    let data: Vec<i32> = read_file_to_list(filename).unwrap();
    let res = count_increasing(data.iter());
    println!("There are {} decreasing measurements in the input.", res);
    let res = count_triplet_sum_increasing(&data);
    println!(
        "There are {} decreasing triplet measurements in the input.",
        res
    );
}

pub fn count_increasing<T: PartialOrd + Copy, I: Iterator<Item = T>>(values: I) -> usize {
    values
        .tuple_windows::<(_, _)>()
        .filter(|&window| window.0 < window.1)
        .count()
}

pub fn count_triplet_sum_increasing(values: &[i32]) -> usize {
    count_increasing(values.get_triplet_sums())
}

pub trait TripletSums {
    fn get_triplet_sums(&self) -> EfficientTripletSumIter;
}

impl<T: AsRef<[i32]>> TripletSums for T {
    fn get_triplet_sums(&self) -> EfficientTripletSumIter {
        EfficientTripletSumIter {
            data: self.as_ref(),
            pos: 0,
            current_sum: 0,
        }
    }
}

pub struct EfficientTripletSumIter<'a> {
    data: &'a [i32],
    pos: usize,
    current_sum: i32,
}

impl<'a> Iterator for EfficientTripletSumIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == 0 {
            if self.data.len() >= 3 {
                self.pos += 1;
                self.current_sum = self.data.iter().take(3).sum();
                return Some(self.current_sum);
            } else {
                return None;
            }
        }

        if self.pos + 2 < self.data.len() {
            self.current_sum -= self.data[self.pos - 1];
            self.current_sum += self.data[self.pos + 2];
            self.pos += 1;
            Some(self.current_sum)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day01::count_increasing;
    use crate::day01::count_triplet_sum_increasing;

    #[test]
    fn empty_list_doesnt_increase() {
        let empty: Vec<i32> = Vec::new();
        assert_eq!(0, count_increasing(empty.iter()));
    }

    #[test]
    fn list_of_length_ones_doesnt_increase() {
        assert_eq!(0, count_increasing([1].iter()));
    }

    #[test]
    fn try_example_input() {
        let example_input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, count_increasing(example_input.iter()));
        assert_eq!(5, count_triplet_sum_increasing(&example_input));
    }
}
