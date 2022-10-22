/**
 * Demonstrates an issue with borrowing; The string is borrowed and then its ownership is given to the dbg! macro, after this the borrowed instance is used. This violates the borrow checker.
 *
 * This prevents the borrowed instance from being modified after its allocated and then used. This likely causes unexpected errors?. TODO: More info??
 *  - Bad
 */
fn error() {
    let string_literal = "1234";
    let string = String::from("127.0.0.1:8080");
    let string_slice = &string[10..];

    dbg!(string);
    dbg!(string_slice);
}
