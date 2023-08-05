use std::env;
use std::io::Read;
use std::io::Write;
use std::os::fd::AsRawFd;
fn main() {
    let mut args_iter = env::args();
    let _ = args_iter.next();
    let mut args = Vec::<String>::new();
    while let Some(arg) = args_iter.next() {
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
            return;
        }
    }
    let _ = stdout.write(b"\x1b[?1049h\x1b[H\x1b[2J");
    let _ = stdout.write(&bytes);
    let _ = stdout.write(b"\x1b[H");
    let _ = stdout.flush();
    let mut buf = [0; 64];
    let mut sum_bytes = 0;
    'outer: while sum_bytes < bytes.len() {
        let n_read_from_stdin = match stdin.read(&mut buf) {
            Ok(n) => n,
            Err(_) => {
                eprintln!("reading from stdin failed.");
                return;
            }
        };
        for idx in 0..n_read_from_stdin {
            match buf[idx] {
                8 => {
                    if sum_bytes > 0 {
                        sum_bytes -= 1;
                        let _ = stdout.write(b"\x1b[1D");
                        let _ = stdout.flush();
                        continue;
                    }
                }
                b'\n' => {}
                b'\x1b' => {
                    break 'outer;
                }
                _ => {}
            }
            if bytes[sum_bytes + idx] != buf[idx] {
                let _ = stdout.write(b"\x1b[48;2;255;;m");
            } else {
                sum_bytes += 1;
            }
            let _ = stdout.write(&buf[idx..idx + 1]);
            let _ = stdout.write(b"\x1b[0m");
            let _ = stdout.flush();
        }
    }
    termios.c_lflag |= termios::ICANON;
    termios.c_lflag |= termios::ECHO;
    match termios::tcsetattr(stdin.as_raw_fd(), termios::TCSANOW, &mut termios) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("tcsetattr failed.");
            return;
        }
    }
    let _ = stdout.write(b"\x1b[2J");
}
