use std::{fs, env};

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut rows = Vec::new();
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let row: Vec<i32> = trimmed.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        rows.push(row);
    }
    rows
}

fn safe_diffs(diffs: &Vec<i32>) -> bool {
    diffs.iter().all(|&x| x.abs() <= 3)
}

fn safe_row(row: &Vec<i32>) -> bool {
    let diffs: Vec<i32> = row.
        windows(2).
        map(|window| window[0] - window[1]).
        collect();

    let all_inreasing = diffs.iter().all(|&x| x > 0);
    let all_decresing = diffs.iter().all(|&x| x < 0);
    if all_inreasing || all_decresing {
        safe_diffs(&diffs)
    } else {
        false
    }
}

fn num_safe_rows(rows: &Vec<Vec<i32>>) -> i32 {
    rows.iter().filter(|&row| safe_row(row)).count() as i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprint!("Usage: {} <filename>", args[0]);
    }

    let file_path = &args[1];
    match fs::read_to_string(file_path) {
        Ok(content) => {
            let rows = parse_input(&content);
            let num_safe = num_safe_rows(&rows);
            println!("{}", num_safe);
        },
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = r#"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
        "#;

        let rows = parse_input(input);
        assert_eq!(rows, vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ]);
    }

    #[test]
    fn test_num_safe_rows() {
        let rows = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(num_safe_rows(&rows), 2);
    }

    #[test]
    fn test_safe_row() {
        assert_eq!(safe_row(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(safe_row(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(safe_row(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(safe_row(&vec![1, 3, 2, 4, 5]), false);
        assert_eq!(safe_row(&vec![8, 6, 4, 4, 1]), false);
        assert_eq!(safe_row(&vec![1, 3, 6, 7, 9]), true);
    }
}
