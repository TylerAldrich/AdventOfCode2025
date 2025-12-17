use crate::parser::parse_file_from_day;


pub(crate) fn solution1(is_test: bool) -> usize {
    let input = parse_file_from_day(1, is_test);

    let mut current_number: isize = 50;
    let mut zeroes: usize = 0;

    for value in input {
        let mut chars = value.chars();
        let rotation_dir = chars.next().unwrap();
        let rotation_amount = chars.as_str().parse::<isize>().unwrap();

        match rotation_dir {
            'L' => current_number = (current_number - rotation_amount) % 100,
            'R' => current_number = (current_number + rotation_amount) % 100,
            _ => panic!("Invalid rotation dir: {}", rotation_dir)
        }

        if current_number == 0 {
            zeroes += 1;
        }
    }

    zeroes
}

pub(crate) fn solution2(is_test: bool) -> usize {
    let input = parse_file_from_day(1, is_test);

    let mut current_number: isize = 50;
    let mut zeroes: isize = 0;

    for value in input {
        let mut chars = value.chars();
        let rotation_dir = chars.next().unwrap();
        let mut rotation_amount = chars.as_str().parse::<isize>().unwrap();

        if rotation_amount > 100 {
            zeroes += rotation_amount / 100;
            rotation_amount = rotation_amount % 100;
        }

        match rotation_dir {
            'L' => {
                if current_number != 0 && rotation_amount > current_number {
                    zeroes += 1
                }
                current_number = current_number - rotation_amount;
                if current_number < 0 {
                    current_number = 100 - current_number.abs();
                }

            },
            'R' => {
                if current_number + rotation_amount > 100 && current_number != 0 {
                    zeroes += 1
                }
                current_number = (current_number + rotation_amount) % 100
            },
            _ => panic!("Invalid rotation dir: {}", rotation_dir)
        }

        println!("{}", current_number);

        if current_number == 0 {
            zeroes += 1;
        }
    }

    zeroes.try_into().unwrap()
}