use zipng::{
    byte_buffer,
    dev::{init, save},
    palettes::{
        oceanic::{BALANCE, GRAY, TOPO},
        singles::TURBO,
        viridis::VIRIDIS,
    },
    panic, poc_zipng,
    BitDepth::EightBit,
    Png,
};

fn main() -> Result<(), panic> {
    init!();

    save!({ poc_zipng(TURBO)? } - turbo.png.zip)?;
    save!({ poc_zipng(VIRIDIS)? } - viridis.png.zip)?;
    save!({ poc_zipng(BALANCE)? } - balance.png.zip)?;
    save!({ poc_zipng(GRAY)? } - gray.png.zip)?;
    save!({ poc_zipng(TOPO)? } - topo.png.zip)?;

    Ok(())
}

#[test]
fn test() {
    main().unwrap()
}
