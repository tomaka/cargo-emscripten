use std::io::fs::File;

static LIBCOLLECTIONS: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/collections.ll"));
static LIBCORE: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/core.ll"));
static LIBLIBC: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/libc.ll"));
static LIBRAND: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/rand.ll"));
static LIBSTD: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/std.ll"));
static LIBUNICODE: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/unicode.ll"));

pub fn write_std(path: &Path) -> (Vec<Path>, Vec<(&'static str, Path)>) {
    File::create(&path.join("collections.ll")).write(LIBCOLLECTIONS).unwrap();
    File::create(&path.join("core.ll")).write(LIBCORE).unwrap();
    File::create(&path.join("libc.ll")).write(LIBLIBC).unwrap();
    File::create(&path.join("rand.ll")).write(LIBRAND).unwrap();
    File::create(&path.join("std.ll")).write(LIBSTD).unwrap();
    File::create(&path.join("unicode.ll")).write(LIBUNICODE).unwrap();

    File::create(&path.join("libcollections.rlib")).write(
                include_bytes!(concat!(env!("OUT_DIR"), "/libcollections.rlib"))).unwrap();
    File::create(&path.join("libcore.rlib")).write(
                include_bytes!(concat!(env!("OUT_DIR"), "/libcore.rlib"))).unwrap();
    File::create(&path.join("liblibc.rlib")).write(
                include_bytes!(concat!(env!("OUT_DIR"), "/liblibc.rlib"))).unwrap();

    (
        vec![
            path.join("collections.ll"),
            path.join("core.ll"),
            path.join("libc.ll"),
            path.join("rand.ll"),
            path.join("std.ll"),
            path.join("unicode.ll"),
        ],
        vec![
            ("collections", path.join("libcollections.rlib")),
            ("core", path.join("libcore.rlib")),
            ("libc", path.join("liblibc.rlib"))
        ]
    )
}
