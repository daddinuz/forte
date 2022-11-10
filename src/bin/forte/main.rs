mod error;

use std::{env, fs, io};

use forte::Vm;

use crate::error::CliError;

fn main() -> Result<(), CliError> {
    let exe = env!("CARGO_PKG_NAME");
    let args: Vec<String> = env::args().skip(1).collect();

    let (interactive, file) = match &args[..] {
        [mode, file] if mode == "-i" => (true, Some(file)),
        [mode] if mode == "-i" => (true, None),
        [file] => (false, Some(file)),
        [] => (true, None),
        _ => return Err(CliError::from(format!("{exe} [-i] [file]"))),
    };

    let source = file.map_or_else(|| Ok(String::new()), fs::read_to_string)?;
    let program = source.parse()?;
    let mut vm = Vm::load(program);

    if !interactive {
        vm.run()?;
        return Ok(());
    }

    let mut line = String::new();
    loop {
        if let Err(error) = vm.run() {
            eprintln!("Error: {error}");
            vm.unwind();
        }

        line.clear();
        if io::stdin().read_line(&mut line)? == 0 {
            break;
        }

        match line.parse() {
            Ok(program) => vm.extend_program(program),
            Err(error) => eprintln!("Error: {error}"),
        }
    }

    Ok(())
}
