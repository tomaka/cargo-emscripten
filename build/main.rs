extern crate git2;

fn main() {
    let out_dir = Path::new(std::os::getenv("OUT_DIR").unwrap());
    let rust_dir = out_dir.join("rust");

    clone_rust(&rust_dir);
}

fn clone_rust(path: &Path) -> git2::Repository {
    use std::io::fs::PathExtensions;

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
    git2::Repository::clone("https://github.com/rust-lang/rust", path).unwrap()
}
