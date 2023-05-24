use std::path::PathBuf;

struct Config {
    data_file: PathBuf
}

fn main() {
    dbn::decode::dbn::Decoder::from_zstd_file(path)
}