pub fn compress_zstd(data: &Vec<u8>) -> std::io::Result<Vec<u8>> {
    zstd::bulk::compress(data, 5)
}
