use zipng::{
    dev::{init, save},
    panic, Png,
};

fn main() -> Result<(), panic> {
    init!();

    save!({ Png::from_unstructured_bytes(&fill(32)).write_vec()? } / 32.png)?;
    save!({ Png::from_unstructured_bytes(&fill(512)).write_vec()? } / 512.png)?;
    save!({ Png::from_unstructured_bytes(&fill(2048)).write_vec()? } / 2048.png)?;
    save!({ Png::from_unstructured_bytes(&fill(1048576)).write_vec()? } / big1048576.png)?;

    Ok(())
}

fn fill(n: usize) -> Vec<u8> {
    let mut v = vec![0u8; n];
    for i in 0..n {
        v[i] = (i.count_ones() as u8)
            .wrapping_add((i as u8).wrapping_add(*v.get(i.saturating_sub(1)).unwrap_or(&13)))
            ^ *v.get(i.saturating_sub(4)).unwrap_or(&0);
    }
    v
}

#[test]
fn test() {
    main().unwrap()
}
