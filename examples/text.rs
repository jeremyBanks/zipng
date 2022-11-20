use {
    bitvec::{
        prelude::{Lsb0, Msb0},
        vec::BitVec,
        view::AsBits,
    },
    zipng::*,
};

fn main() -> Result<(), panic> {
    let text = "Hello, world!";
    // let bytes = text_png(text);

    // eprintln!(
    //     "Rendered {} characters of text in {} bytes of PNG.",
    //     text.chars().count(),
    //     bytes.len()
    // );

    // std::fs::write("target/out.png", bytes)?;

    /*
    let mut data = Vec::new();

    let font = Mini5pt;
    let width = font.width();
    let bits = font.width() * font.height();
    for (character, bitmap) in font.glyphs() {
        // if *character != 'A' {
        //     continue;
        // }
        let mut v: Vec<_> = bitmap
            .to_le_bytes()
            .as_bits::<Lsb0>()
            .iter()
            .take(bits + 1)
            .map(|bit| if *bit { 0xFFu8 } else { 0x00 })
            .collect();
        v.reverse();
        let invisible = v.remove(0) != 0;

        if invisible {
            v.fill(0);
        }
        data.extend(v);
        data.extend(vec![0; width]);
        data.extend(vec![0; width]);
    }

    let mut buffer = Vec::new();

    let ZipngOptions {
        png:
            PngOptions {
                bit_depth,
                color_mode,
                color_palette,
                // width,
                ..
            },
        ..
    } = ZipngOptions::default_for_data(&data);
    let color_palette = color_palette.as_deref();

    let bit_depth = EightBit;
    let color_mode = Indexed;
    let color_palette = Some(PALLETTE_8_BIT_DATA.as_slice());

    // let bit_depth = EightBit;
    // let color_mode = RedGreenBlue;
    // let color_palette = None::<&[u8]>;

    // let bit_depth = OneBit;
    // let color_mode = Indexed;
    // let color_palette = [0xFF_u8, 0xFF, 0xEE, 0x11, 0x11, 0x33];
    // let color_palette = Some(&color_palette[..]);

    let bits_per_pixel = bit_depth.bits_per_sample() * color_mode.samples_per_pixel();
    let pixels = data.len() * 8 / bits_per_pixel;

    let height = (pixels / width).max(1);

    write_png(
        &mut buffer,
        &data,
        width as u32,
        height as u32,
        bit_depth,
        color_mode,
        color_palette,
    );

    std::fs::write("target/test.png", buffer)?;
     */

    Ok(())
}
