use cc;

fn main() {
    cc::Build::new().cpp(true).file("src/tcpp.cpp").warnings(false).compile("tcpp")
}
