//use std::env;
//use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/parse.cpp");

    cc::Build::new()
        .cpp(true)
        .file("src/parse.cpp")
        .compile("libparse.a");
}
