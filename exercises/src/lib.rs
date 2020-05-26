pub mod multiples {
    fn is_multiple(number: i64, multiple: i64) -> bool {
        if (number % multiple) == 0 {
            return true;
        }

        false
    }


    pub fn multiples_of(max_number: i64) -> i128 {
        let three = 3;
        let five = 5;
        let mut answer: i128 = 0;

        for number in 0..(max_number) {
            if is_multiple(number, three) || is_multiple(number, five) {
                answer = answer + number as i128;
            }
        }

        return answer;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_is_multiple() {
            assert_eq!(is_multiple(1, 3), false);
            assert_eq!(is_multiple(2, 3), false);
            assert_eq!(is_multiple(3, 3), true);
            assert_eq!(is_multiple(4, 3), false);
            assert_eq!(is_multiple(5, 3), false);
            assert_eq!(is_multiple(6, 3), true);
            assert_eq!(is_multiple(7, 3), false);
            assert_eq!(is_multiple(8, 3), false);
            assert_eq!(is_multiple(9, 3), true);
            assert_eq!(is_multiple(10, 3), false);
        }

        #[test]
        fn test_multiples_of() {
            assert_eq!(multiples_of(10), 23);
            assert_eq!(multiples_of(1000), 233168);
            assert_eq!(multiples_of(1000000), 233333166668);
        }
    }
}

