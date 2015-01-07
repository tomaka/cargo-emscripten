#![feature(phase)]

#[phase(plugin)]
extern crate regex_macros;

extern crate cargo;
extern crate regex;

use cargo::ops::{ExecEngine, CommandPrototype};
use cargo::util::{mod, CargoResult, ProcessError, ProcessBuilder};

use std::io::TempDir;
use std::io::fs::{File, PathExtensions};
use std::io::process::ProcessOutput;

mod std_inc;

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
    let build_dir = TempDir::new("cargo-emscripten").unwrap();

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

    // writing `specs.json` in the out dir
    File::create(&build_dir.path().join("emscripten.json")).write(include_bytes!("../specs-target.json")).unwrap();

    // writing libstd in the target directory
    let (libs_ll, libs_rlib) = std_inc::write_std(&Path::new(build_dir.path()));

    // if we don't find `--crate-type bin`, returning immediatly
    /*if command.get_args().windows(2)
                         .find(|&args| {
                             args[0].as_str() == Some("--crate-type") &&
                             args[1].as_str() == Some("bin")
                         }).is_none()
    {
        let p = command.into_process_builder().unwrap();
        return do_exec(p, with_output);
    }*/

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

    // executing compiler
    {
        let mut new_command = CommandPrototype::new(command.get_type().clone()).unwrap();
        for arg in command.get_args().iter().filter(|a| !a.as_str().unwrap().starts_with("--emit")) {
            new_command = new_command.arg(arg.as_bytes_no_nul());
        }
        for (key, val) in command.get_envs().iter() {
            new_command = new_command.env(key.as_slice(), val.as_ref().map(|v| v.as_bytes_no_nul()));
        }
        //new_command = new_command.cwd(command.get_cwd().clone());

        new_command = new_command.arg("--emit=llvm-ir,dep-info");
        new_command = new_command.arg("--target").arg("emscripten.json");
        new_command = new_command.cwd(build_dir.path().clone());

        for (name, path) in libs_rlib.into_iter() {
            new_command = new_command.arg("--extern").arg(format!("{}={}", name, path.as_str().unwrap()));
        }

        // TODO: shouldn't be necessary with the "--extern" commands
        new_command = new_command.arg("-L").arg(build_dir.path());

        try!(do_exec(new_command.into_process_builder().unwrap(), with_output));
    }

    // this is the output file
    let ll_output_file = Path::new(format!("{}/{}.ll", out_dir, crate_name));
    assert!(ll_output_file.exists());

    // writing libstd in the target directory
    let libstd = std_inc::write_std(&Path::new(out_dir.clone()));

    // building the "emcc" comand
    let emcc = {
        let emcc = engine.emcc.as_ref().map(|p| p.as_str().unwrap()).unwrap_or("emcc");
        
        let mut process = util::process(emcc).unwrap();
        process = process.arg(ll_output_file);

        for lib in libs_ll.into_iter() {
            process = process.arg(lib);
        }

        process = process.arg("-lGL").arg("-lSDL").arg("-s").arg("USE_SDL=2");
        process = process.arg("-o").arg(format!("{}/{}.html", out_dir, crate_name));
        process = process.cwd(build_dir.path().clone());
        process
    };

    // executing emcc
    do_exec(emcc, with_output)
}

fn do_exec(process: ProcessBuilder, with_output: bool) -> Result<Option<ProcessOutput>, ProcessError> {
    if with_output {
        process.exec_with_output().map(|o| Some(o))
    } else {
        process.exec().map(|_| None)
    }
}
