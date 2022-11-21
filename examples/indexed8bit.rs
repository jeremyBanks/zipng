use {
    bytemuck::bytes_of,
    std::fs,
    zipng::{
        palettes::{EIGHT_BIT_HEAT, EIGHT_BIT_MAPPING},
        panic, EightBit, Png,
    },
};

fn main() -> Result<(), panic> {
    let mut png = Png::new_indexed(512, 128, EightBit, bytes_of(&EIGHT_BIT_HEAT));

    for y in 0..png.height {
        for x in 0..png.width {
            png.set_pixel(x, y, &[
                EIGHT_BIT_MAPPING[(x * 4 / 9 + x.abs_diff(y) / 15).min(255)]
            ])?;
        }
    }

    let example = option_env!("CARGO_CRATE_NAME").unwrap_or("example");

    let mut f = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!("test_data/{example}.png"))?;

    png.write(&mut f)?;

    Ok(())
}

#[test]
fn test() {
    main().unwrap()
}
