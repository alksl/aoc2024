use std::{env, fs, process};

type PageOrdering = Vec<i32>;

#[derive(Debug, PartialEq, Clone)]
struct Rule {
    before: i32,
    after: i32,
}

impl Rule {
    const fn new(before: i32, after: i32) -> Rule {
        Rule { before, after }
    }

    fn parse(line: &str) -> Result<Rule, &str> {
        let mut parts = line.trim().split('|');
        let before = parts.next().map_or(Err("Missing before"), |part| {
            part.trim().parse().map_err(|_| "Invalid before")
        })?;
        let after = parts.next().map_or(Err("Missing after"), |part| {
            part.trim().parse().map_err(|_| "Invalid after")
        })?;
        Ok(Rule::new(before, after))
    }

    fn applicable(&self, ordering: &[i32]) -> bool {
        let includes_before = ordering.iter().any(|&page| page == self.before);
        let includes_after = ordering.iter().any(|&page| page == self.after);
        includes_before && includes_after
    }
}

fn parse_input(input: &str) -> Result<(Vec<Rule>, Vec<PageOrdering>), &str> {
    let mut rules: Vec<Rule> = Vec::new();
    let mut page_orderings: Vec<PageOrdering> = Vec::new();
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if !line.is_empty() {
            rules.push(Rule::parse(line)?);
            break;
        }
    }

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        rules.push(Rule::parse(line)?);
    }

    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            let ordering = trimmed
                .split(',')
                .map(|part| part.parse().map_err(|_| "Invalid page number"))
                .collect::<Result<Vec<i32>, &str>>()?;
            page_orderings.push(ordering);
        }
    }

    Ok((rules, page_orderings))
}

fn applicable_rules(ordering: &[i32], rules: &[Rule]) -> Vec<Rule> {
    rules
        .iter()
        .filter(|rule| rule.applicable(ordering))
        .cloned()
        .collect()
}

fn correct(ordering: &[i32], rules: &[Rule]) -> Vec<i32> {
    let mut corrected = ordering.to_vec();
    corrected.sort_by(|a, b| {
        let sort_rule = rules.iter().find(|rule| rule.before == *a && rule.after == *b);
        match sort_rule {
            Some(_) => std::cmp::Ordering::Less,
            None => std::cmp::Ordering::Equal,
        }
    });
    corrected
}

fn in_order(ordering: &[i32], rules: &[Rule]) -> bool {
    let applicable_rules = applicable_rules(ordering, rules);
    for rule in applicable_rules {
        let before_index = ordering
            .iter()
            .position(|&page| page == rule.before)
            .unwrap();
        let after_index = ordering
            .iter()
            .position(|&page| page == rule.after)
            .unwrap();
        if before_index > after_index {
            return false;
        }
    }
    true
}

fn middle(ordering: &[i32]) -> i32 {
    let middle_index = ordering.len() / 2;
    ordering[middle_index]
}

fn sum_of_middle_valid_numbers(page_orderings: &[PageOrdering], rules: &[Rule]) -> i32 {
    page_orderings
        .iter()
        .filter(|ordering| in_order(ordering, rules))
        .map(|ordering| middle(&ordering))
        .sum()
}

fn sum_of_middle_corrected_numbers(page_orderings: &[PageOrdering], rules: &[Rule]) -> i32 {
    page_orderings
        .iter()
        .filter(|ordering| !in_order(ordering, rules))
        .map(|ordering| correct(ordering, &applicable_rules(ordering, rules)))
        .map(|ordering| middle(&ordering))
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        process::exit(1);
    }

    match fs::read_to_string(&args[1]) {
        Ok(input) => match parse_input(&input) {
            Ok((rules, page_orderings)) => {
                println!(
                    "Sum middle valid orderings {}",
                    sum_of_middle_valid_numbers(&page_orderings, &rules)
                );
                println!(
                    "Sum middle corrected orderings {}",
                    sum_of_middle_corrected_numbers(&page_orderings, &rules)
                );
            }
            Err(e) => {
                eprintln!("Error parsing input: {}", e);
                process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static RULES: &[Rule] = &[
        Rule::new(47, 53),
        Rule::new(97, 13),
        Rule::new(97, 61),
        Rule::new(97, 47),
        Rule::new(75, 29),
        Rule::new(61, 13),
        Rule::new(75, 53),
        Rule::new(29, 13),
        Rule::new(97, 29),
        Rule::new(53, 29),
        Rule::new(61, 53),
        Rule::new(97, 53),
        Rule::new(61, 29),
        Rule::new(47, 13),
        Rule::new(75, 47),
        Rule::new(97, 75),
        Rule::new(47, 61),
        Rule::new(75, 61),
        Rule::new(47, 29),
        Rule::new(75, 13),
        Rule::new(53, 13),
    ];

    #[test]
    fn test_parse_input() {
        let input = r#"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
        "#;
        let (rules, page_orderings) = parse_input(input).expect("Failed to parse input");
        assert_eq!(rules, RULES);
        assert_eq!(
            page_orderings,
            vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47],
            ],
        );
    }

    #[test]
    fn test_sum_middle_valid_numbers() {
        let page_orderings = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        assert_eq!(sum_of_middle_valid_numbers(&page_orderings, RULES), 143);
    }

    #[test]
    fn test_sum_middle_corrected_numbers() {
        let page_orderings = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        assert_eq!(sum_of_middle_corrected_numbers(&page_orderings, RULES), 123);
    }

    #[test]
    fn test_parse_rule() {
        let rule = Rule::parse("47|53").expect("Failed to parse rule");
        assert_eq!(rule.before, 47);
        assert_eq!(rule.after, 53);
    }

    #[test]
    fn test_applicable_rule() {
        let ordering = vec![75, 47, 61, 53, 29];
        let applicable_rule = Rule::new(47, 53);
        let missing_before = Rule::new(13, 47);
        let missing_after = Rule::new(47, 13);
        assert_eq!(applicable_rule.applicable(&ordering), true);
        assert_eq!(missing_before.applicable(&ordering), false);
        assert_eq!(missing_after.applicable(&ordering), false);
    }

    #[test]
    fn test_valid_orderngs() {
        assert_eq!(in_order(&vec![75, 47, 61, 53, 29], RULES), true);
        assert_eq!(in_order(&vec![97, 61, 53, 29, 13], RULES), true);
        assert_eq!(in_order(&vec![75, 29, 13], RULES), true);
    }

    #[test]
    fn test_invalid_orderings() {
        assert_eq!(in_order(&vec![75, 97, 47, 61, 53], RULES), false);
        assert_eq!(in_order(&vec![61, 13, 29], RULES), false);
        assert_eq!(in_order(&vec![97, 13, 75, 29, 47], RULES), false);
    }

    #[test]
    fn test_correct() {
        let vec1 = vec![75,97,47,61,53];
        let vec2 = vec![61,13,29];
        let vec3 = vec![97,13,75,29,47];
        assert_eq!(
            correct(&vec1, &applicable_rules(&vec1, &RULES)),
            vec![97, 75, 47, 61, 53]
        );
        assert_eq!(
            correct(&vec2, &applicable_rules(&vec2, &RULES)),
            vec![61, 29, 13]
        );
        assert_eq!(
            correct(&vec3, &applicable_rules(&vec3, &RULES)),
            vec![97, 75, 47, 29, 13],
        );
    }
}
