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

fn transpose(grid: &Grid) -> Grid {
    let mut transposed: Grid = Vec::new();
    for i in 0..grid.len() {
        let mut row: Row = Vec::new();
        for j in 0..grid.len() {
            row.push(grid[j][i]);
        }
        transposed.push(row);
    }
    transposed
}

fn flip(grid: &Grid) -> Grid {
    let mut flipped: Grid = Vec::new();
    for row in grid.iter() {
        let mut new_row = row.clone();
        new_row.reverse();
        flipped.push(new_row);
    }
    flipped
}

fn diagonal(grid: &Grid) -> Row {
    let mut diagonal: Row = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        if i < row.len() {
            diagonal.push(row[i]);
        }
    }
    diagonal
}

fn shrink(grid: &Grid, n: usize, m: usize) -> Grid {
    let mut shrunk: Grid = Vec::new();
    for i in n..grid.len() {
        let mut row: Row = Vec::new();
        for j in m..grid.len() {
            row.push(grid[i][j]);
        }
        shrunk.push(row);
    }
    shrunk
}

fn diagonals(grid: &Grid) -> Vec<Row> {
    let mut diagonals: Vec<Row> = Vec::new();
    diagonals.push(diagonal(&grid));
    for i in 1..grid.len() {
        diagonals.push(diagonal(&shrink(&grid, i, 0)));
    }
    for i in 1..grid.len() {
        diagonals.push(diagonal(&shrink(&grid, 0, i)));
    }
    diagonals
}

fn generate_grid_variants(grid: &Grid) -> Vec<Grid> {
    let mut variants: Vec<Grid> = Vec::new();
    variants.push(grid.clone());
    variants.push(transpose(&grid));
    variants.push(flip(&grid));
    variants.push(flip(&transpose(&grid)));

    variants
}

fn num_matches_in_row(row: &Row) -> usize {
    row.windows(4)
        .filter(|window| *window == ['X', 'M', 'A', 'S'])
        .count()
}

fn num_straight_lines(grid: &Grid) -> usize {
    let grid_variants = generate_grid_variants(&grid);
    grid_variants
        .iter()
        .map(|grid| {
            grid.iter()
                .map(|row| num_matches_in_row(row))
                .sum::<usize>()
        })
        .sum()
}

fn reverse(row: &Row) -> Row {
    row.iter().rev().cloned().collect()
}

fn num_diagonals(grid: &Grid) -> usize {
    diagonals(&grid)
        .iter()
        .map(|diagonal| num_matches_in_row(diagonal) + num_matches_in_row(&reverse(diagonal)))
        .sum()
}

fn num_xmas(grid: &Grid) -> usize {
    num_straight_lines(&grid) + num_diagonals(&grid) + num_diagonals(&flip(&grid))
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
            println!("XMAS Count: {:?}", num_xmas(&grid));
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
        assert_eq!(num_xmas(&grid), 18);
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
    fn test_transpose() {
        let grid = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];
        let transposed = transpose(&grid);
        assert_eq!(
            transposed,
            vec![
                vec!['1', '4', '7'],
                vec!['2', '5', '8'],
                vec!['3', '6', '9'],
            ]
        );
    }

    #[test]
    fn test_diagonal() {
        let grid = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];
        assert_eq!(diagonal(&grid), vec!['1', '5', '9']);
    }

    #[test]
    fn test_shrink() {
        let grid = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];
        assert_eq!(shrink(&grid, 1, 1), vec![vec!['5', '6'], vec!['8', '9'],],);
        assert_eq!(
            shrink(&grid, 0, 1),
            vec![vec!['2', '3'], vec!['5', '6'], vec!['8', '9'],],
        );
        assert_eq!(
            shrink(&grid, 1, 0),
            vec![vec!['4', '5', '6'], vec!['7', '8', '9'],],
        );
    }

    #[test]
    fn test_diagonals() {
        let grid = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];
        let diagonals = diagonals(&grid);
        assert_eq!(
            diagonals,
            vec![
                vec!['1', '5', '9'],
                vec!['4', '8'],
                vec!['7'],
                vec!['2', '6'],
                vec!['3'],
            ],
        );
    }

    #[test]
    fn test_multiple_row_matches() {
        let row = vec![
            '.', 'X', 'M', 'A', 'S', 'X', 'M', 'A', 'S', '.', 'X', 'M', 'A', 'S', '.',
        ];
        assert_eq!(num_matches_in_row(&row), 3);
    }

    #[test]
    fn test_match_horizontal() {
        let grid = vec![
            vec!['.', 'X', 'M', 'A', 'S', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['.', 'X', 'M', 'A', 'S', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['.', 'X', 'M', 'A', 'S', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(num_xmas(&grid), 3);
    }

    #[test]
    fn test_match_vertical() {
        let grid = vec![
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['X', '.', 'X', '.', 'X', '.'],
            vec!['M', '.', 'M', '.', 'M', '.'],
            vec!['A', '.', 'A', '.', 'A', '.'],
            vec!['S', '.', 'S', '.', 'S', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(num_xmas(&grid), 3);
    }

    #[test]
    fn test_match_diagonal() {
        let grid = vec![
            vec!['.', 'X', '.', '.', '.', '.'],
            vec!['.', 'X', 'M', '.', '.', '.'],
            vec!['.', 'X', 'M', 'A', '.', '.'],
            vec!['.', '.', 'M', 'A', 'S', '.'],
            vec!['.', '.', '.', 'A', 'S', '.'],
            vec!['.', '.', '.', '.', 'S', '.'],
        ];
        assert_eq!(num_xmas(&grid), 3);
    }

    #[test]
    fn test_flipped_diagonal() {
        let grid = vec![
            vec!['.', '.', '.', '.', 'S', '.'],
            vec!['.', '.', '.', 'A', 'S', '.'],
            vec!['.', '.', 'M', 'A', 'S', '.'],
            vec!['.', 'X', 'M', 'A', '.', '.'],
            vec!['.', 'X', 'M', '.', '.', '.'],
            vec!['.', 'X', '.', '.', '.', '.'],
        ];
        assert_eq!(num_xmas(&grid), 3);
    }


    #[test]
    fn test_flipped_horizontal() {
        let grid = vec![
            vec!['.', 'S', 'A', 'M', 'X', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['.', 'S', 'A', 'M', 'X', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['.', 'S', 'A', 'M', 'X', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
        ];

        assert_eq!(num_xmas(&grid), 3);
    }

    #[test]
    fn test_flipped_vertical() {
        let grid = vec![
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['S', '.', 'S', '.', 'S', '.'],
            vec!['A', '.', 'A', '.', 'A', '.'],
            vec!['M', '.', 'M', '.', 'M', '.'],
            vec!['X', '.', 'X', '.', 'X', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(num_xmas(&grid), 3);
    }

    #[test]
    fn test_reversed_diagonal() {
        let grid = vec![
            vec!['.', 'S', '.', '.', '.', '.'],
            vec!['.', 'S', 'A', '.', '.', '.'],
            vec!['.', 'S', 'A', 'M', '.', '.'],
            vec!['.', '.', 'A', 'M', 'X', '.'],
            vec!['.', '.', '.', 'M', 'X', '.'],
            vec!['.', '.', '.', '.', 'X', '.'],
        ];
        assert_eq!(num_xmas(&grid), 3);
    }
}
