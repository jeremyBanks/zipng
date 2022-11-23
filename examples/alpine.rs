use zipng::{
    dev::{init, save},
    panic, Png, Zip, ZipEntry, Zipng, SORT_BY_BODY, SORT_BY_NAME, SORT_BY_SIZE,
};

fn main() -> Result<(), panic> {
    init!();

    let mut zip = Zip::new_from_path(".\\data\\alpine-minirootfs-3.17.0-x86_64")?;

    zip.sort_by(SORT_BY_SIZE);

    let zipng = Zipng::new(&zip);

    save!({ zipng.write_vec()? }.zip.png)
}

#[test]
fn test() {
    main().unwrap()
}
