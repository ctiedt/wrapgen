use std::env;
use wrapgen::WrapGen;

fn main() {
    let args: Vec<String> = env::args().collect();

    WrapGen::new(&args[1])
        .prefix("rs_")
        .use_core(true)
        .generate(&args[2]);
}
