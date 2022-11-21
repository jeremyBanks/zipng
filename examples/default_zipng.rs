use zipng::{
    dev::{init, save},
    panic,
};

fn main() -> Result<(), panic> {
    init!();

    let bytes = b""; // Zipng::default().write_vec()?;

    save!(bytes.zip.png)
}

#[test]
fn test() {
    main().unwrap()
}
