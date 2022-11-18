use indexmap::IndexMap;
use zipng::generic::panic;
use zipng::png::write_png;
use zipng::png::BitDepth::EightBit;
use zipng::png::ColorMode::Indexed;
use zipng::png::ColorMode::RedGreenBlue;
use zipng::png::PALLETTE_8_BIT_DATA;
use zipng::zip;

fn main() -> Result<(), panic> {
    let files: [(&[u8], &[u8]); 4] = [
        (b"mimetype".as_ref(), b"zip/zip".as_ref()),
        (b"README.md".as_ref(), b"welcome".as_ref()),
        (b"assets/icon.png".as_ref(), include_bytes!("../icon.png")),
        (
            b"assets/png.exe".as_ref(),
            include_bytes!("../target/debug/examples/example1.exe"),
        ),
    ];
    let files = IndexMap::from_iter(files.iter().map(|(k, v)| (k.to_vec(), v.to_vec())));

    let data = zip(&files.into());

    let mut buffer = Vec::new();

    let bit_depth = EightBit;
    let color_type = Indexed;
    let palette = Some(PALLETTE_8_BIT_DATA.as_slice());

    let bit_depth = EightBit;
    let color_type = RedGreenBlue;
    let palette = None;

    let bits_per_pixel = bit_depth.bits_per_sample() * color_type.samples_per_pixel();
    let pixels = data.len() * 8 / bits_per_pixel;

    let width = 1024;
    let height = pixels as u32 / width;
    write_png(
        &mut buffer,
        &data,
        width,
        height,
        bit_depth,
        color_type,
        palette,
    );

    std::fs::write("target/test.png", buffer)?;

    Ok(())
}
