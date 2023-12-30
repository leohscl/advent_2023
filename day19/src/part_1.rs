use std::{collections::HashMap, str::FromStr};

#[derive(PartialEq, Eq, Clone, Debug)]
enum Outcome {
    Send(String),
    Accept,
    Reject,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
enum Category {
    XCool,
    Musical,
    Aerodynamic,
    Shiny,
}

use Category::*;
use Outcome::*;
#[derive(PartialEq, Eq, Clone, Debug)]
enum Condition {
    Less(u64),
    More(u64),
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Step {
    category: Option<Category>,
    condition: Option<Condition>,
    outcome: Outcome,
}

impl FromStr for Step {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter_elt = s.split(':');
        let first_part = iter_elt.next().unwrap();
        let mut iter_chars_first_part = first_part.chars();
        let first_element = iter_chars_first_part.next().unwrap();
        let mut opt_cat = match first_element {
            'a' => Some(Aerodynamic),
            'x' => Some(XCool),
            's' => Some(Shiny),
            'm' => Some(Musical),
            _ => None,
        };
        dbg!(&first_part);
        let opt_condition_char = iter_chars_first_part.next();
        if let Some(condition_char) = opt_condition_char {
            if condition_char != '<' && condition_char != '>' {
                opt_cat = None;
            }
        }
        if let Some(category) = opt_cat {
            let condition_char = opt_condition_char.unwrap();
            let condition = {
                let num = iter_chars_first_part
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
                match condition_char {
                    '<' => Condition::Less(num),
                    '>' => Condition::More(num),
                    _ => panic!(),
                }
            };
            let last_elt = iter_elt.next().unwrap();
            let outcome = match last_elt {
                "A" => Accept,
                "R" => Reject,
                _ => Send(last_elt.to_string()),
            };
            Ok(Step {
                category: Some(category),
                condition: Some(condition),
                outcome,
            })
        } else {
            let outcome = match first_part {
                "A" => Accept,
                "R" => Reject,
                _ => Send(first_part.to_string()),
            };
            Ok(Step {
                category: None,
                condition: None,
                outcome,
            })
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Workflow {
    steps: Vec<Step>,
}

impl FromStr for Workflow {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let steps: Vec<_> = s
            .split(',')
            .map(|step_s| Step::from_str(step_s).unwrap())
            .collect();
        Ok(Workflow { steps })
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Machinery {
    workflows: HashMap<String, Workflow>,
}

impl FromStr for Machinery {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut workflows = HashMap::new();
        s.split('\n').for_each(|line| {
            let mut iter_name_and_workflow = line.split(['{', '}']);
            let name = iter_name_and_workflow.next().unwrap().to_string();
            let workflow = Workflow::from_str(iter_name_and_workflow.next().unwrap()).unwrap();
            workflows.insert(name, workflow);
        });
        Ok(Machinery { workflows })
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Part {
    rating: HashMap<Category, u64>,
}

impl FromStr for Part {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iter_params = s.split([',', '=', '}']);
        let params: Vec<_> = iter_params
            .skip(1)
            .step_by(2)
            .take(4)
            .inspect(|s| {
                dbg!(&s);
            })
            .map(|num_s| num_s.parse::<u64>().unwrap())
            .collect();
        let mut rating = HashMap::new();
        rating.insert(XCool, params[0]);
        rating.insert(Musical, params[1]);
        rating.insert(Aerodynamic, params[2]);
        rating.insert(Shiny, params[3]);
        Ok(Part { rating })
    }
}

impl Part {
    fn score(&self) -> u64 {
        self.rating.values().sum()
    }
}

impl Machinery {
    fn process_part(&self, part: Part, workflow_s: &str) -> bool {
        let workflow = &self.workflows[workflow_s];
        let outcome = workflow
            .steps
            .iter()
            .find_map(|step| {
                if let Some(category) = step.category.clone() {
                    let condition = step.condition.clone().unwrap();
                    let num_cmp = part.rating[&category];
                    match condition {
                        Condition::More(x) => {
                            if num_cmp > x {
                                Some(step.outcome.clone())
                            } else {
                                None
                            }
                        }
                        Condition::Less(x) => {
                            if num_cmp < x {
                                Some(step.outcome.clone())
                            } else {
                                None
                            }
                        }
                    }
                } else {
                    Some(step.outcome.clone())
                }
            })
            .unwrap();
        match outcome {
            Accept => true,
            Reject => false,
            Send(s) => self.process_part(part, &s.to_string()),
        }
    }
}

fn main() {
    // let input = include_str!("../input_test.txt");
    let input = include_str!("../input.txt");
    let mut iter_para = input.split("\n\n");
    let machinery = Machinery::from_str(iter_para.next().unwrap()).unwrap();
    let sum_part: u64 = iter_para
        .next()
        .unwrap()
        .split_terminator('\n')
        .map(|part_str| Part::from_str(part_str).unwrap())
        .map(|part| {
            let score = part.score();
            if machinery.process_part(part, "in") {
                score
            } else {
                0
            }
        })
        .sum();
    dbg!(sum_part);
}
