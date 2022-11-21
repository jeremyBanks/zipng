use {
    std::fs::read,
    zipng::{
        dev::{init, save},
        panic, Png,
    },
};

fn main() -> Result<(), panic> {
    init!();

    let input = read("src/text/sixth.png")?;

    let png = Png::read_slice(&input)?;

    save!({ png.write_vec()? }.png)
}

#[test]
fn test() {
    main().unwrap()
}
