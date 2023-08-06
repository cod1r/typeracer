use std::env;
use std::io::Read;
use std::io::Write;
fn main() {
    let mut args_iter = env::args();
    let _ = args_iter.next();
    let mut args = Vec::<String>::new();
    while let Some(arg) = args_iter.next() {
        args.push(arg);
    }
    let mut bytes: Vec<u8>;
    match args.get(0) {
        Some(a) => match a.as_str() {
            "--help" => {
                println!(
                    "{}",
                    r#"
    The program takes in one argument, which is the file to read from.
    The requirements of the file is that it has to be ascii.
    Ex:
        typeracer <file>
"#
                );
                return;
            }
            _ => match std::fs::read(a) {
                Ok(bv) => bytes = bv,
                Err(_) => {
                    eprintln!("reading {} failed", a);
                    return;
                }
            },
        },
        None => {
            println!(
                "{}",
                r#"
    The program takes in one argument, which is the file to read from.
    The requirements of the file is that it has to be ascii.
    Ex:
        typeracer <file>
"#
            );
            return;
        }
    }
}
