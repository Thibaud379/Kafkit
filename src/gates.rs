use rand::seq::SliceRandom;
use rand::{distributions::Standard, prelude::Distribution};
use strum_macros::AsRefStr;

use crate::Circuit;

// pub fn cx_to_hczh(line: &str) -> String {
//     let _new_lines = String::from(line);

//     String::from("")
// }
#[derive(AsRefStr)]
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
    ccx,
}
impl Distribution<Gate> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Gate {
        match rng.gen_range(0..=8) {
            0 => Gate::cx,
            1 => Gate::x,
            2 => Gate::h,
            3 => Gate::z,
            4 => Gate::swap,
            5 => Gate::y,
            6 => Gate::s,
            7 => Gate::t,
            _ => Gate::ccx,
        }
    }
}

impl Gate {
    fn num_qbits(&self) -> usize {
        match self {
            Gate::x | Gate::h | Gate::z | Gate::y | Gate::s | Gate::t => 1,
            Gate::cx | Gate::swap => 2,
            Gate::ccx => 3,
        }
    }

    pub fn generate_gate(&self, circuit: &Circuit) -> String {
        let mut rng = rand::thread_rng();
        let args: Vec<usize> = (0..*circuit.size.as_ref().unwrap()).collect();
        let args: Vec<&usize> = args.choose_multiple(&mut rng, self.num_qbits()).collect();
        let args = args
            .iter()
            .fold(String::new(), |acc, &n| acc + &n.to_string() + ", ");

        let gate = format!(
            "{}.{}({})",
            circuit.name.as_ref().unwrap(),
            self.as_ref(),
            &args[0..(args.len() - 2)]
        );
        gate
    }
}

/*
def x_to_cnotxcnot(codeline:str,number:int):
def cz_to_hcnoth(codeline:str,number:int):
x_to_hssh(codeline:str,number:int):
 */
