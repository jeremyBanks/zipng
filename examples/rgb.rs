use {
    std::fs,
    zipng::{panic, Png},
};

fn main() -> Result<(), panic> {
    let mut png = Png::new_rgb(512, 128);

    for y in 0..png.height {
        for x in 0..png.width {
            png.set_pixel(x, y, &[
                (x / 4) as u8,
                y as u8,
                (x * 4 / 9 + x.abs_diff(y) / 15).min(255) as u8,
            ])?;
        }
    }

    for x in 0..png.height {
        png.set_pixel(png.height * 2 - x, x, &[0x00, 0, 0])?;
        png.set_pixel(png.height * 2 - x + 1, x, &[0xFF, 0, 0])?;
        png.set_pixel(png.height * 2 - x + 2, x, &[0xFF, 0xFF, 0])?;
        png.set_pixel(png.height * 2 - x + 3, x, &[0, 0xFF, 0])?;
        png.set_pixel(png.height * 2 - x + 4, x, &[0, 0xFF, 0xFF])?;
        png.set_pixel(png.height * 2 - x + 5, x, &[0, 0, 0xFF])?;
        png.set_pixel(png.height * 2 - x + 6, x, &[0xFF, 0, 0xFF])?;
        png.set_pixel(png.height * 2 - x + 7, x, &[0xFF, 0xFF, 0xFF])?;
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
