use regex::Regex;
//(.+)\s*=\s*QuantumCircuit\((\d+)(?:,(\d+))?\)
pub fn circ_tags() -> (Regex, Regex, Regex, Regex) {
    (
        Regex::new("# Circuit begin").unwrap(),
        Regex::new("# Circuit end").unwrap(),
        Regex::new(r"(\S+)\s*=\s*(\d+)\s*# Circuit size").unwrap(),
        Regex::new(r"for (\w+) in range\((\w+)\):\s*# Unroll").unwrap(),
    )
}

pub fn gate(ciruit_name: &str) -> Regex {
    Regex::new(format!(r"{ciruit_name}\.(\w+)\((.+)\)").as_str()).unwrap()
}

pub fn circ_def(size_name: &str) -> Regex {
    Regex::new(format!(r"(.+)\s*=\s*QuantumCircuit\({size_name}\)\s*# Circuit").as_str()).unwrap()
}
