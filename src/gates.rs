use rand::seq::SliceRandom;
use rand::{distributions::Standard, prelude::Distribution};
use strum_macros::{AsRefStr, EnumString};

use crate::Circuit;

// pub fn cx_to_hczh(line: &str) -> String {
//     let _new_lines = String::from(line);

//     String::from("")
// }
#[derive(AsRefStr, EnumString)]
#[allow(non_camel_case_types)]
pub enum Gate {
    cx,
    x,
    h,
    z,
    swap,
    y,
    s,
    t,
    cz,
    ccx,
}
impl Distribution<Gate> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Gate {
        match rng.gen_range(0..=9) {
            0 => Gate::cx,
            1 => Gate::x,
            2 => Gate::h,
            3 => Gate::z,
            4 => Gate::swap,
            5 => Gate::y,
            6 => Gate::s,
            7 => Gate::t,
            8 => Gate::cz,
            _ => Gate::ccx,
        }
    }
}

impl Gate {
    fn num_qbits(&self) -> usize {
        match self {
            Gate::x | Gate::h | Gate::z | Gate::y | Gate::s | Gate::t => 1,
            Gate::cx | Gate::swap | Gate::cz => 2,
            Gate::ccx => 3,
        }
    }

    pub fn generate_gate(&self, args: Option<&Vec<usize>>, circuit: &Circuit) -> String {
        let mut rng = rand::thread_rng();
        let args_local;
        let args = match args {
            Some(a) => a,
            None => {
                let a: Vec<usize> = (0..*circuit.size.as_ref().unwrap()).collect();
                args_local = a
                    .choose_multiple(&mut rng, self.num_qbits())
                    .cloned()
                    .collect();
                &args_local
            }
        };

        let args = args
            .iter()
            .fold(String::new(), |acc, &n| acc + &n.to_string() + ", ");

        let gate = format!(
            "{}.{}({})",
            circuit.name.as_ref().unwrap(),
            self.as_ref(),
            &args[0..(args.len() - 2)] //remove last comma
        );
        gate
    }

    pub fn generate_equiv(&self, args: &Vec<usize>, circuit: &Circuit) -> (bool, String) {
        let mut res = "".into();
        let mut changed = true;
        match self {
            // HCzH
            Gate::cx => {
                let a = vec![args.get(1).unwrap().clone()];
                res += (Gate::h.generate_gate(Some(&a), circuit) + "\n").as_str();
                res += (Gate::cz.generate_gate(Some(args), circuit) + "\n").as_str();
                res += Gate::h.generate_gate(Some(&a), circuit).as_str();
            }
            // CxXCx, HssH
            Gate::x => {
                if rand::random::<bool>() {
                    // CxHCx
                    let a = match args.get(0).unwrap() {
                        0 => vec![1, 0],
                        i => vec![0, *i],
                    };
                    res += (Gate::cx.generate_gate(Some(&a), circuit) + "\n").as_str();
                    res += (self.generate_gate(Some(args), circuit) + "\n").as_str();
                    res += Gate::cx.generate_gate(Some(&a), circuit).as_str();
                } else {
                    //HssH
                    res += (Gate::h.generate_gate(Some(args), circuit) + "\n").as_str();
                    res += (Gate::s.generate_gate(Some(args), circuit) + "\n").as_str();
                    res += (Gate::s.generate_gate(Some(args), circuit) + "\n").as_str();
                    res += Gate::h.generate_gate(Some(args), circuit).as_str();
                }
            }
            // CxZCX, ss
            Gate::z => {
                if rand::random::<bool>() {
                    // CxZCx
                    let a = match args.get(0).unwrap() {
                        0 => vec![1, 0],
                        i => vec![0, *i],
                    };
                    res += (Gate::cx.generate_gate(Some(&a), circuit) + "\n").as_str();
                    res += (self.generate_gate(Some(args), circuit) + "\n").as_str();
                    res += Gate::cx.generate_gate(Some(&a), circuit).as_str();
                } else {
                    //ss
                    res += (Gate::s.generate_gate(Some(args), circuit) + "\n").as_str();
                    res += Gate::s.generate_gate(Some(args), circuit).as_str();
                }
            }
            // CxCxCx
            Gate::swap => {
                let mut a = args.clone();
                a.reverse();

                res += (Gate::cx.generate_gate(Some(args), circuit) + "\n").as_str();
                res += (Gate::cx.generate_gate(Some(&a), circuit) + "\n").as_str();
                res += Gate::cx.generate_gate(Some(args), circuit).as_str()
            }
            //tt
            Gate::s => {
                res += (Gate::t.generate_gate(Some(args), circuit) + "\n").as_str();
                res += Gate::t.generate_gate(Some(args), circuit).as_str();
            }
            //HCxH
            Gate::cz => {
                let a = vec![args.get(1).unwrap().clone()];
                res += (Gate::h.generate_gate(Some(&a), circuit) + "\n").as_str();
                res += (Gate::cx.generate_gate(Some(args), circuit) + "\n").as_str();
                res += Gate::h.generate_gate(Some(&a), circuit).as_str();
            }
            // No equivalent in Qdiff project
            _ => changed = false,
        };
        (changed, res)
    }
}

/*
def x_to_cnotxcnot(codeline:str,number:int):
def cz_to_hcnoth(codeline:str,number:int):
x_to_hssh(codeline:str,number:int):
 */
