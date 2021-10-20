use std::fs::OpenOptions;
use std::io::Write;

pub fn debug<T: AsRef<str>>(data: &T) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("/tmp/chess_debug")
        .unwrap();

    file.write_all(data.as_ref().as_bytes()).unwrap();
}
