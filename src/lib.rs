struct ThreeSum {
    sorted_numbers: Vec<i64>,
}

impl ThreeSum {
    fn new(numbers: Vec<i64>) -> ThreeSum {
        ThreeSum {
            sorted_numbers: numbers
        }
    }

    fn get_triples_sum_to_zero(&self) -> Vec<Vec<i64>> {
        vec![]
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
        assert_eq!(sorter.get_triples_sum_to_zero(), vec![]);
    }
}
