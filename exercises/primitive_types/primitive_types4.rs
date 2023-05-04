// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.

#[test]
fn slice_out_of_array() {
    //just to make sure about how many bits i want to locate
    let a:[i32;5] = [1, 2, 3, 4, 5];
    // this is first hard part i face so far 
    // the conecpt is about ownership 
    // run hint in your terminal and it will direct you to rust-lang docs
    let nice_slice:&[i32] = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
