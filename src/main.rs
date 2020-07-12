use std::env;
use wrapgen::WrapGen;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Invoke wrapgen with `wrapgen <input.rs> <output.rs>`");
        return;
    }

    WrapGen::new(&args[1])
        .prefix("rs_")
        .use_core(true)
        .generate(&args[2]);
}
