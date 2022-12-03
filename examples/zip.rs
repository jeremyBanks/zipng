use zipng::{
    dev::{init, save},
    panic, Zip,
};

fn main() -> Result<(), panic> {
    init!();

    let zip = Zip::new_with_files(vec![
        (b"README.md".to_vec(), b"hello, world?".to_vec()),
        (b"LICENSE.md".to_vec(), b"not applicable".to_vec()),
    ]);

    let output = zip.serialize();
    save!({ output.as_ref() }.zip)?;
    save!({ output.to_string().as_bytes() }.xml)?;
    Ok(())
}

#[test]
fn test() {
    main().unwrap()
}
