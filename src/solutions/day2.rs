use std::ops::Index;
use crate::parser::parse_file_from_day;


fn is_invalid_part1(num: &str) -> bool {
    // Invalid IDs are any range that has digits repeated twice.
    // Any digit with an odd number of digits is immediately valid.
    // Any even digit number we need to split the digits in half and compare left half == right half
    if num.len() % 2 != 0 {
        false
    } else {
        let half = num.len() / 2;
        let first_half = &num[0..half];
        let second_half = &num[half..];
        first_half.eq(second_half)
    }
}

fn is_invalid_part2(num: &str) -> bool {
    // Now, an ID is invalid if it is made only of some sequence of digits repeated at least twice.
    // So, 12341234 (1234 two times), 123123123 (123 three times), 1212121212 (12 five times),
    // and 1111111 (1 seven times) are all invalid IDs.
    let len = num.len();
    let max_split = len / 2;

    // Algorithm: Iterate over all possible ways to split this number (e.g. 1 digit, 2 digit, etc.)
    // The max split is half the length of the digit (e.g. 123123, max_split = 3 because "1231" could never repeat
    // Then, check if the pattern split by that many digits repeats and bail early if it doesn't match.
    // e.g. for "123123", check if "1" repeats, then "12", then "123".
    // If the string isn't divisible by the current split step being tested, we can continue early.
    // e.g. "1234567" can't have a repeating 2 digit or 3 digit pattern in it, so only check for 1 digit repeating.
    for steps in 1..=max_split {
        if len % steps != 0 {
            continue;
        }

        let mut all_sections_match = true;
        for start in (0..len - steps).step_by(steps) {
            let half = start + steps;
            let end = start + steps + steps;
            if !&num[start..half].eq(&num[half..end]) {
                all_sections_match = false;
                break;
            }
        }

        if all_sections_match {
            return true;
        }
    }

    false
}


pub(crate) fn solution1(is_test: bool) -> usize {
    let input = parse_file_from_day(2, is_test);
    let ids = input.index(0).split(',');

    let mut solution = 0;

    for id_range in ids {
        let mut range = id_range.split('-');
        let start_range = range.next().unwrap().parse::<usize>().unwrap();
        let end_range = range.next().unwrap().parse::<usize>().unwrap();

        for id in start_range..=end_range {
            if is_invalid_part1(&id.to_string()) {
                solution += id
            }
        }
    }

    solution
}

pub(crate) fn solution2(is_test: bool) -> usize {
    let input = parse_file_from_day(2, is_test);
    let ids = input.index(0).split(',');

    let mut solution = 0;

    for id_range in ids {
        let mut range = id_range.split('-');
        let start_range = range.next().unwrap().parse::<usize>().unwrap();
        let end_range = range.next().unwrap().parse::<usize>().unwrap();

        for id in start_range..=end_range {
            if is_invalid_part2(&id.to_string()) {
                solution += id
            }
        }
    }

    solution
}