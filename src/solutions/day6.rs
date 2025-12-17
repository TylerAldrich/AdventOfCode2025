use crate::parser::parse_file_from_day;

#[derive(Debug)]
enum Operator {
    ADD,
    MULT
}

pub(crate) fn solution1(is_test: bool) -> usize {
    let input = parse_file_from_day(6, is_test);

    let mut number_grid: Vec<Vec<usize>> = Vec::new();
    let mut operators = Vec::new();
    for line in input {
        let mut number_row = Vec::new();
        for val in line.split(' ') {
            let val = val.trim();
            if val.eq("+") || val.eq("*") {
                match val {
                    "+" => { operators.push(Operator::ADD); }
                    "*" => { operators.push(Operator::MULT); }
                    _ => unreachable!()
                }
            } else if val != "" {
                number_row.push(val.parse::<usize>().unwrap());
            }
        }
        if number_row.len() > 0 {
            number_grid.push(number_row);
        }
    }

    let mut result = 0;
    for (col, operator) in operators.iter().enumerate() {
        let initial_num = match operator {
            Operator::ADD => 0,
            Operator::MULT => 1,
        };
        let inner_result = number_grid.iter().map(|row| {
            row[col]
        }).fold(initial_num, |acc, num| {
            match operator {
                Operator::ADD => acc + num,
                Operator::MULT => acc * num,
            }
        });

        result += inner_result
    }

    result
}

pub(crate) fn solution2(is_test: bool) -> usize {
    let input = parse_file_from_day(6, is_test);

    let mut numbers = Vec::new();
    let mut operators = Vec::new();
    for (idx, line) in input.iter().enumerate() {
        if idx == input.len() - 1 {
            for val in line.split(' ') {
                match val {
                    "+" => { operators.push(Operator::ADD); }
                    "*" => { operators.push(Operator::MULT); }
                    _ => continue
                }
            }
        } else if idx == 0 {
            for val in line.chars() {
                numbers.push(val.to_string());
            }
        } else {
            for (idx, val) in line.chars().enumerate() {
                if val == ' ' {
                    continue
                }
                numbers[idx].push_str(&val.to_string());
            }
        }
    }

    let mut result = 0;

    let mut operator = operators.pop().unwrap();
    // for operator in operators.iter().rev() {
    let mut inner_result = match operator {
        Operator::ADD => 0,
        Operator::MULT => 1,
    };
    while numbers.len() > 0 {
        let num_or_space = numbers.pop().unwrap();
        if num_or_space.trim().eq("") {
            // Add up our current number and reset the maths for the next operator
            result += inner_result;
            operator = operators.pop().unwrap();
            inner_result = match operator {
                Operator::ADD => 0,
                Operator::MULT => 1,
            };
        } else {
            let num = num_or_space.trim().parse::<usize>().unwrap();
            inner_result = match operator {
                Operator::ADD => inner_result + num,
                Operator::MULT => inner_result * num,
            };
        }
    }
    // Add final result
    result += inner_result;

    result
}