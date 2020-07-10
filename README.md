# wrapgen

`wrapgen` is a tool to automatically generate Rust wrappers around C functions called via FFI.
It will wrap pointer returns in an `Option` and `int` returns in a `Result`.
As of now, `wrapgen` only works if your functions adhere to the C convention of returning
0 on a successful run and another value otherwise.

## How to use `wrapgen`

You can use wrapgen as a standalone binary:

`wrapgen input.rs output.rs`

where `input.rs` contains one function declaration per line

or include it in your `build.rs` file:

```rust
fn main() {
    WrapGen::new("input1.rs")
        .add_file("input2.rs")
        .function("fn my_test_fn(arg1: cty::c_int) -> cty::c_int")
        .prefix("rs_")
        .use_core(false)
        .generate("output.rs");
}
```