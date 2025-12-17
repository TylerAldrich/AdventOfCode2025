use crate::parser::parse_file_from_day;


fn str_into_nums(string: &String) -> Vec<usize> {
    let mut nums: Vec<usize> = Vec::with_capacity(string.len());
    for c in string.chars() {
        nums.push(c as usize - '0' as usize);
    }
    nums
}

pub(crate) fn solution1(is_test: bool) -> usize {
    let input = parse_file_from_day(3, is_test);

    let mut solution = 0;
    for bank in input {
        let nums = str_into_nums(&bank);

        let split_nums = nums.split_at(nums.len() - 1);
        let first_set = split_nums.0.to_vec();
        let first_max = *first_set.clone().iter().max().unwrap();

        let index = first_set.iter().position(|x| *x == first_max).unwrap();
        let second_max = nums.split_at(index + 1).1.iter().max().unwrap();

        solution += first_max * 10 + second_max;
    }

    solution
}

pub(crate) fn solution2(is_test: bool) -> usize {
    let input = parse_file_from_day(3, is_test);

    let mut solution = 0;
    for bank in input {
        let mut nums = str_into_nums(&bank);

        let mut final_nums = Vec::new();
        let max_nums = 12;
        while final_nums.len() < max_nums {
            // Continue to get the max number in the "first half" of the remaining numbers available to use
            // The remaining numbers are split based on how many numbers are left available to use in the str.
            let first_half = nums.split_at(nums.len() - (max_nums - final_nums.len() - 1)).0.to_vec();

            let max = first_half.iter().max().unwrap();
            let index = first_half.iter().position(|x| x == max).unwrap();
            final_nums.push(*max);

            nums = nums.split_at(index + 1).1.to_vec();
        }

        for (i, value) in final_nums.iter().enumerate() {
            solution += value * 10usize.pow(11 - i as u32);
        }
    }

    solution
}