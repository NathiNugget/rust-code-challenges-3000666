use std::any;
use std::any::type_name_of_val;
use std::ffi::CString;
use std::fmt::{Debug, Display};

fn info<T: Debug>(a: &T) -> () {
    println!("{:?}", a);
}

fn main() {
    let a = "?";
    let b = "?".to_string();
    info(&a);
    info(&b);

    // Advanced 1
    use std::ffi::CString;

    let c = CString::new("?").unwrap();
    info(&c);

    // Advanced 2

}

#[test]
fn str() {
    let input = "Rust";
    info(&input);
}

#[test]
fn string() {
    let input = String::from("Rust");
    info(&input);
}

// #[test]
// fn chars() {
//     let input = 'r';
//     info(&input);
// }

// #[test]
// fn cstring() {
//     use std::ffi::{CString};
//     let input = CString::new("Rust").unwrap();
//     info(&input);
// }

// #[test]
// fn path() {
//     use std::path::Path;
//     let input = Path::new("/tmp/rust");
//     info(input);
// }
