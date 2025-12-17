use std::ops::RangeInclusive;
use crate::parser::parse_file_from_day;

pub(crate) fn solution1(is_test: bool) -> usize {
    let input = parse_file_from_day(5, is_test);

    let mut fresh_ranges = Vec::new();
    let mut ingredients: Vec<usize> = Vec::new();

    let mut hit_blank = false;
    for line in input {
        if hit_blank {
            ingredients.push(line.parse::<usize>().unwrap());
        } else if line == "" {
            hit_blank = true;
        } else {
            let mut range = line.split("-");
            let range_start = range.next().unwrap().parse::<usize>().unwrap();
            let range_end = range.next().unwrap().parse::<usize>().unwrap();
            fresh_ranges.push(RangeInclusive::new(range_start, range_end));
        }
    }

    let mut total_fresh = 0;
    for ingredient in &ingredients {
        for range in &fresh_ranges {
            if range.contains(ingredient) {
                total_fresh += 1;
                break;
            }
        }
    }

    total_fresh
}

pub(crate) fn solution2(is_test: bool) -> usize {
    let input = parse_file_from_day(5, is_test);

    let mut fresh_ranges = Vec::new();

    for line in input {
        if line == "" {
            break;
        } else {
            let mut range = line.split("-");
            let range_start = range.next().unwrap().parse::<usize>().unwrap();
            let range_end = range.next().unwrap().parse::<usize>().unwrap();
            fresh_ranges.push((range_start, range_end));
        }
    }

    fresh_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut current_range = fresh_ranges[0];
    let mut combined_ranges = Vec::new();
    for (range_start, range_end) in fresh_ranges[1..].to_vec() {
        if range_start < current_range.1 && range_end < current_range.1 {
            // range is completely encompassed by the current range we're trying to expand, so skip
            continue;
        } else if range_start <= current_range.1 && range_end > current_range.1 {
            // Range starts within the current range, but ends outside of it - expand range to
            // include all values.
            current_range = (current_range.0, range_end);
        } else if range_start > current_range.1 {
            // We're now at the start of a new range segment, so start over and add old
            // current_range to our new list of combined ranges
            combined_ranges.push((current_range.0, current_range.1));
            current_range = (range_start, range_end);
        }
    }
    // Add the last combined range
    combined_ranges.push((current_range.0, current_range.1));

    let mut total_fresh = 0;
    for (x, y) in combined_ranges {
        total_fresh += (y - x) + 1
    }

    total_fresh
}