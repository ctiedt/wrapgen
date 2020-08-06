use wrapgen::{WrapGen, WrapperType};

fn main() {
    WrapGen::default()
        .wrap_pointer_type(
            WrapperType::new("inode", "Inode")
                .with_field("i_sb", "*mut super_block")
                .with_field_writeonly("i_ino", "cty::c_int"),
        )
        .wrap_pointer_type(WrapperType::new("dentry", "Dentry"))
        .add_function("fn get_inode(sb: *mut super_block) -> *mut inode")
        .add_function("fn c_dget(dentry: *mut dentry) -> *mut dentry;")
        .generate("outfile.rs")
}
