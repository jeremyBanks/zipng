use {bytemuck::bytes_of, std::fs, zipng::*};

fn main() -> Result<(), panic> {
    let mut png = Png::new(&(
        (128, 128),
        (BitDepth::EightBit, bytes_of(&palettes::EIGHT_BIT_HEAT)),
    ));

    for y in 0..png.height {
        for x in 0..png.width {
            png.set_pixel(x, y, &[(x ^ y) as u8])?;
        }
    }

    let mut f = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("test_data/revolution.png")?;

    png.write(&mut f)?;

    Ok(())
}
