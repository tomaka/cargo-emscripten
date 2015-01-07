extern crate git2;

use std::io::TempDir;
use std::io::fs::{File, PathExtensions};
use std::io::process;

fn main() {
    let out_dir = Path::new(std::os::getenv("OUT_DIR").unwrap());
    let rust_dir = out_dir.join("rust");

    println!("Cloning Rust");
    clone_rust(&rust_dir);

    println!("Compiling libcore");
    compile(&rust_dir.join("src").join("libcore").join("lib.rs"), &out_dir);

    println!("Compiling liblibc");
    compile(&rust_dir.join("src").join("liblibc").join("lib.rs"), &out_dir);
    
    println!("Compiling liballoc");
    compile(&rust_dir.join("src").join("liballoc").join("lib.rs"), &out_dir);

    println!("Compiling libunicode");
    compile(&rust_dir.join("src").join("libunicode").join("lib.rs"), &out_dir);

    println!("Compiling libcollections");
    compile(&rust_dir.join("src").join("libcollections").join("lib.rs"), &out_dir);

    println!("Compiling librand");
    compile(&rust_dir.join("src").join("librand").join("lib.rs"), &out_dir);

    /*println!("Compiling rustrt");
    File::create(&out_dir.join("rustrt.rs")).write_str("#![no_std]\n#![crate_name = \"rust_builtin\"]\n#![crate_type = \"staticlib\"]\n").unwrap();
    compile(&out_dir.join("rustrt.rs"), &out_dir);

    println!("Compiling libstd");
    compile(&rust_dir.join("src").join("libstd").join("lib.rs"), &out_dir);*/
}

fn clone_rust(path: &Path) -> git2::Repository {
    if path.exists() {
        match git2::Repository::open(path) {
            Ok(r) => {
                if !r.is_empty().unwrap() {
                    return r;
                }
            },
            _ => ()
        };

        std::io::fs::rmdir_recursive(path).unwrap();
    }

    std::io::fs::mkdir(path, std::io::USER_RWX).unwrap();
    git2::Repository::clone("https://github.com/rust-lang/rust#ad9e75938", path).unwrap()
}

fn compile(krate: &Path, out_dir: &Path) {
    let cwd = TempDir::new("cargo-emscripten-compilation").unwrap();
    std::io::fs::copy(&Path::new(std::os::getenv("CARGO_MANIFEST_DIR").unwrap()).join("specs-build.json"),
                      &cwd.path().join("emscripten.json")).unwrap();

    let mut command = process::Command::new("rustc");
    command.arg(krate);
    command.arg("--out-dir").arg(out_dir);
    command.arg("-L").arg(out_dir);
    command.arg("--target").arg("emscripten.json");
    command.cwd(cwd.path());

    let mut ir_command = command.clone();
    ir_command.arg("--emit=llvm-ir");

    exec(ir_command);
    exec(command);
}

fn exec(mut cmd: process::Command) {
    cmd.stdout(process::StdioContainer::InheritFd(1));
    cmd.stderr(process::StdioContainer::InheritFd(2));

    let cmd_str = cmd.to_string();

    if !cmd.status().unwrap().success() {
        panic!("Error while executing `{}`", cmd_str);
    }
}
