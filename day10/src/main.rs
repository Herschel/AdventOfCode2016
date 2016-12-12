extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

type ChipIndex = u32;
type RobotIndex = u32;
type OutputIndex = u32;

type RobotChips = (Option<ChipIndex>, Option<ChipIndex>);

enum RobotDest {
    Robot(RobotIndex),
    Output(OutputIndex),
}

struct RobotRule {
    low_dest: RobotDest,
    high_dest: RobotDest,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Invalid input");
    
    let mut robots: HashMap<RobotIndex, RobotChips> = HashMap::new();
    let mut outputs: HashMap<OutputIndex, Vec<ChipIndex>> = HashMap::new();
    let mut robot_rules: HashMap<RobotIndex, RobotRule> = HashMap::new();

    let re = Regex::new(r"(value (\d+) goes to bot (\d+))|(bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+))").unwrap();
    for line in input.lines() {
        if let Some(cap) = re.captures(&line) {
            if cap.at(1).is_some() {
                let chip = cap.at(2).unwrap().parse().unwrap();
                let robot_index = cap.at(3).unwrap().parse().unwrap();
                let mut robot = robots.entry(robot_index).or_insert((None, None));
                give_chip_to_robot(&mut robot, chip);
            } else if cap.at(4).is_some() {
                let robot_index = cap.at(5).unwrap().parse().unwrap();
                let low_dest = string_to_dest(cap.at(6).unwrap(), cap.at(7).unwrap().parse().unwrap());
                let high_dest = string_to_dest(cap.at(8).unwrap(), cap.at(9).unwrap().parse().unwrap());

                let robot = robots.entry(robot_index).or_insert((None, None));
                robot_rules.insert(robot_index, RobotRule {
                    low_dest: low_dest,
                    high_dest: high_dest,
                });
            }
        }
    }


    let states = vec![robots.clone().into_iter().collect::<Vec<_>>()];

    let mut robot_i = None;
    for (key, val) in robots.iter() {
        if let &(Some(a), Some(b)) = val {
            robot_i = Some(*key);
            break;
        }
    }

    while let Some(i) = robot_i {
        let (low, high) = {
            let mut src_robot = robots.get_mut(&robot_i.unwrap()).unwrap();
            let (low, high) = (src_robot.0.unwrap(), src_robot.1.unwrap());
            *src_robot = (None, None);
            (low, high)
        };
        let mut robot_rule = robot_rules.get(&robot_i.unwrap()).unwrap();
        match robot_rule.low_dest {
            RobotDest::Robot(i) => {
                let dest = robots.get_mut(&i).unwrap();
                give_chip_to_robot(dest, low);
            },
            RobotDest::Output(i) => {
                let mut output = outputs.entry(i).or_insert_with(|| Vec::new());
                output.push(low);
            }
        }

        match robot_rule.high_dest {
            RobotDest::Robot(i) => {
                let dest = robots.get_mut(&i).unwrap();
                give_chip_to_robot(dest, high);
            },
            RobotDest::Output(i) => {
                let mut output = outputs.entry(i).or_insert_with(|| Vec::new());
                output.push(high);
            }
        }

        for (key, val) in robots.iter() {
            if let &(Some(17), Some(61)) = val {
                println!("Result: Robot {}", *key);
            }
        }

        robot_i = None;
        for (key, val) in robots.iter() {
            if let &(Some(a), Some(b)) = val {
                robot_i = Some(*key);
                break;
            }
        }
    }

    println!("Outputs 0*1*2: {}", outputs.get(&0).unwrap()[0] * outputs.get(&1).unwrap()[0] * outputs.get(&2).unwrap()[0]);
}

fn give_chip_to_robot(robot: &mut RobotChips, chip: ChipIndex) {
    let &mut (ref mut a, ref mut b) = robot;
    if a.is_some() {
        if b.is_some() {
            panic!("Robot has its hands full");
        }
        if a.unwrap() < chip {
            *b = Some(chip)
        } else {
            *b = *a;
            *a = Some(chip);
        }
    } else {
        *a = Some(chip)
    }
}

fn string_to_dest(s: &str, i: u32) -> RobotDest {
    match s {
        "bot" => RobotDest::Robot(i),
        "output" => RobotDest::Output(i),
        _ => panic!("Invalid destination {}", s),
    }
}