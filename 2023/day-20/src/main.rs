use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum PulseType {
    High,
    Low,
    Noop,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ModuleType {
    Broadcast(Vec<String>),
    Flipflop(Vec<String>, bool),
    Conjunction(Vec<String>, HashMap<String, PulseType>),
}

impl ModuleType {
    fn input_pulse(&mut self, src_id: &String, pulse: &PulseType) -> (&Vec<String>, PulseType) {
        match self {
            ModuleType::Broadcast(outs) => {
                return (outs, *pulse);
            }
            ModuleType::Flipflop(outs, state) => {
                let out_pulse = if *pulse == PulseType::Low && !*state {
                    *state = true;
                    PulseType::High
                } else if *pulse == PulseType::Low && *state {
                    *state = false;
                    PulseType::Low
                } else {
                    PulseType::Noop
                };
                return (outs, out_pulse);
            }
            ModuleType::Conjunction(outs, memory) => {
                if let Some(mem_pulse) = memory.get_mut(src_id) {
                    *mem_pulse = *pulse;
                }
                let out_pulse = if memory.iter().any(|(_, v)| *v == PulseType::Low) {
                    PulseType::High
                } else {
                    PulseType::Low
                };
                return (outs, out_pulse);
            }
        }
    }
}

fn main() {
    let data = read_input();
    part1(&data);
    part2(&data);
}

fn read_input() -> HashMap<String, ModuleType> {
    let contents = fs::read_to_string("D:\\Projects\\Code\\adventofcode\\2023\\day-20\\input.txt")
        .expect("Error reading input file.");
    let mut modules = HashMap::new();
    let mut conjunction_inputs = HashMap::new();
    for line in contents.lines() {
        let mut split_line = line.split(" -> ");
        let module_desc = split_line.next().expect("Error getting module desc");
        let dest_modules = split_line
            .next()
            .expect("Error getting destination modules")
            .split(",")
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>();
        if module_desc == "broadcaster" {
            modules.insert(module_desc.to_owned(), ModuleType::Broadcast(dest_modules));
        } else if module_desc.starts_with("%") {
            modules.insert(
                module_desc[1..].to_owned(),
                ModuleType::Flipflop(dest_modules, false),
            );
        } else if module_desc.starts_with("&") {
            modules.insert(
                module_desc[1..].to_owned(),
                ModuleType::Conjunction(dest_modules, HashMap::new()),
            );
            conjunction_inputs.insert(module_desc[1..].to_owned(), Vec::new());
        } else {
            panic!("Unexpected module name {}", module_desc);
        }
    }
    for (id, module) in &modules {
        let outs = match module {
            ModuleType::Broadcast(outs) => outs,
            ModuleType::Flipflop(outs, _) => outs,
            ModuleType::Conjunction(outs, _) => outs,
        };
        for out_id in outs {
            if let Some(conj_inputs) = conjunction_inputs.get_mut(out_id) {
                conj_inputs.push(id.clone());
            }
        }
    }
    for (id, inputs) in conjunction_inputs {
        if let Some(ModuleType::Conjunction(_, in_mem)) = modules.get_mut(&id) {
            for input in inputs {
                in_mem.insert(input, PulseType::Low);
            }
        }
    }
    modules
}

fn part1(modules_orig: &HashMap<String, ModuleType>) {
    let mut modules = modules_orig.clone();
    let mut total_count = (0, 0);
    let mut cycles = 0;
    loop {
        cycles += 1;
        let count = press_button(&mut modules);
        total_count.0 += count.0;
        total_count.1 += count.1;
        if cycles == 1000 {
            break;
        }
    }
    println!("Part 1: {}", total_count.0 * total_count.1);
}

fn part2(modules_orig: &HashMap<String, ModuleType>) {
    let mut modules = modules_orig.clone();
}

fn press_button(modules: &mut HashMap<String, ModuleType>) -> (i64, i64) {
    let mut pulse_count = (0, 0);
    let mut pulse_queue = vec![(
        String::from("button"),
        vec![String::from("broadcaster")],
        PulseType::Low,
    )];
    while !pulse_queue.is_empty() {
        let (pulse_src, pulse_targets, pulse) =
            pulse_queue.pop().expect("Error popping from queue");
        for target in &pulse_targets {
            match pulse {
                PulseType::High => pulse_count.1 += 1,
                PulseType::Low => pulse_count.0 += 1,
                _ => {}
            }
            //println!("{} -{:?}-> {}", pulse_src, pulse, target);

            if let Some(module) = modules.get_mut(target) {
                let (out_targets, out_pulse) = module.input_pulse(&pulse_src, &pulse);
                if out_pulse != PulseType::Noop {
                    pulse_queue.insert(0, (target.clone(), out_targets.clone(), out_pulse));
                }
            }
        }
    }
    pulse_count
}
