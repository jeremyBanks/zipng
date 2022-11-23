use zipng::SORT_BY_BODY;
use zipng::SORT_BY_NAME;
use zipng::{
    dev::{init, save},
    panic, Png, Zip, ZipEntry, SORT_BY_SIZE,
};

fn main() -> Result<(), panic> {
    init!();

    let mut zip = Zip::new_from_path(".\\data\\alpine-minirootfs-3.17.0-x86_64")?;

    zip.files.sort_by(|a, b| {
        SORT_BY_SIZE(
            &ZipEntry {
                name: &a.0,
                body: &a.1,
            },
            &ZipEntry {
                name: &b.0,
                body: &b.1,
            },
        )
    });

    let zipng = Png::from_unstructured_bytes(&zip.write_vec()?);

    save!({ zipng.write_vec()? }.zip.png)
}

#[test]
fn test() {
    main().unwrap()
}