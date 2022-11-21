#![allow(non_snake_case)]
use std::{env, fs, io::Write};

fn main() {
    let mut buffer = Vec::new();

    for entry in fs::read_dir(env::args_os().nth(1).unwrap()).unwrap() {
        let entry = entry.unwrap();
        if !entry.file_type().unwrap().is_dir() {
            continue;
        }
        let name = entry.file_name().to_str().unwrap().to_owned();
        if name.starts_with('+') {
            continue;
        }
        let NAME = heck::ToShoutySnakeCase::to_shouty_snake_case(&*name);
        let path = entry.path().to_str().unwrap().to_owned();

        let data = fs::read_to_string(format!("{path}/{name}.tbl")).unwrap();

        let bytes: Vec<u8> = data
            .split_ascii_whitespace()
            .filter(|s| !s.is_empty() && s.chars().all(|c| c.is_ascii_digit()))
            .map(|s| s.parse().unwrap())
            .collect();

        writeln!(
            &mut buffer,
            "//! 256-color RGB palettes derived from Fabio Crameri's _Scientific Color Maps_, \
             v7.0.1."
        )
        .unwrap();

        writeln!(&mut buffer, "/// Fabio Crameri's _{name}_ color map").unwrap();
        writeln!(&mut buffer, "pub static {NAME}: &[u8; 768] = &[").unwrap();

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

    fs::write("src/png/palettes/colormaps.rs", buffer).unwrap();
}
