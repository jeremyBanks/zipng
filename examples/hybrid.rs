use zipng::{
    dev::{init, save},
    panic, Png, Zip,
};

fn main() -> Result<(), panic> {
    init!();

    let zip = Zip {
        files: vec![
            (
                b"Cargo.lock".to_vec(),
                include_bytes!("../Cargo.lock").to_vec(),
            ),
            (
                b"sqlite_zstd.dll".to_vec(),
                include_bytes!("../sqlite_zstd.dll").to_vec(),
            ),
            (
                b"indexed8bit.png".to_vec(),
                include_bytes!("../test_data/indexed8bit.png").to_vec(),
            ),
        ],
        ..Zip::default()
    };

    // let zipng = Zipng::new(&zip);
    let zipng = Png::from_unstructured_bytes(&zip.write_vec()?);

    save!({ zipng.write_vec()? }.zip.png)
}

#[test]
fn test() {
    main().unwrap()
}
