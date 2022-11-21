use zipng::{
    dev::{init, save},
    panic, Png,
};

fn main() -> Result<(), panic> {
    init!();

    save!({ Png::default().write_vec()? }.png)
}

#[test]
fn test() {
    main().unwrap()
}
