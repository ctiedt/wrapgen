fn page_symlink(inode: *mut inode, symname: *const cty::c_char, len: cty::c_int) -> cty::c_int;
fn c_dget(dentry: *mut dentry) -> *mut dentry;
fn test_fn();
