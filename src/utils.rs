use std::error::Error;
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

pub fn hex_to_rgb<T: AsRef<str>>(hex: &T) -> Result<(u8, u8, u8), Box<dyn Error>> {
    let hex_ref = hex.as_ref();
    let digit = &hex_ref[1..hex_ref.len()];

    let (red, rest) = digit.split_at(2);
    let (green, blue) = rest.split_at(2);

    Ok((
        u8::from_str_radix(red, 16)?,
        u8::from_str_radix(green, 16)?,
        u8::from_str_radix(blue, 16)?,
    ))
}

pub fn parse_config_hex(hex: &str, default: (u8, u8, u8)) -> (u8, u8, u8) {
    match hex.is_empty() {
        false => match hex_to_rgb(&hex) {
            Ok(value) => value,
            Err(_) => default,
        },
        true => default,
    }
}
