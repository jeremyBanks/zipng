use zipng::{
    dev::{init, save},
    palettes::{viridis::INFERNO, MAP_BIT_COUNT},
    panic,
    BitDepth::EightBit,
    Png, Zip, Zipng,
};

fn main() -> Result<(), panic> {
    init!();

    let zip = Zip::default();

    // let zipng = Zipng::new(&zip);
    let zipng = Png::from_unstructured_bytes(&zip.write_vec()?);

    save!({ zipng.write_vec()? }.zip.png)
}

#[test]
fn test() {
    main().unwrap()
}
