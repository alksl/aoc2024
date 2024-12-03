use std::{env, fs};
use std::vec::Vec;

#[derive(PartialEq, Debug)]
enum Expr {
    Do,
    Dont,
    Multiply(Mul),
}

#[derive(PartialEq, Debug)]
struct Mul {
    x: i32,
    y: i32,
}

impl Mul {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn value(&self) -> i32 {
        self.x * self.y
    }

    fn from_str(s: &str) -> Result<Self, &str> {
        let parts = s.split(|c: char| c == '(' || c == ',' || c == ')').collect::<Vec<&str>>();
        if parts.len() != 4 {
            return Result::Err("Invalid parts");
        }
        if parts[0] != "mul" {
            return Result::Err("Invalid function");
        }
        let x = parts[1].parse::<i32>().or_else(|_| Result::Err("Invalid x"))?;
        let y = parts[2].parse::<i32>().or_else(|_| Result::Err("Invalid y"))?;
        Result::Ok(Self::new(x, y))
    }
}

impl std::fmt::Display for Mul {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mul({}, {})", self.x, self.y)
    }
}

fn parse(input: &str) -> Result<Vec<Expr>, &str> {
    let mut state = String::new();
    let mut exps: Vec<Expr> = Vec::new();
    for char in input.chars() {
        match char {
            'm' => {
                if state == "" {
                    state.push(char);
                } 
            },
            'u' => {
                if state == "m" {
                    state.push(char);
                } else {
                    state.clear();
                }
            },
            'd' => {
                if state == "" {
                    state.push(char);
                } 
            },
            'o' => {
                if state == "d" {
                    state.push(char);
                } else {
                    state.clear();
                }
            },
            'n' => {
                if state == "do" {
                    state.push(char);
                } else {
                    state.clear();
                }
            },
            '\'' => {
                if state == "don" {
                    state.push(char);
                } else {
                    state.clear();
                }
            },
            't' => {
                if state == "don'" {
                    state.push(char);
                } else {
                    state.clear();
                }
            },
            'l' => {
                if state == "mu" {
                    state.push(char);
                } else {
                    state.clear();
                }
            },
            '(' => {
                if state == "mul" || state == "do" || state == "don't" {
                    state.push(char);
                } else {
                    state.clear();
                }
            },
            '0'..='9' => {
                if state == "mul(" || state.ends_with(|c: char| c.is_ascii_digit() || c == ',') {
                    state.push(char);
                } else {
                    state.clear()
                }
            },
            ',' => {
                if state.ends_with(|c: char| c.is_ascii_digit()) {
                    state.push(char);
                } else {
                    state.clear()
                }
            },
            ')' => {
                if state.ends_with(|c: char| c.is_ascii_digit()) {
                    state.push(char);
                    let mul = Mul::from_str(&state).or_else(|_| Result::Err("Failed to create Mul"))?;
                    exps.push(Expr::Multiply(mul));
                } else if state == "do(" {
                    exps.push(Expr::Do);   
                } else if state == "don't(" {
                    exps.push(Expr::Dont);
                }
                state.clear();
            },
            _ => { 
                state.clear();
            }
        }
    }
    Ok(exps)
}

fn run_expressions(exps: Vec<Expr>) -> i32 {
    let mut value = 0;
    let mut accumulate = true;
    for exp in exps {
        match exp {
            Expr::Do => accumulate = true,
            Expr::Dont => accumulate = false,
            Expr::Multiply(exp) => {
                if accumulate {
                    value += exp.value();
                }
            }
        }
    }
    value
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <name>", args[0]);
        return;
    }

    match fs::read_to_string(&args[1]) {
        Ok(content) => {
            let exps = parse(&content).expect("Failed to parse");
            let value = run_expressions(exps);
            println!("Value: {}", value);
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let input = r#"
        mul(44,46)
        mul(123,4)
        "#;
        let exps = parse(input).expect("Failed to parse");
        assert_eq!(
            exps, 
            vec![
                Expr::Multiply(Mul::new(44, 46)),
                Expr::Multiply(Mul::new(123, 4)),
            ],
        );
    }

    #[test]
    fn test_mul_from_str() {
        let exp = Mul::from_str("mul(44,46)").expect("Failed to create from str");
        assert_eq!(exp, Mul::new(44, 46));
    }

    #[test]
    fn test_parse_invalid_examples() {
        let input = r#"
         mul(4*
         mul(6,9!
         ?(12,34)
         mul ( 2 , 4 )
        "#;
        let exps = parse(input).expect("Failed to parse");
        assert_eq!(exps.len(), 0);
    }

    #[test]
    fn test_run_corrupted_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let exps = parse(input).expect("Failed to parse");
        let value = run_expressions(exps);
        assert_eq!(value, 161);
    }

    #[test]
    fn test_the_do_and_donts() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let exps = parse(input).expect("Failed to parse");
        let value = run_expressions(exps);
        assert_eq!(value, 48);
    }
}
