use std::{env, fs, process};

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let mut parts = trimmed.split_whitespace();
        let left_val = parts.next().unwrap();
        let right_val = parts.next().unwrap();
        left.push(left_val.parse().unwrap());
        right.push(right_val.parse().unwrap());

    }
    (left, right)
}

fn order_by_smallest(col: &Vec<i32>) -> Vec<i32> {
    let mut ordered = col.clone();
    ordered.sort();
    ordered
}

fn calculate_differences(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    let left_ordered = order_by_smallest(left);
    let right_ordered = order_by_smallest(right);
    let mut difference = Vec::new();

    assert_eq!(left_ordered.len(), right_ordered.len());
    for i in 0..left.len() {
        difference.push((left_ordered[i] - right_ordered[i]).abs());
    }
    difference
}

fn calculate_distance(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let diffs = calculate_differences(&left, &right);
    diffs.iter().sum()
}

fn calculate_similarities(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    let mut similarities = Vec::new();
    for i in 0..left.len() {
        let occurrences = right.iter().filter(|&x| *x == left[i]).count() as i32;
        let similarity = occurrences * left[i];
        similarities.push(similarity);
    }
    similarities
}

fn calculate_similarity_score(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let similarities = calculate_similarities(left, right);
    similarities.iter().sum()
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
            let (left, right) = parse_input(&content);
            let distance = calculate_distance(&left, &right);
            let similarity = calculate_similarity_score(&left, &right);
            println!("Distance: {}", distance);
            println!("Similarity: {}", similarity);
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
        let (left, right) = parse_input(input);
        assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_order_by_smallest() {
        let unordered = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let ordered = order_by_smallest(&unordered);
        assert_eq!(ordered, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_calculate_differences() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];
        let difference = calculate_differences(&left, &right);
        assert_eq!(difference, vec![2, 1, 0, 1, 2, 5]);
    }

    #[test]
    fn test_calculate_similarity_score() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];
        let score = calculate_similarity_score(&left, &right);
        assert_eq!(score, 31);
    }

    #[test]
    fn test_calculate_similarities() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];
        let similarities = calculate_similarities(&left, &right);
        assert_eq!(similarities, vec![9, 4, 0, 0, 9, 9]);
    }

    #[test]
    fn test_calculate_distance() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(11, calculate_distance(&left, &right));
    }
}
