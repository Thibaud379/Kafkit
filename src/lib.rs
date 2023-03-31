#![feature(iter_intersperse)]

mod eq_gates;
mod re;
#[derive(Clone, clap::ValueEnum)]
pub enum Mode {
    Eq,
    Add,
    Del,
}

// enum Gates {
//     CX,
//     X,
//     H,
//     Z,
//     SWAP,
//     Y,
//     S,
//     T,
// }

pub fn mutate(
    file: String,
    mode: Mode,
    _mutation_amount: f32,
    nb_mut: i32,
) -> Result<Vec<String>, String> {
    let mut mutations: Vec<String> = Vec::with_capacity(nb_mut.try_into().unwrap());
    let f = match mode {
        Mode::Add => mutate_add,
        Mode::Eq => mutate_eq,
        Mode::Del => mutate_del,
    };
    let (re_begin, re_end, re_size, re_loop) = re::circ_tags();
    let mut re_def = re::circ_def("n");
    let mut re_gate = re::gate("circuit");
    for _ in 0..nb_mut {
        let mut mutation = String::new();
        let mut in_circuit = false;
        let mut circuit_name = None;
        let mut size: Option<i32> = None;
        let mut size_name = None;
        let mut in_loop = false;
        let mut entered_loop = false;
        let mut loop_buffer = String::new();
        let mut loop_indent = String::new();
        let mut loop_var = String::new();
        let mut loop_steps = 0;
        for line in file.lines() {
            if re_end.is_match(line) {
                in_circuit = false;
            }

            if !in_circuit {
                mutation += &format!("{line}\n");
            } else {
                if !in_loop {
                    if let Some(c) = re_loop.captures(line) {
                        in_loop = true;
                        entered_loop = true;
                        loop_var = c.get(1).unwrap().as_str().into();
                        let steps = c.get(2).unwrap();
                        if let Ok(n) = steps.as_str().parse::<i32>() {
                            loop_steps = n;
                        } else if Some(steps.as_str().to_string()).eq(&size_name) {
                            loop_steps = *size.as_ref().unwrap();
                        }
                    } else {
                        mutation += &format!("{line}\n");
                    }
                } else {
                    if let Some(c) = re_loop.captures(line) {
                        return Err("Nested loops not handled".into());
                    }
                    if entered_loop {
                        entered_loop = false;
                        loop_indent = line.chars().take_while(|&c| c.is_whitespace()).collect();
                        loop_buffer += &format!("{line}\n");
                    } else {
                        let ident: String =
                            line.chars().take_while(|&c| c.is_whitespace()).collect();
                        if !ident.eq(&loop_indent) {
                            in_loop = false;
                            mutation += &unroll_loop(
                                &loop_buffer,
                                &loop_var,
                                loop_steps,
                                f,
                                _mutation_amount,
                            );
                            loop_buffer = String::new();
                            loop_var = String::new();
                            loop_indent = String::new();
                            loop_steps = 0;
                        } else {
                            loop_buffer += &format!("{line}\n");
                        }
                    }
                }
            }

            if re_begin.is_match(line) {
                if circuit_name.is_none() {
                    return Err("Circuit variable was not defined before circuit definition".into());
                }
                if size.is_none() {
                    return Err("Circuit size was not defined before circuit variable".into());
                }
                in_circuit = true;
            }

            if size.is_none() {
                if let Some(s) = re_size.captures(line) {
                    let name = s.get(1).unwrap().as_str().to_string();
                    re_def = re::circ_def(name.as_str());
                    re_gate = re::gate(name.as_str());
                    size_name = Some(name);
                    size = Some(s.get(2).unwrap().as_str().parse().unwrap());
                }
            }
            if circuit_name.is_none() {
                if let Some(n) = re_def.captures(line) {
                    circuit_name = Some(n.get(1).unwrap().as_str().to_string());
                }
            }
        }
        mutations.push(mutation)
    }
    Ok(mutations)
}

fn mutate_eq(line: &mut String, _mutation_amount: f32) {}
fn mutate_add(line: &mut String, _mutation_amount: f32) {}
fn mutate_del(line: &mut String, _mutation_amount: f32) {}

fn unroll_loop(
    buffer: &String,
    var_name: &String,
    loop_steps: i32,
    f: fn(&mut String, f32) -> (),
    mut_amount: f32,
) -> String {
    let mut out = String::new();
    let pat = format!("({var_name})");
    for i in 0..loop_steps {
        let u = format!("({i})");
        for line in buffer.lines() {
            let mut new_line = line.trim_start().replace(&pat, &u);
            f(&mut new_line, mut_amount);
            out += &new_line;
            out += "\n";
        }
    }
    out
}
