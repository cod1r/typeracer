use std::io::Read;
use std::io::Write;
use std::os::fd::AsRawFd;
fn main() {
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
    let _ = stdout.write(br#"OH MAY
        sdfasdfasdfasdf
        sdfasdfasdfasdfasdf
        asdfa
        sdf
        asdfas
        df
        asdf
        as
        fd
        asdf
        as
        fd
        asdf
        a"#);
    let _ = stdout.write(b"\x1b[H");
    let _ = stdout.flush();
    let mut buf = [0; 1];
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
