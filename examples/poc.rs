use zipng::{
    byte_buffer,
    dev::{init, save},
    palettes::singles::TURBO,
    panic, poc_zipng,
    BitDepth::EightBit,
    Png,
};

fn main() -> Result<(), panic> {
    init!();

    save!({ poc_zipng()? }.png.zip)
}

#[test]
fn test() {
    main().unwrap()
}
