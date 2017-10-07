extern crate mar;

use std::env::args;

fn main() {
    let mut args = args().skip(1);

    match args.next().as_ref().map(String::as_str) {
        Some("-X") => {
            let path = args.next().expect("-X requires a file name");
            mar::extract::extract(path).unwrap();
        }

        Some(x) => eprintln!("Unrecognized option {:?}", x),

        _ => eprintln!("Usage: [TODO]")
    }
}
