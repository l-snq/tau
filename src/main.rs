use std::fs::File;
use std::io::{Error, Write};
use xdg;

fn main() -> Result<(), Error> {

    let xdg_dir = xdg::BaseDirectories::with_prefix("firefox").unwrap();

    let config_path = xdg_dir
        .place_config_file("config.ini")
        .expect("cant create your config");
    let mut config_file = File::create(config_path)?;
    write!(&mut config_file, "configured = 1")?;
    let config_match = match config_file() {
        Ok(config_file) => config_file,
        Err(e) => return Err(e),

    };
    return config_match;
}
