#![allow(non_snake_case)]
use std::{env, fs, io::Write, path::PathBuf};

fn main() {
    let mut buffer = Vec::new();
    writeln!(
        &mut buffer,
        "//! A collection of [`EightBit`][crate::EightBit]
//! [`RedGreenBlue`][crate::RedGreenBlue] \
         palettes derived from Kristen M. Thyng
//! et al's [_cmocean_ color maps for oceanography](http://dx.doi.org/10.5670/oceanog.2016.66).\
         "
    )
    .unwrap();

    let mut dir = PathBuf::from(env::args_os().nth(1).unwrap());
    dir.push("cmocean");
    dir.push("rgb");
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            continue;
        }
        let full_name = entry.file_name().to_str().unwrap().to_owned();
        if !full_name.ends_with("-rgb.txt") {
            continue;
        }
        let name = full_name.strip_suffix("-rgb.txt").unwrap();
        let NAME = heck::ToShoutySnakeCase::to_shouty_snake_case(name);

        let data = fs::read_to_string(entry.path()).unwrap();

        let bytes: Vec<u8> = data
            .split_ascii_whitespace()
            .filter(|s| !s.is_empty() && s.chars().all(|c| c.is_ascii()))
            .map(|s| (s.parse::<f32>().unwrap() * 255.).round().clamp(0., 255.) as u8)
            .collect();

        writeln!(&mut buffer, "/// _{name}_").unwrap();
        writeln!(&mut buffer, "pub static {NAME}: &[u8] = &[").unwrap();

        for (i, byte) in bytes.iter().enumerate() {
            if i % 16 == 0 {
                write!(&mut buffer, "    ").unwrap();
            }
            write!(&mut buffer, "0x{byte:02x}, ").unwrap();
            if i % 16 == 15 {
                writeln!(&mut buffer).unwrap();
            }
        }

        writeln!(&mut buffer, "];\n").unwrap();
    }

    fs::write("src/png/palettes/oceanographic.rs", buffer).unwrap();
}
