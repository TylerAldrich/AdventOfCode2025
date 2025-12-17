use std::ops::Index;
use crate::parser::parse_file_from_day;


fn is_invalid(num: &str) -> bool {
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

pub(crate) fn solution1(is_test: bool) -> usize {
    let input = parse_file_from_day(2, is_test);
    let ids = input.index(0).split(',');

    let mut solution = 0;

    for id_range in ids {
        let mut range = id_range.split('-');
        let start_range = range.next().unwrap().parse::<usize>().unwrap();
        let end_range = range.next().unwrap().parse::<usize>().unwrap();

        for id in start_range..=end_range {
            if is_invalid(&id.to_string()) {
                solution += id
            }
        }
    }

    solution
}

pub(crate) fn solution2(is_test: bool) -> usize {
    let input = parse_file_from_day(2, is_test);
    let _ids = input.index(0).split(',');

    1
}