use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;



fn main() {

    let re = Regex::new(r"([%&]*)([a-z]+) -> ([a-z, ]*)")
        .unwrap();
    let mut hm = HashMap::<String, Module>::new();
    let mut broadcasts = Vec::<String>::new();

    let file = File::open("input1.txt").unwrap();
    io::BufReader::new(&file)
        .lines()
        .for_each(|x|{
            let x = x.unwrap();
            let (_, [tp, nm, recpts]) = re.captures(&x).unwrap().extract();
            let recpts = recpts.split(", ").map(|x| x.to_string()).collect();

            match tp {
                "" => {broadcasts = recpts;},
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

    let mut mods = Modules{ modules, broadcasts};

    println!("{:?}", mods);
    println!("-----------------------");
    let (h,l) = mods.press_button();
    println!("{:?}", mods);

    println!("Part 1: {}, {}", h,l);
}





#[derive(PartialEq,Eq, Debug, Clone)]
struct Modules{
    modules: HashMap<String, Module>,
    broadcasts: Vec<String>,
}

#[derive(Clone)]
struct Message {
    src: String,
    dst: String,
    sig: Signal,
}

impl Modules {
    fn press_button(&mut self) -> (usize, usize) {
        let (mut lows, mut highs) = (0,0);

        let mut queue: VecDeque<Message> = self
            .broadcasts
            .iter()
            .map(|x| {
                Message { src: "broadcaster".to_string(), dst: x.clone(), sig: Signal::Low }
            })
            .collect();
        lows += queue.len();

        while !queue.is_empty() {
            let mut nq = VecDeque::<Message>::new();

            while let Some(msg) = queue.pop_front() {

                match self.modules.get_mut(&msg.dst) {
                    Some(Module::Conjunction(m)) => {
                        // we have to collect all entries of name n in quee
                        let (inq, sigs) = queue
                            .iter()
                            .fold(
                                (VecDeque::<Message>::new(),Vec::<(String,Signal)>::new()),
                                |mut a,i| {
                                    if i.dst == msg.dst {
                                        a.1.push((i.src.clone(), i.sig.clone()));
                                    } else {
                                        a.0.push_back(i.clone());
                                    }
                                    a
                                }
                            );
                        let sig = m.get_signal(sigs).unwrap();
                        let mut recpts: VecDeque<Message>= m.get_recipients().into_iter().map(|x| Message{src: msg.dst.clone(), dst: x.clone(), sig: sig.clone()}).collect();
                        match sig {
                            Signal::High => {highs += recpts.len();},
                            Signal::Low => {lows += recpts.len();},
                        }
                        nq.append(&mut recpts);
                        queue = inq;
                    },
                    Some(Module::FlipFlop(m)) => {
                        if let Some(sig) = m.get_signal(msg.sig) {
                            let mut recpts: VecDeque<Message> = m.get_recipients().into_iter().map(|x| Message{src: msg.dst.clone(), dst: x.clone(), sig: sig.clone()}).collect();
                            match sig {
                                Signal::High => {highs += recpts.len();},
                                Signal::Low => {lows += recpts.len();},
                            }
                            nq.append(&mut recpts);
                        };
                    },
                    _ => {},
                }
            }
            queue = nq;
        }
        (lows, highs)
    }

}
trait Recipients {
    fn get_recipients(&self) -> &Vec<String>;
}


#[derive(PartialEq,Eq, Debug, Clone)]
enum Signal {
    High,
    Low,
}

#[derive(PartialEq, Eq, Debug, Clone)]
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
