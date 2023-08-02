use std::os::fd::AsRawFd;
fn main() {
    let mut stdin = std::io::stdin();
    let mut termios = termios::Termios::from_fd(stdin.as_raw_fd()).expect("Termios struct");
    termios.c_cc[termios::VMIN] = 1;
}
