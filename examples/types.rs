use wrapgen::{WrapGen, WrapperType};

fn main() {
    WrapGen::default()
        .wrap_pointer_type(
            WrapperType::new("inode", "Inode")
                .with_field("i_sb", "*mut super_block")
                .with_field_writeonly("i_ino", "cty::c_int"),
        )
        .generate("outfile.rs")
}
