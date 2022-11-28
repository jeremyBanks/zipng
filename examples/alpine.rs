use zipng::{
    byte_buffer,
    dev::{init, save},
    panic, Zip, SORT_BY_SIZE,
};

fn main() -> Result<(), panic> {
    init!();

    let mut zip = Zip::new_from_path(".\\data\\alpine-minirootfs-3.17.0-x86_64")?;

    zip.sort_by(SORT_BY_SIZE);

    let mut buffer = byte_buffer();
    // write_glass_zipng(&zip, &mut buffer)?;

    save!({ buffer.get_ref() }.zip.png)
}

#[test]
fn test() {
    main().unwrap()
}
