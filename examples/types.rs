use wrapgen::WrapGen;

fn main() {
    WrapGen::default()
        .wrap_pointer_type("inode", "Inode")
        .generate("outfile.rs")
}
