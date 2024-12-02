use std::{fs, env};

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut rows = Vec::new();
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let row: Vec<u32> = trimmed.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        rows.push(row);
    }
    rows
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
            for row in &rows {
                println!("{:?}", row);
            }
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
}
