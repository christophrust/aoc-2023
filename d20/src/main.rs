use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;



fn main() {

    let re = Regex::new(r"([%&]*)([a-z]+) -> ([a-z, ]*)")
        .unwrap();
    let mut hm = HashMap::<String, Module>::new();
    let mut broadcast = Vec::<String>::new();

    let file = File::open("input1.txt").unwrap();
    io::BufReader::new(&file)
        .lines()
        .for_each(|x|{
            let x = x.unwrap();
            let (_, [tp, nm, recpts]) = re.captures(&x).unwrap().extract();
            let recpts = recpts.split(", ").map(|x| x.to_string()).collect();

            match tp {
                "" => {broadcast = recpts;},
                "&" => {hm.insert(
                    nm.to_string(),
                    Module::Conjunction(
                        Conjunction { inputs: HashMap::<String, Signal>::new() , recipients: recpts })
                );},
                "%" => {hm.insert(
                    nm.to_string(),
                    Module::FlipFlop(
                        FlipFlop { state: false, recipients: recpts }
                    )
                );},
                _ => unreachable!(),
            };
            ()
        });
    let mut modules = hm.clone();
    for (k,v) in modules.iter_mut() {
        if let Module::Conjunction(m) = v {
            let inputs = hm
                .iter()
                .filter_map(|(nm, v)| {
                    if v.get_recipients().contains(&k) {
                        return Some((nm.to_owned(), Signal::Low));
                    }
                    None
                })
                .collect();
            m.inputs = inputs;
        }
    }

    println!("{:?}", modules);

    println!("Part 1: {}", 1);
}



trait Recipients {
    fn get_recipients(&self) -> &Vec<String>;
}


#[derive(PartialEq,Eq, Debug, Clone)]
enum Signal {
    High,
    Low,
}

#[derive(PartialEq,Eq, Debug, Clone)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

impl Recipients for Module {
    fn get_recipients(&self) -> &Vec<String> {
        match self {
            Module::FlipFlop(x) => x.get_recipients(),
            Module::Conjunction(x) => x.get_recipients(),
        }
    }
}

#[derive(PartialEq,Eq, Debug, Clone)]
struct FlipFlop {
    state: bool,
    recipients: Vec<String>,
}

impl FlipFlop {
    fn new() -> Self {
        Self{
            state: false,
            recipients: vec![],
        }
    }
    fn get_signal(&mut self, signal: Signal ) -> Option<Signal> {
        if signal == Signal::Low {
            match self.state {
                true => {
                    self.state = false;
                    return Some(Signal::Low)
                },
                false => {
                    self.state = true;
                    return Some(Signal::High)
                },
            }
        }
        None
    }
}

impl Recipients for FlipFlop{
    fn get_recipients(&self) -> &Vec<String> {
        &self.recipients
    }
}

impl Recipients for Conjunction {
    fn get_recipients(&self) -> &Vec<String> {
        &self.recipients
    }
}


#[derive(PartialEq,Eq, Debug, Clone)]
struct Conjunction {
    inputs: HashMap<String, Signal>,
    recipients: Vec<String>,
}


impl Conjunction {
    fn new() -> Self {
        Self{
            inputs: HashMap::<String, Signal>::new(),
            recipients: vec![],
        }
    }
    fn add_input(&mut self, inp: String) {
        self.inputs.insert(inp, Signal::Low);
    }
    fn get_signal(&mut self, signals: Vec<(String, Signal)> ) -> Option<Signal> {
        for (sn, s) in signals {
            if let Some(v) = self.inputs.get_mut(&sn) {
                *v = s;
            } else {
                unreachable!()
            }
        }
        match self.inputs.iter().all(|(_,x)| *x == Signal::High) {
            true => Some(Signal::Low),
            false => Some(Signal::High),
        }

    }
}
