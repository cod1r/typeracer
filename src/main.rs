use std::env;
use std::io::Read;
use std::io::Write;
use std::os::fd::AsRawFd;
fn main() {
    let _ = env::args().next();
    let mut args = Vec::<String>::new();
    while let Some(arg) = env::args().next() {
        args.push(arg);
    }
    let mut bytes = Vec::<u8>::new();
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
            }
            _ => {
                match std::fs::read(a) {
                    Ok(bv) => bytes = bv,
                    Err(_) => {
                        eprintln!("reading {} failed", a);
                    }
                }
            }
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
        }
    }
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut termios = termios::Termios::from_fd(stdin.as_raw_fd()).expect("Termios struct");
    termios.c_cc[termios::VMIN] = 1;
    termios.c_cc[termios::VTIME] = 0;
    termios.c_lflag &= !termios::ICANON;
    termios.c_lflag &= !termios::ECHO;
    match termios::tcsetattr(stdin.as_raw_fd(), termios::TCSANOW, &mut termios) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("tcsetattr failed.");
        }
    }
    let _ = stdout.write(b"\x1b[?1049h\x1b[H\x1b[2J");
    let _ = stdout.write(b"\x1b[H");
    let _ = stdout.flush();
    let mut buf = [0; 64];
    let _ = stdin.read(&mut buf);
    termios.c_lflag |= termios::ICANON;
    termios.c_lflag |= termios::ECHO;
    match termios::tcsetattr(stdin.as_raw_fd(), termios::TCSANOW, &mut termios) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("tcsetattr failed.");
        }
    }
    let _ = stdout.write(b"\x1b[2J");
}
