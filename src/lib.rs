#![feature(iter_intersperse)]

mod gates;
mod re;

use gates::Gate;
use rand::random;

use re::RegexStore;

#[derive(Clone, clap::ValueEnum)]
pub enum Mode {
    Eq,
    Add,
    Del,
}

#[derive(Default)]
pub struct Circuit {
    name: Option<String>,
    size: Option<usize>,
    size_name: Option<String>,
}

#[derive(Default)]
struct LoopUnroll {
    in_loop: bool,
    entered_loop: bool,
    loop_buffer: String,
    loop_indent: String,
    loop_var: String,
    loop_steps: usize,
}

pub fn mutate(
    file: String,
    mode: Mode,
    mutation_amount: f32,
    nb_mut: usize,
) -> Result<Vec<String>, String> {
    let mut mutations: Vec<String> = std::iter::repeat(String::new()).take(nb_mut).collect();
    let f = match mode {
        Mode::Add => mutate_add,
        Mode::Eq => mutate_eq,
        Mode::Del => mutate_del,
    };

    let mut rs = RegexStore::new();

    let (file_unrolled, circuit) = pre_process(file, &mut rs);
    let mut in_circuit = false;
    for line in file_unrolled.lines() {
        if rs.end.is_match(line) {
            in_circuit = false;
        }

        if !in_circuit {
            append_line(&mut mutations, line);
        } else {
            for mutation in mutations.iter_mut() {
                let mut new_line = String::from(line);

                f(&mut new_line, mutation_amount, &rs, &circuit);
                *mutation += &format!("{new_line}\n");
            }
        }

        if rs.begin.is_match(line) {
            in_circuit = true;
        }
    }
    Ok(mutations)
}

fn mutate_eq(_line: &mut String, _mutation_amount: f32, _rs: &RegexStore, _c: &Circuit) {}
fn mutate_add(line: &mut String, mutation_amount: f32, _: &RegexStore, c: &Circuit) {
    if random::<f32>() < mutation_amount {
        let gate = random::<Gate>();
        let gate = gate.generate_gate(c);
        *line += format!("\n{gate} # Added gate").as_str();
    }
}
fn mutate_del(line: &mut String, mutation_amount: f32, rs: &RegexStore, _: &Circuit) {
    if rs.gate().is_match(line) && random::<f32>() < mutation_amount {
        *line = String::from("# Deleted");
    }
}

fn append_line(v: &mut Vec<String>, line: &str) {
    for m in v {
        *m += format!("{line}\n").as_str();
    }
}

fn pre_process(input: String, rs: &mut RegexStore) -> (String, Circuit) {
    let mut output = String::new();
    let mut circuit = Circuit::default();
    let mut l = LoopUnroll::default();

    let mut in_circuit = false;

    for line in input.lines() {
        if rs.end.is_match(line) {
            in_circuit = false;
        }
        if !in_circuit {
            output += format!("{line}\n").as_str();

            if circuit.size_name.is_none() {
                if let Some(s) = rs.size.captures(line) {
                    let name = s.get(1).unwrap().as_str().to_string();
                    rs.set_size_name(&name);
                    circuit.size_name = Some(name);
                    circuit.size = Some(s.get(2).unwrap().as_str().parse().unwrap());
                }
            } else if circuit.name.is_none() {
                if let Some(n) = rs.circ().captures(line) {
                    let name = n.get(1).unwrap().as_str().to_string();
                    rs.set_circ_name(&name);
                    circuit.name = Some(name);
                }
            }
        } else if !l.in_loop {
            if let Some(c) = rs.l.captures(line) {
                l.in_loop = true;
                l.entered_loop = true;
                l.loop_var = c.get(1).unwrap().as_str().into();
                let steps = c.get(2).unwrap();
                if let Ok(n) = steps.as_str().parse::<usize>() {
                    l.loop_steps = n;
                } else if Some(steps.as_str().to_string()).eq(&circuit.size_name) {
                    l.loop_steps = *circuit.size.as_ref().unwrap();
                } else {
                    panic!("Cannot evaluate loop iterations.")
                }
                output += "# Unrolled loop\n";
            } else {
                output += &format!("{line}\n");
            }
        } else {
            if rs.l.is_match(line) {
                panic!("Nested loops not handled.");
            }
            if l.entered_loop {
                l.entered_loop = false;
                l.loop_indent = line.chars().take_while(|&c| c.is_whitespace()).collect();
                l.loop_buffer += &format!("{line}\n");
            } else {
                let indent: String = line.chars().take_while(|&c| c.is_whitespace()).collect();
                if !indent.eq(&l.loop_indent) {
                    output += &unroll_loop(&l);

                    l = LoopUnroll::default();
                } else {
                    l.loop_buffer += &format!("{line}\n");
                }
            }
        }
        if rs.begin.is_match(line) {
            in_circuit = true;
        }
    }
    (output, circuit)
}

fn unroll_loop(l: &LoopUnroll) -> String {
    let mut out = String::new();
    let pat = format!("({})", l.loop_var);
    for i in 0..l.loop_steps {
        let u = format!("({i})");
        out += format!("# Itération {i}\n").as_str();
        for line in l.loop_buffer.lines() {
            out += &line.trim_start().replace(&pat, &u);
            out += "\n";
        }
    }
    out += "# Loop end\n";
    out
}
