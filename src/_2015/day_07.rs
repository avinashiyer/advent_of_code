use regex::{Regex, RegexSet};
use std::cell::LazyCell;
use std::collections::{HashMap, HashSet};
use std::ops::Not;
use std::str::Split;
use std::sync::LazyLock;
use std::{hash::Hash, io::Error};


static mut CACHED_VALUES:LazyCell<HashMap<String,u16>> = LazyCell::new(|| HashMap::new()); 


pub fn driver(lines: Vec<&str>, tgt:&str) -> Result<u16, Error> {
    let regexSet = RegexSet::new([
        r"^(?:\d+|[a-z]+) -> (?:[a-z]+)",
        r"^NOT (?:[a-z]+) -> (?:[a-z]+)",
        r"^(?:[a-z]+|\d+) (?:OR|AND) (?:[a-z]+|\d+) -> (?:[a-z]+)",
        r"^(?:[a-z]+|\d+) (?:LSHIFT|RSHIFT) (?:\d+) -> (?:[a-z]+)",
    ])
    .unwrap();

    let mut out_wire_map = HashMap::<String,Gate>::new();
    for (line) in lines.iter() {
        let pat = which_pat(line, &regexSet).expect("Which Pat returned None");
        let gate = dispatch_to_gate_constructor(line, pat);
        out_wire_map.insert(gate.out_wire.clone(), gate);
        // dbg!(line);
        // dbg!(gate);
    }
    todo!()
}

fn eval_op(map:&HashMap<String,Gate>,tgt:String) -> u16 {
    let source_gate = map.get(&tgt).expect("Target String is not in map");
    todo!();
}

fn eval_and_gate(map:&HashMap<String,Gate>, gate:&Gate) -> u16 {
    if let GateKind::And(l_op, r_op) = &gate.kind {
        let r_val = get_val(map, r_op.clone());
        let l_val = get_val(map, l_op.clone());
        return r_val & l_val; 
    }
    panic!("Didn't match if let in eval_and_gate");
}

fn eval_or_gate(map:&HashMap<String,Gate>,gate:&Gate) -> u16 {
    if let GateKind::Or(l_op, r_op) = &gate.kind {
        let r_val = get_val(map, r_op.clone());
        let l_val = get_val(map, l_op.clone());
        return r_val | l_val; 
    }
    panic!("Didn't match if let in eval_or_gate");
}

fn get_val(map:&HashMap<String,Gate>,NorV:NumOrVar) -> u16 {
    match NorV {
        NumOrVar::NUM(n) => n,
        NumOrVar::VAR(var)=> eval_op(map, var),
    }
}


fn which_pat(line: &str, set: &RegexSet) -> Option<usize> {
    let matches: Vec<_> = set.matches(line).iter().collect();
    // let debug_copy:Vec<_> = matches.iter().copied().collect();
    // Regexs should be mutually exclusive across given input text so only one match whould be possible
    assert_eq!(matches.len(), 1);
    Some(matches[0])
}

fn dispatch_to_gate_constructor(line: &str, pat: usize) -> Gate {
    match pat {
        0 => make_assign_gate(tokenize_input(line)),
        1 => make_not_gate(tokenize_input(line)),
        2 => make_and_or_gate(tokenize_input(line)),
        3 => make_shift_gate(tokenize_input(line)),
        _ => panic!("Index passed to dispatch is out of range"),
    }
}

fn make_assign_gate(tokens: Vec<&str>) -> Gate {
     // Line must match pattern: r"(\d|[a-z]+) -> ([a-z]+)
    assert_eq!(tokens.len(), 3);
    let left_side = NumOrVar::convert_slice(tokens[0]);
    let right_side = tokens[2].to_string();
    // Lazily evaluate the value of left_side, should be evaluated during recursion call
    Gate {
        kind: GateKind::Assign(left_side),
        out_wire: right_side,
    }
}

fn make_not_gate(tokens: Vec<&str>) -> Gate {
    // matched regex r"NOT (\d+|[a-z]+) -> (a-z])+
    assert_eq!(tokens.len(), 4);
    let left_side = NumOrVar::convert_slice(tokens[1]);
    let right_side = tokens[3].to_string();
    // Lazily evaluate
    Gate {
        kind: GateKind::Not(left_side),
        out_wire: right_side,
    }
}

fn make_and_or_gate(tokens: Vec<&str>) -> Gate {
    // matched regex r"^([a-z]+|\d+) (OR|AND) ([a-z]+|\d+) -> ([a-z]+)",
    assert_eq!(tokens.len(), 5);
    let right_side = tokens[4].to_string();
    match tokens[1] {
        "AND" => Gate {
            kind: GateKind::And(
                NumOrVar::convert_slice(tokens[0]),
                NumOrVar::convert_slice(tokens[2]),
            ),
            out_wire: right_side,
        },
        "OR" => Gate {
            kind: GateKind::Or(
                NumOrVar::convert_slice(tokens[0]),
                NumOrVar::convert_slice(tokens[2]),
            ),
            out_wire: right_side,
        },
        _ => panic!("Regex pattern matched bad input. Second token isnt OR or AND"),
    }
}

fn make_shift_gate(tokens: Vec<&str>) -> Gate{
    // Assert Line Matched r"^(?:[a-z]+|\d+) (?:LSHIFT|RSHIFT) (?:\d+) -> (?:[a-z]+)"
    assert_eq!(tokens.len(),5);
    let right_side = tokens[4].to_string();
    let left_var = tokens[0].to_string();
    let left_num:u16 = tokens[2].parse().unwrap();
    match tokens[1] {
        "LSHIFT" => Gate{ kind: GateKind::Lshift(left_var, left_num), out_wire: right_side },
        "RSHIFT" => Gate{ kind: GateKind::Rshift(left_var, left_num), out_wire: right_side },
        _ => panic!("Regex pattern matched bad input. Second token isnt LSHIFT or RSHIFT"),
    }
}

// Convenience function to split input line into tokens before passing to a Gate constructor
fn tokenize_input(line: &str) -> Vec<&str>{
    line.split_ascii_whitespace().collect()
}
#[derive(Debug)]
enum GateKind {
    And(NumOrVar, NumOrVar),
    Or(NumOrVar, NumOrVar),
    Not(NumOrVar),
    Lshift(String, u16),
    Rshift(String, u16),
    Assign(NumOrVar),
}

#[derive(Debug, Clone)]
enum NumOrVar {
    NUM(u16),
    VAR(String),
}

impl NumOrVar {
    fn convert_slice(tgt: &str) -> NumOrVar {
        static RE: LazyLock<RegexSet> =
            LazyLock::new(|| RegexSet::new([r"^\d+$", r"^[a-z][a-z]?$"]).unwrap());
        let matches: Vec<_> = RE.matches(tgt).iter().collect();
        // Regexes are mutually exclusive. If no matches are found then that's an input error, if 2 are found thats an error as well
        assert_eq!(matches.len(), 1);
        match matches[0] {
            //If matches[0]==0 then tgt is a number, if matches[0] == 1 then tgt is a variable
            0 => NumOrVar::NUM(tgt.parse::<u16>().unwrap()),
            1 => NumOrVar::VAR(tgt.to_string()),
            _ => panic!("Regex Set was edited without editing match statement"),
        }
    }
}
#[derive(Debug)]
struct Gate {
    kind: GateKind,
    out_wire: String,
}
