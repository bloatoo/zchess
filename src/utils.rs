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

pub fn fmt_clock<'a>(time: u64) -> String {
    let sec = (time as f32 / 1000.0).floor();
    let min = (sec / 60.0).floor();
    let sec = sec - (min * 60.0);

    let min_str;

    if (min as u32) < 10 {
        min_str = format!("0{}", min as u32);
    } else {
        min_str = (min as u32).to_string();
    }

    let sec_str;

    if (sec as u32) < 10 {
        sec_str = format!("0{}", sec as u32);
    } else {
        sec_str = (sec as u32).to_string();
    }

    format!("{}:{}", min_str, sec_str)
}
