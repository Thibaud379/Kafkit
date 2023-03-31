use clap::Parser;
use core::panic;
use kafkit::*;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    input_path: PathBuf,
    output_path: PathBuf,

    #[clap(default_value_t = 5)]
    nb_outputs: i32,

    #[clap(value_enum, default_value_t=Mode::Eq)]
    mode: Mode,

    #[clap(default_value_t = 0.2)]
    mutation_amount: f32,
}

fn main() {
    let args = Cli::parse();
    let input_file = match std::fs::read_to_string(&args.input_path) {
        Ok(file) => file,
        Err(e) => panic!("Error opening file {}:\n {e}", args.input_path.display()),
    };

    let mutations = match mutate(input_file, args.mode, args.mutation_amount, args.nb_outputs) {
        Ok(m) => m,
        Err(e) => panic!("Error mutating circuit:\n{e}"),
    };
    let out = args.output_path;
    let name = out
        .file_name()
        .expect("Output path not a file")
        .to_str()
        .unwrap();
    for (i, mutation) in mutations.iter().enumerate() {
        std::fs::write(out.with_file_name(format!("{name}_{i}.py")), mutation)
            .expect("Error writing file");
    }
}
