use {
    std::io::{Cursor, Read},
    tracing::info,
    zipng::{
        output_buffer,
        dev::{init, save},
        palettes::{
            crameri::{BATLOW, ROMA_O},
            oceanic::{BALANCE, GRAY, TOPO},
            singles::TURBO,
            viridis::VIRIDIS,
        },
        panic, poc_zipng,
        BitDepth::EightBit,
        Png,
    },
};

fn main() -> Result<(), panic> {
    init!();

    save!({ poc_zipng(TURBO)? } - turbo.png.zip)?;
    save!({ poc_zipng(BATLOW)? } - batlow.png.zip)?;
    save!({ poc_zipng(ROMA_O)? } - roma_o.png.zip)?;
    save!({ poc_zipng(VIRIDIS)? } - viridis.png.zip)?;
    save!({ poc_zipng(BALANCE)? } - balance.png.zip)?;
    save!({ poc_zipng(GRAY)? } - gray.png.zip)?;
    save!({ poc_zipng(TOPO)? } - topo.png.zip)?;

    // let mut zip = ::zip::ZipArchive::new(Cursor::new(poc_zipng(TURBO)?))?;
    // let names: Vec<String> = zip.file_names().map(|s| s.to_string()).collect();
    // for (index, name) in names.iter().enumerate() {
    //     let mut content_by_index = zip.by_index(index)?;
    //     let name_by_index = content_by_index.name().to_string();
    //     let mut body_by_index = Vec::new();
    //     content_by_index.read_to_end(&mut body_by_index)?;
    //     drop(content_by_index);

    //     let mut content_by_name = zip.by_name(name)?;
    //     let name_by_name = content_by_name.name().to_string();
    //     let mut body_by_name = Vec::new();
    //     content_by_name.read_to_end(&mut body_by_name)?;
    //     drop(content_by_name);

    //     info!(
    //         index = index,
    //         name = name,
    //         name_by_index = name_by_index,
    //         name_by_name = name_by_name,
    //     );
    // }

    Ok(())
}

#[test]
fn test() {
    main().unwrap()
}
