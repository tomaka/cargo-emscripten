extern crate git2;

use std::io::fs::PathExtensions;
use std::io::process;

fn main() {
    let out_dir = Path::new(std::os::getenv("OUT_DIR").unwrap());
    let rust_dir = out_dir.join("rust");

    println!("Cloning Rust");
    clone_rust(&rust_dir);

    println!("Compiling libcore");
    if !out_dir.join("core.ll").exists() {
        compile(&rust_dir.join("src").join("libcore").join("lib.rs"), &out_dir);
    }

    println!("Compiling liblibc");
    if !out_dir.join("libc.ll").exists() {
        compile(&rust_dir.join("src").join("liblibc").join("lib.rs"), &out_dir);
    }

    println!("Compiling liballoc");
    if !out_dir.join("alloc.ll").exists() {
        compile(&rust_dir.join("src").join("liballoc").join("lib.rs"), &out_dir);
    }

    println!("Compiling libcollections");
    if !out_dir.join("collections.ll").exists() {
        compile(&rust_dir.join("src").join("libcollections").join("lib.rs"), &out_dir);
    }

    println!("Compiling libunicode");
    if !out_dir.join("unicode.ll").exists() {
        compile(&rust_dir.join("src").join("libunicode").join("lib.rs"), &out_dir);
    }

    println!("Compiling librand");
    if !out_dir.join("rand.ll").exists() {
        compile(&rust_dir.join("src").join("librand").join("lib.rs"), &out_dir);
    }

    println!("Compiling libstd");
    if !out_dir.join("std.ll").exists() {
        compile(&rust_dir.join("src").join("libstd").join("lib.rs"), &out_dir);
    }
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
    let mut command = process::Command::new("rustc");
    command.arg(krate);
    command.arg("--out-dir").arg(out_dir);
    command.arg("--emit=llvm-ir");
    
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
