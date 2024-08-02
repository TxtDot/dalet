use dalet::{
    abstractions::{HeadingLevel, Tag, ToDaletl},
    daletpack::*,
};
use flate2::Compression;
use std::io::Write;

#[macro_export]
macro_rules! iprint {
    ($name:expr, $func:expr) => {{
        let start = std::time::Instant::now();
        let result = $func;
        let elapsed = start.elapsed();
        println!("{} ({:#?}): {} bytes", $name, elapsed, result.len());

        result
    }};
}

pub fn compress_deflate(data: &Vec<u8>) -> std::io::Result<Vec<u8>> {
    let mut c = flate2::write::DeflateEncoder::new(Vec::new(), Compression::default());
    c.write(data)?;
    c.finish()
}

pub fn compress_zlib(data: &Vec<u8>) -> std::io::Result<Vec<u8>> {
    let mut c = flate2::write::ZlibEncoder::new(Vec::new(), Compression::default());
    c.write(data)?;
    c.finish()
}

#[test]
fn bench() {
    let mut page: Vec<Tag> = vec![
        // Tag::H("I am heading".to_owned(), HeadingLevel::One),
        // Tag::H("Heading 2".to_owned(), HeadingLevel::Two),
    ];

    for i in 0..500 {
        page.push(Tag::H(format!("{}. Heading", i), HeadingLevel::One))
    }

    let dalet_page = page.to_daletl();

    let daletpack = iprint!("Daletpack", encode(&dalet_page).unwrap());
    let messagepack = iprint!("Messagepack", rmp_serde::to_vec(&dalet_page).unwrap());
    let bincode = iprint!("Bincode", bincode::serialize(&dalet_page).unwrap());

    println!();

    iprint!("Daletpack zstd", utils::compress_zstd(&daletpack).unwrap());
    iprint!(
        "Messagepack zstd",
        utils::compress_zstd(&messagepack).unwrap()
    );
    iprint!("Bincode zstd", utils::compress_zstd(&bincode).unwrap());

    println!();

    iprint!("Daletpack Zlib", compress_zlib(&daletpack).unwrap());
    iprint!("Messagepack Zlib", compress_zlib(&messagepack).unwrap());
    iprint!("Bincode Zlib", compress_zlib(&bincode).unwrap());

    println!();

    iprint!("Daletpack deflate", compress_deflate(&daletpack).unwrap());
    iprint!(
        "Messagepack deflate",
        compress_deflate(&messagepack).unwrap()
    );
    iprint!("Bincode deflate", compress_deflate(&bincode).unwrap());

    // fs::write("./test.daletpack", daletpack).unwrap();
}
