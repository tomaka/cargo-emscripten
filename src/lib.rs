#![feature(phase)]

#[phase(plugin)]
extern crate regex_macros;

extern crate cargo;
extern crate regex;

use cargo::ops::{ExecEngine, CommandPrototype};
use cargo::util::{mod, CargoResult, ProcessError, ProcessBuilder};

use std::io::fs::{File, PathExtensions};
use std::io::process::ProcessOutput;

pub struct EmscriptenEngine {
    pub emcc: Option<Path>,
}

impl ExecEngine for EmscriptenEngine {
    fn exec(&self, command: CommandPrototype) -> Result<(), ProcessError> {
        exec(command, false, self).map(|_| ())
    }

    fn exec_with_output(&self, command: CommandPrototype)
                        -> Result<ProcessOutput, ProcessError> {
        exec(command, true, self).map(|a| a.unwrap())
    }
}

fn exec(command: CommandPrototype, with_output: bool, engine: &EmscriptenEngine)
        -> Result<Option<ProcessOutput>, ProcessError>
{
    // if we don't find `--crate-type bin`, returning immediatly
    if command.get_args().windows(2)
                         .find(|&args| {
                             args[0].as_str() == Some("--crate-type") &&
                             args[1].as_str() == Some("bin")
                         }).is_none()
    {
        let p = command.into_process_builder().unwrap();
        return do_exec(p, with_output);
    }

    // finding crate name
    let crate_name = command.get_args().windows(2)
                            .filter_map(|args| {
                                if args[0].as_str() == Some("--crate-name") {
                                    Some(args[1].as_str().unwrap().to_string())
                                } else {
                                    None
                                }
                            })
                            .next().unwrap();

    // finding out dir
    let out_dir = command.get_args().windows(2)
                            .filter_map(|args| {
                                if args[0].as_str() == Some("--out-dir") {
                                    Some(args[1].as_str().unwrap().to_string())
                                } else {
                                    None
                                }
                            })
                            .next().unwrap();

    // executing compiler
    {
        let mut new_command = CommandPrototype::new(command.get_type().clone()).unwrap();
        for arg in command.get_args().iter().filter(|a| !a.as_str().unwrap().starts_with("--emit")) {
            new_command = new_command.arg(arg.as_bytes_no_nul());
        }
        for (key, val) in command.get_envs().iter() {
            new_command = new_command.env(key.as_slice(), val.as_ref().map(|v| v.as_bytes_no_nul()));
        }
        new_command = new_command.cwd(command.get_cwd().clone());

        let new_command = new_command.arg("--emit=llvm-ir,dep-info");
        try!(do_exec(new_command.into_process_builder().unwrap(), with_output));
    }

    // this is the output file
    let ll_output_file = Path::new(format!("{}/{}.ll", out_dir, crate_name));
    assert!(ll_output_file.exists());

    // dropping "dereferenceable" from the content of the file
    drop_unsupported_ir(&ll_output_file);

    // executing emcc
    let emcc = engine.emcc.as_ref().map(|p| p.as_str().unwrap()).unwrap_or("emcc");
    do_exec(util::process(emcc).unwrap()
                .arg(ll_output_file)
                .arg("-lGL").arg("-lSDL").arg("-s").arg("USE_SDL=2")
                .arg("-o").arg(format!("{}/{}.html", out_dir, crate_name))
        , with_output)
}

fn do_exec(process: ProcessBuilder, with_output: bool) -> Result<Option<ProcessOutput>, ProcessError> {
    if with_output {
        process.exec_with_output().map(|o| Some(o))
    } else {
        process.exec().map(|_| None)
    }
}

fn drop_unsupported_ir(file: &Path) {
    let mut content = File::open(file).unwrap().read_to_string().unwrap();

    loop {
        let pos = match content.as_slice().find_str("dereferenceable(") {
            Some(p) => p,
            None => break
        };

        let len = content.as_slice().slice_from(pos).find(')').unwrap();

        content = format!("{}{}", content.slice_to(pos), content.slice_from(pos + len + 1));
    }

    write!(&mut File::create(file).unwrap(), "{}", content).unwrap();
}
