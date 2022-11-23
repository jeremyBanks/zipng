use zipng::{
    dev::{init, save},
    palettes::mappings::BIT_COUNT,
    panic, EightBit, Png,
};

fn main() -> Result<(), panic> {
    init!();

    let mut png = Png::new_grayscale(512, 128, EightBit);

    for y in 0..png.height {
        for x in 0..png.width {
            png.set_pixel(
                x,
                y,
                &[BIT_COUNT[(x * 4 / 9 + x.abs_diff(y) / 15).min(255)]],
            )?;
        }
    }

    save!({ png.write_vec()? }.png)
}

#[test]
fn test() {
    main().unwrap()
}
