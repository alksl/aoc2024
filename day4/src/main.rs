use std::{env, fs, process};

type Grid = Vec<Vec<char>>;
type Row = Vec<char>;

fn parse_grid(input: &str, size: usize) -> Result<Grid, &str> {
    let mut grid: Grid = Vec::new();
    let lines: Vec<&str> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();
    assert!(lines.len() == size, "Invalid grid size");
    for line in lines {
        let row: Vec<char> = line.chars().collect();
        assert!(row.len() == size, "Invalid row size");
        grid.push(row);
    }
    Ok(grid)
}

fn conv_match_cannonical(conv: &Grid) -> bool {
    assert!(conv.len() == 3, "Invalid convolution size");
    conv[0][0] == 'M' &&
        conv[2][0] == 'M' &&
        conv[1][1] == 'A' &&
        conv[0][2] == 'S' &&
        conv[2][2] == 'S'
}

fn conv_match_transposed(conv: &Grid) -> bool {
    assert!(conv.len() == 3, "Invalid convolution size");
    conv[0][0] == 'M' &&
        conv[0][2] == 'M' &&
        conv[1][1] == 'A' &&
        conv[2][0] == 'S' &&
        conv[2][2] == 'S'
}


fn conv_match_flipped(conv: &Grid) -> bool {
    assert!(conv.len() == 3, "Invalid convolution size");
    conv[0][0] == 'S' &&
        conv[2][0] == 'S' &&
        conv[1][1] == 'A' &&
        conv[0][2] == 'M' &&
        conv[2][2] == 'M'
}
fn conv_match_transposed_flipped(conv: &Grid) -> bool {
    assert!(conv.len() == 3, "Invalid convolution size");
    conv[0][0] == 'S' &&
        conv[0][2] == 'S' &&
        conv[1][1] == 'A' &&
        conv[2][0] == 'M' &&
        conv[2][2] == 'M'
}

fn conv_match(conv: &Grid) -> bool {
    conv_match_cannonical(&conv) ||
        conv_match_transposed(&conv) ||
        conv_match_flipped(&conv) ||
        conv_match_transposed_flipped(&conv)
}

fn conv(grid: &Grid, window_size: usize) -> Vec<Grid> {
    let mut convolutions: Vec<Grid> = Vec::new();
    for i in 0..grid.len() - window_size + 1 {
        for j in 0..grid.len() - window_size + 1 {
            let mut conv: Grid = Vec::new();
            for k in 0..window_size {
                let mut row: Row = Vec::new();
                for l in 0..window_size {
                    row.push(grid[i + k][j + l]);
                }
                conv.push(row);
            }
            convolutions.push(conv);
        }
    }
    convolutions
}

fn num_xmas(grid: &Grid) -> usize {
    conv(grid, 3)
        .iter()
        .map(|conv| conv_match(conv) as usize)
        .sum::<usize>()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide a file path");
        process::exit(1);
    }

    let file_path = &args[1];
    match fs::read_to_string(file_path) {
        Ok(content) => {
            let grid = parse_grid(&content, 140).expect("Failed to parse grid");
            println!("X-MAS Count: {:?}", num_xmas(&grid));
        }
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
    fn test_xmas_grid() {
        let input = r#"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
        "#;
        let grid = parse_grid(input, 10).expect("Failed to parse");
        assert_eq!(num_xmas(&grid), 9);
    }

    #[test]
    fn test_parse_grid() {
        let input = r#"
        123
        456
        789
        "#;
        let grid = parse_grid(input, 3).expect("Failed to parse");
        assert_eq!(
            grid,
            vec![
                vec!['1', '2', '3'],
                vec!['4', '5', '6'],
                vec!['7', '8', '9'],
            ]
        );
    }

    #[test]
    fn test_conv_match() {
        let conv = vec![
            vec!['M', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'S'],
        ];
        assert_eq!(conv_match(&conv), true);
    }

    #[test]
    fn test_conv_match_transposed() {
        let conv = vec![
            vec!['M', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'S'],
        ];
        assert_eq!(conv_match(&conv), true);
    }

    #[test]
    fn test_conv_match_flipped() {
        let conv = vec![
            vec!['S', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'M'],
        ];
        assert_eq!(conv_match(&conv), true);
    }

    #[test]
    fn test_conv_match_transposed_flipped() {
        let conv = vec![
            vec!['S', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'M'],
        ];
        assert_eq!(conv_match(&conv), true);
    }


    #[test]
    fn test_conv() {
        let grid = vec![
            vec!['0', '1', '2'],
            vec!['3', '4', '5'],
            vec!['6', '8', '9'],
        ];
        assert_eq!(
            conv(&grid, 2),
            vec![
                vec![
                    vec!['0', '1'],
                    vec!['3', '4'],
                ],
                vec![
                    vec!['1', '2'],
                    vec!['4', '5'],
                ],
                vec![
                    vec!['3', '4'],
                    vec!['6', '8'],
                ],
                vec![
                    vec!['4', '5'],
                    vec!['8', '9'],
                ],
            ],
        );
    }
}
