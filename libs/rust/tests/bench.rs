use dalet::{
    abstractions::{Body, HeadingLevel, NotNullBody, Tag, TextOrNullArgument, ToDaletl},
    daletpack::*,
};
use flate2::Compression;
use std::io::{read_to_string, Write};

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
    let page: Vec<Tag> = vec![
        Tag::H("I am heading".to_owned(), HeadingLevel::One),
        Tag::H("Heading 2".to_owned(), HeadingLevel::Two),
        Tag::El(NotNullBody::Tags(vec![
            Tag::El(NotNullBody::Text("Some ".to_owned())),
            Tag::B("bold".to_owned()),
            Tag::I("italic".to_owned()),
            Tag::S("strike".to_owned()),
        ])),
        Tag::Br,
        Tag::Code("Hello world".to_owned(), TextOrNullArgument::Null),
        Tag::Br,
        Tag::Ol(vec![
            Tag::El(NotNullBody::Text("abc".to_owned())),
            Tag::El(NotNullBody::Tags(vec![
                Tag::El(NotNullBody::Text("def".to_owned())),
                Tag::Ol(vec![
                    Tag::El(NotNullBody::Text("defabc".to_owned())),
                    Tag::El(NotNullBody::Text("defdef".to_owned())),
                ]),
            ])),
            Tag::El(NotNullBody::Text("xyz".to_owned())),
        ]),
        Tag::Br,
        Tag::El(NotNullBody::Tags(vec![
            Tag::El(NotNullBody::Text("Lorem ipsum ".to_owned())),
            Tag::Link(
                Body::Tags(vec![Tag::Img("https://my-picture".to_owned())]),
                "https://some-link".to_owned(),
            ),
            Tag::El(NotNullBody::Text(
                " dolor sit amet consequetur adipiscing elit".to_owned(),
            )),
        ])),
        Tag::Table(vec![
            Tag::Tpcol(vec![
                Tag::El(NotNullBody::Text("Col 1".to_owned())),
                Tag::El(NotNullBody::Text("Col 2".to_owned())),
                Tag::El(NotNullBody::Text("Col 3".to_owned())),
            ]),
            Tag::Tcol(vec![
                Tag::El(NotNullBody::Text("Never gonna".to_owned())),
                Tag::El(NotNullBody::Text("give you".to_owned())),
                Tag::El(NotNullBody::Text("up".to_owned())),
            ]),
        ]),
    ];

    let dalet_page = page.to_daletl();

    let markdown = iprint!("Markdown", include_str!("./bench.md").as_bytes().to_vec());
    let daletpack = iprint!("Daletpack", encode_no_compress(&dalet_page).unwrap());
    let messagepack = iprint!("Messagepack", rmp_serde::to_vec(&dalet_page).unwrap());
    let bincode = iprint!("Bincode", bincode::serialize(&dalet_page).unwrap());

    println!();

    iprint!("Markdown zstd", utils::compress_zstd(&markdown).unwrap());
    iprint!("Daletpack zstd", utils::compress_zstd(&daletpack).unwrap());
    iprint!(
        "Messagepack zstd",
        utils::compress_zstd(&messagepack).unwrap()
    );
    iprint!("Bincode zstd", utils::compress_zstd(&bincode).unwrap());

    println!();

    iprint!("Markdown Zlib", compress_zlib(&markdown).unwrap());
    iprint!("Daletpack Zlib", compress_zlib(&daletpack).unwrap());
    iprint!("Messagepack Zlib", compress_zlib(&messagepack).unwrap());
    iprint!("Bincode Zlib", compress_zlib(&bincode).unwrap());

    println!();

    iprint!("Markdown deflate", compress_deflate(&markdown).unwrap());
    iprint!("Daletpack deflate", compress_deflate(&daletpack).unwrap());
    iprint!(
        "Messagepack deflate",
        compress_deflate(&messagepack).unwrap()
    );
    iprint!("Bincode deflate", compress_deflate(&bincode).unwrap());
}
