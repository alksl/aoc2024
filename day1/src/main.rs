use std::{env, fs, process};

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let mut parts = trimmed.split_whitespace();
        let col1_val = parts.next().unwrap();
        let col2_val = parts.next().unwrap();
        col1.push(col1_val.parse().unwrap());
        col2.push(col2_val.parse().unwrap());

    }
    (col1, col2)
}

fn order_by_smallest(col: &Vec<i32>) -> Vec<i32> {
    let mut ordered = col.clone();
    ordered.sort();
    ordered
}

fn calculate_differences(col1: &Vec<i32>, col2: &Vec<i32>) -> Vec<i32> {
    let col1_ordered = order_by_smallest(col1);
    let col2_ordered = order_by_smallest(col2);
    let mut difference = Vec::new();

    assert_eq!(col1_ordered.len(), col2_ordered.len());
    for i in 0..col1.len() {
        difference.push((col1_ordered[i] - col2_ordered[i]).abs());
    }
    difference
}

fn calculate_distance(input: &str) -> i32 {
    let (col1, col2) = parse_input(input);
    let diffs = calculate_differences(&col1, &col2);
    diffs.iter().sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprint!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];
    match fs::read_to_string(file_path) {
        Ok(content) => {
            let distance = calculate_distance(&content);
            println!("Distance: {}", distance);
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_read_example() {
        let input = r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        "#;
        let (col1, col2) = parse_input(input);
        assert_eq!(col1, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(col2, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_order_by_smallest() {
        let unordered = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let ordered = order_by_smallest(&unordered);
        assert_eq!(ordered, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_calculate_differences() {
        let col1 = vec![3, 4, 2, 1, 3, 3];
        let col2 = vec![4, 3, 5, 3, 9, 3];
        let difference = calculate_differences(&col1, &col2);
        assert_eq!(difference, vec![2, 1, 0, 1, 2, 5]);
    }

    #[test]
    fn test_calculate_distance() {
        let input = r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        "#;
        assert_eq!(11, calculate_distance(input))
    }
}
