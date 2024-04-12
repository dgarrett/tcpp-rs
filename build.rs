use cc;

fn main() {
    println!("cargo:rerun-if-changed=src/");
    cc::Build::new()
        .cpp(true)
        .warnings(false)
        .flag_if_supported("-std=c++14")
        .file("src/tcpp.cpp")
        .compile("tcpp")
}
