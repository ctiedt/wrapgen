use wrapgen::WrapGen;

fn main() {
    WrapGen::new("examples/to_wrap.rs").generate("outfile.rs");
}
