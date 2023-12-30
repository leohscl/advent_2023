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
    // LessEqual(u64),
    // MoreEqual(u64),
}

impl Condition {
    fn mirror(&self) -> Condition {
        match self {
            &Self::More(x) => Self::Less(x + 1),
            &Self::Less(x) => Self::More(x - 1),
        }
    }
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
    // fn process_part(&self, part: Part, workflow_s: &str) -> bool {
    //     let workflow = &self.workflows[workflow_s];
    //     let outcome = workflow
    //         .steps
    //         .iter()
    //         .find_map(|step| {
    //             if let Some(category) = step.category.clone() {
    //                 let condition = step.condition.clone().unwrap();
    //                 let num_cmp = part.rating[&category];
    //                 match condition {
    //                     Condition::More(x) => {
    //                         if num_cmp > x {
    //                             Some(step.outcome.clone())
    //                         } else {
    //                             None
    //                         }
    //                     }
    //                     Condition::Less(x) => {
    //                         if num_cmp < x {
    //                             Some(step.outcome.clone())
    //                         } else {
    //                             None
    //                         }
    //                     }
    //                 }
    //             } else {
    //                 Some(step.outcome.clone())
    //             }
    //         })
    //         .unwrap();
    //     match outcome {
    //         Accept => true,
    //         Reject => false,
    //         Send(s) => self.process_part(part, &s.to_string()),
    //     }
    // }

    fn build_paths(&self, current_node: &str, current_path: Path) -> Vec<Path> {
        let mut all_paths = Vec::new();
        let workflow = self.workflows[current_node].clone();
        let mut steps_iter = workflow.steps.iter();
        let mut path_continue = current_path.clone();
        loop {
            let step = steps_iter.next().unwrap();
            let mut path_jump = path_continue.clone();
            if let Some(condition) = step.condition.clone() {
                let category = step.category.clone().unwrap();
                path_jump.push((condition.clone(), category.clone()));
                let paths_out = match step.outcome.clone() {
                    Send(s) => self.build_paths(&s, path_jump),
                    Accept => vec![path_jump],
                    Reject => Vec::new(),
                };
                all_paths.extend(paths_out.into_iter());
                path_continue.push((condition.mirror(), category));
            } else {
                let paths_out = match step.outcome.clone() {
                    Send(s) => self.build_paths(&s, path_jump),
                    Accept => vec![path_jump],
                    Reject => Vec::new(),
                };
                all_paths.extend(paths_out.into_iter());
                break;
            }
        }
        all_paths
    }
}

fn score_path(path: &Path) -> u64 {
    [XCool, Musical, Aerodynamic, Shiny]
        .iter()
        .map(|cat| {
            path.iter()
                .filter(|(_, c)| c == cat)
                .fold(Some((1, 4000)), |acc, (condition, _)| {
                    if let Some((min, max)) = acc {
                        match condition {
                            Condition::Less(num) => {
                                let new_max = *num - 1;
                                if min > new_max {
                                    None
                                } else {
                                    Some((min, new_max))
                                }
                            }
                            Condition::More(num) => {
                                let new_min = *num + 1;
                                if new_min > max {
                                    None
                                } else {
                                    Some((new_min, max))
                                }
                            }
                        }
                    } else {
                        None
                    }
                })
                .map(|(min, max)| max - min + 1)
                .unwrap_or(0)
        })
        .product()
}

type Path = Vec<(Condition, Category)>;

fn main() {
    // let input = include_str!("../input_test.txt");
    let input = include_str!("../input.txt");
    let mut iter_para = input.split("\n\n");
    let machinery = Machinery::from_str(iter_para.next().unwrap()).unwrap();
    let all_paths = machinery.build_paths("in", Vec::new());
    let sum_scores: u64 = all_paths.iter().map(|path| score_path(path)).sum();
    dbg!(sum_scores);
    // dbg!(&all_paths);
}
