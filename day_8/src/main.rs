use std::io::Write;

fn main() {
    std::io::stdout().write(b"1n").unwrap();
    std::io::stdout().write(b"1n", b"2\n").unwrap();
    std::io::stdout().write(b"1n", b"2\n", b"3\n").unwrap();
}
