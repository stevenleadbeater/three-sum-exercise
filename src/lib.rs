use std::ops::Index;

struct ThreeSum {
    sorted_numbers: Vec<i64>,
}

impl ThreeSum {
    fn new(numbers: Vec<i64>) -> ThreeSum {
        let mut instance = ThreeSum {
            sorted_numbers: numbers
        };
        instance.sorted_numbers.sort();
        instance
    }

    fn get_triples_sum_to_zero(&self) -> Vec<Vec<i64>> {
        let mut result: Vec<Vec<i64>> = vec![];
        for index in 0..self.sorted_numbers.len() {
            if self.sorted_numbers[index] > 0 {
                break;
            }
            if index != 0 && self.sorted_numbers[index] == self.sorted_numbers[index - 1] {
                continue;
            }
            let mut low_pointer = index + 1;
            let mut high_pointer = self.sorted_numbers.len() - 1;
            while low_pointer < high_pointer {
                if self.sorted_numbers[index] + self.sorted_numbers[low_pointer] + self.sorted_numbers[high_pointer] == 0 {
                    result.push(vec![self.sorted_numbers[index], self.sorted_numbers[low_pointer], self.sorted_numbers[high_pointer]]);
                }
                if self.sorted_numbers[index] + self.sorted_numbers[low_pointer] + self.sorted_numbers[high_pointer] >= 0 {
                    high_pointer -= 1;
                } else if self.sorted_numbers[index] + self.sorted_numbers[low_pointer] + self.sorted_numbers[high_pointer] <= 0 {
                    low_pointer += 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::ThreeSum;

    #[test]
    fn new_sorts() {
        assert_eq!(ThreeSum::new(vec![-1, 0, 1, 2, -1, -4]).sorted_numbers, vec![-4, -1, -1, 0, 1, 2])
    }

    #[test]
    fn finds_2() {
        let sorter = ThreeSum::new(vec![-1, 0, 1, 2, -1, -4]);
        assert_eq!(sorter.get_triples_sum_to_zero(), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn finds_none() {
        let sorter = ThreeSum::new(vec![-10, 0, 9, 2, -10, -40]);
        let expected: Vec<Vec<i64>> = vec![];
        assert_eq!(sorter.get_triples_sum_to_zero(), expected);
    }
}
