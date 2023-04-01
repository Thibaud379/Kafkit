use regex::Regex;
//(.+)\s*=\s*QuantumCircuit\((\d+)(?:,(\d+))?\)

pub struct RegexStore {
    pub begin: Regex,
    pub end: Regex,
    pub size: Regex,
    pub l: Regex,
    circ: Option<Regex>,
    gate: Option<Regex>,
}

impl RegexStore {
    pub fn new() -> Self {
        RegexStore {
            begin: Regex::new("# Circuit begin").unwrap(),
            end: Regex::new("# Circuit end").unwrap(),
            size: Regex::new(r"(\S+)\s*=\s*(\d+)\s*# Circuit size").unwrap(),
            l: Regex::new(r"for (\w+) in range\((\w+)\):\s*# Unroll").unwrap(),
            circ: None,
            gate: None,
        }
    }

    pub fn set_size_name(&mut self, sn: &str) {
        self.circ = Some(
            Regex::new(format!(r"(\S+)\s*=\s*QuantumCircuit\({sn}\)\s*# Circuit").as_str())
                .unwrap(),
        )
    }

    pub fn set_circ_name(&mut self, cn: &str) {
        self.gate = Some(Regex::new(format!(r"{cn}\.(\w+)\((.+)\)").as_str()).unwrap())
    }

    pub fn circ(&self) -> &Regex {
        match &self.circ {
            Some(r) => r,
            None => panic!("Error, input file cannot be handled"),
        }
    }
    pub fn gate(&self) -> &Regex {
        match &self.gate {
            Some(r) => r,
            None => panic!("Error, input file cannot be handled"),
        }
    }
}
