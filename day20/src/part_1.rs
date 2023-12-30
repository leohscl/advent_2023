use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Pulse {
    Low,
    High,
}

impl Pulse {
    fn opposite(&self) -> Pulse {
        match self {
            Low => High,
            High => Low,
        }
    }
}

use Pulse::*;

#[derive(Clone, Debug, PartialEq, Eq)]
enum ModuleType {
    FlipFlop(Pulse),
    Conjuction(HashMap<String, Pulse>),
    Broadcaster,
}

use ModuleType::*;

struct System {
    all_modules: HashMap<String, Module>,
    count_low: u32,
    count_high: u32,
}

impl System {
    fn reset_low(&mut self) {
        // dbg!(self.count_low);
        self.count_low = 0;
    }
    fn score(&self) -> u32 {
        self.count_low * self.count_high
    }
    fn send_all_low(&mut self) {
        let all_messages: Vec<_> = self
            .all_modules
            .values()
            .into_iter()
            .flat_map(|module| module.get_all_low().into_iter())
            .collect();
        all_messages.into_iter().for_each(|m| {
            let module_type = self
                .all_modules
                .get(&m.destination)
                .map(|m| m.module_type.clone())
                .unwrap_or(ModuleType::Broadcaster);
            if let ModuleType::Conjuction(_) = module_type {
                self.process_message(m);
            }
        });
    }

    fn run_loop(&mut self) {
        let button_presses = 1000000;
        let mut message_queue = VecDeque::new();
        let base_message = Message {
            source: "".to_string(),
            destination: "broadcaster".to_string(),
            pulse: Pulse::Low,
        };
        for i_press in 0..button_presses {
            dbg!(i_press);
            message_queue.extend(self.process_message(base_message.clone()).into_iter());
            while let Some(message) = message_queue.pop_front() {
                let new_messages = self.process_message(message);
                message_queue.extend(new_messages.into_iter());
            }
        }
    }

    fn process_message(&mut self, message: Message) -> Vec<Message> {
        match message.pulse {
            Low => self.count_low += 1,
            High => self.count_high += 1,
        };
        match (message.destination.clone().as_ref(), message.pulse.clone()) {
            ("rx", Low) => panic!(),
            _ => (),
        }
        // let str_pulse = if message.pulse == Low {
        //     "-low"
        // } else {
        //     "-high"
        // };
        // println!(
        //     "{} {} -> {}",
        //     message.source, str_pulse, message.destination
        // );
        if let Some(module) = &mut self.all_modules.get_mut(&message.destination) {
            module.process_message(message)
        } else {
            vec![]
        }
    }
}

#[derive(Clone, Debug)]
struct Module {
    name: String,
    module_type: ModuleType,
    destinations_str: Vec<String>,
}

impl FromStr for Module {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter_elt = s.split(" -> ");
        let part_1 = iter_elt.next().unwrap();
        let mut iter_chars = part_1.chars();
        let (name, module_type) = match iter_chars.next().unwrap() {
            '%' => (iter_chars.collect(), FlipFlop(Low)),
            '&' => (iter_chars.collect(), Conjuction(HashMap::new())),
            _ => (part_1.to_string(), Broadcaster),
        };
        let destinations_str = iter_elt
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        let module = Module {
            name,
            module_type,
            destinations_str,
        };
        Ok(module)
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Message {
    source: String,
    destination: String,
    pulse: Pulse,
}

impl Module {
    fn build_messages(&self, pulse: Pulse) -> Vec<Message> {
        self.destinations_str
            .iter()
            .map(|dest_str| Message {
                source: self.name.clone(),
                destination: dest_str.clone(),
                pulse: pulse.clone(),
            })
            .collect()
    }

    fn get_all_low(&self) -> Vec<Message> {
        self.build_messages(Pulse::Low)
    }

    fn process_message(&mut self, message: Message) -> Vec<Message> {
        match self.module_type {
            FlipFlop(ref mut pulse_state) => {
                if message.pulse == Pulse::Low {
                    let new_pulse = pulse_state.opposite();
                    *pulse_state = new_pulse.clone();
                    self.build_messages(new_pulse)
                } else {
                    vec![]
                }
            }
            Conjuction(ref mut hash_pulse) => {
                hash_pulse
                    .entry(message.source)
                    .and_modify(|p| *p = message.pulse.clone())
                    .or_insert(message.pulse.clone());
                let all_high_pulse = hash_pulse.values().fold(true, |acc, p| acc && p == &High);
                let send_pulse = if all_high_pulse { Low } else { High };
                self.build_messages(send_pulse)
            }
            Broadcaster => self.build_messages(message.pulse.clone()),
        }
    }
}

fn main() {
    // let input = include_str!("../input_test_1.txt");
    // let input = include_str!("../input_test_2.txt");
    let input = include_str!("../input.txt");
    let mut all_modules = HashMap::new();
    input.split_terminator('\n').for_each(|line| {
        let module = Module::from_str(line).unwrap();
        all_modules.insert(module.name.clone(), module);
    });
    let mut system = System {
        all_modules,
        count_low: 0,
        count_high: 0,
    };
    system.send_all_low();
    system.reset_low();
    system.run_loop();
    let score = system.score();
    dbg!(score);
}
