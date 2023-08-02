use std::io::Read;
use std::os::fd::AsRawFd;
fn main() {
    let mut stdin = std::io::stdin();
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
}
