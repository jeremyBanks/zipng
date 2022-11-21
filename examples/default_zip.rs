use zipng::{
    dev::{init, save},
    panic, Zip,
};

fn main() -> Result<(), panic> {
    init!();

    save!({ Zip::default().write_vec()? }.zip)
}

#[test]
fn test() {
    main().unwrap()
}
