use zipng::{
    dev::{init, save},
    palettes::singles::TURBO,
    panic,
    BitDepth::EightBit,
    Png,
};

fn main() -> Result<(), panic> {
    init!();

    let mut png = Png::new_indexed(512, 128, EightBit, TURBO);

    for y in 0..png.height {
        for x in 0..png.width {
            png.set_pixel(x, y, &[(x * 4 / 9 + x.abs_diff(y) / 15).min(255) as u8])?;
        }
    }

    save!({ png.write_vec()? }.png)
}

#[test]
fn test() {
    main().unwrap()
}
