use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum PartCateg {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum CompType {
    LessThan,
    GreaterThan,
}

#[derive(Debug)]
enum WorkflowRule {
    ConditionRule(PartCateg, CompType, i64, String),
    FallThruRule(String),
}

fn main() {
    let (workflows, parts) = read_input();
    part1(&workflows, &parts);
}

fn read_input() -> (
    HashMap<String, Vec<WorkflowRule>>,
    Vec<HashMap<PartCateg, i64>>,
) {
    let contents = fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-19\\input.txt")
        .expect("Error reading input file");

    let mut workflows = HashMap::new();
    for line in contents.lines().take_while(|&l| !l.is_empty()) {
        let mut split_line = line.trim_matches('}').split("{");
        let name = split_line.next().expect("Error getting workflow name");
        let mut workflow_rules = Vec::new();
        for rule in split_line.next().unwrap().split(",") {
            let mut split_rule = rule.split(":");
            let part1 = split_rule
                .next()
                .expect("Error getting first part of rule split");
            let part2 = split_rule.next();
            if let Some(unwrapped2) = part2 {
                let id = match &part1[..1] {
                    "x" => PartCateg::X,
                    "m" => PartCateg::M,
                    "a" => PartCateg::A,
                    "s" => PartCateg::S,
                    _ => panic!("Unexpected id"),
                };
                let cp = match &part1[1..2] {
                    ">" => CompType::GreaterThan,
                    "<" => CompType::LessThan,
                    _ => panic!("Unexpected comparator"),
                };
                let val = part1[2..].parse::<i64>().expect("Error parsing value");
                workflow_rules.push(WorkflowRule::ConditionRule(
                    id,
                    cp,
                    val,
                    unwrapped2.to_owned(),
                ));
            } else {
                workflow_rules.push(WorkflowRule::FallThruRule(part1.to_owned()));
            }
        }
        workflows.insert(name.to_owned(), workflow_rules);
    }

    let mut parts = Vec::new();
    for line in contents
        .lines()
        .skip_while(|&l| !l.is_empty())
        .filter(|&l| !l.is_empty())
    {
        let mut attr = HashMap::new();
        for line in line.trim_matches('}').trim_matches('{').split(",") {
            let id = match &line[..1] {
                "x" => PartCateg::X,
                "m" => PartCateg::M,
                "a" => PartCateg::A,
                "s" => PartCateg::S,
                _ => panic!("Unexpected id"),
            };
            let val = line[2..].parse::<i64>().expect("Error parsing part value");
            attr.insert(id, val);
        }
        parts.push(attr);
    }

    (workflows, parts)
}

fn part1(workflows: &HashMap<String, Vec<WorkflowRule>>, parts: &Vec<HashMap<PartCateg, i64>>) {
    let mut total_value = 0;
    for part in parts {
        let mut next_workflow = &String::from("in");
        while next_workflow != "R" && next_workflow != "A" {
            if let Some(workflow) = workflows.get(next_workflow) {
                for rule in workflow {
                    match rule {
                        WorkflowRule::ConditionRule(
                            part_category,
                            comparison,
                            val,
                            val_if_true,
                        ) => {
                            let part_val = part
                                .get(&part_category)
                                .expect("Error getting part category");
                            let rule_result = match comparison {
                                CompType::GreaterThan => part_val > val,
                                CompType::LessThan => part_val < val,
                            };
                            if rule_result {
                                next_workflow = val_if_true;
                                break;
                            }
                        }
                        WorkflowRule::FallThruRule(val_automatically) => {
                            next_workflow = val_automatically;
                        }
                    }
                }
            }
        }

        if next_workflow == "A" {
            let mut part_value = 0;
            for (_, cat_val) in part.iter() {
                part_value += cat_val;
            }
            total_value += part_value;
        }
    }
    println!("Part 1: {}", total_value);
}
