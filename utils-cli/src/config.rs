use anyhow::Result;
use configparser::ini::Ini;
use std::io::prelude::*;
use std::{fs::OpenOptions, io::BufReader};

use crate::{CONFIG_FILE, SOURCE_DIR};
pub struct Config;

#[allow(dead_code)]
impl Config {
    /// return the config path
    pub fn path() -> Result<String> {
        // Ok(CONFIG_FILE.as_str().to_string())
        Ok(CONFIG_FILE.as_str().to_string())
    }

    /// create a new config if not exist
    pub fn init() -> Result<()> {
        //  https://stackoverflow.com/questions/35636742/is-there-any-way-to-create-and-open-a-file-if-it-doesnt-exist-but-fail-otherwis
        // write the config file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(CONFIG_FILE.as_str())
            .unwrap();

        // read the default config file and write to the config file
        let default_config = SOURCE_DIR.get_file("utils.conf").unwrap();
        let default_config = default_config.contents_utf8().unwrap();
        file.write_all(default_config.as_bytes()).unwrap();

        Ok(())
    }

    /// read a fied from the config
    /// 
    /// read the config
    /// #example
    /// ```rust
    ///  let rr: Option<String>=  config::Config::parse_key("port", "server")?;
    ///  print!("{:?}", rr);
    /// ````
    pub fn parse_key(key: &str, section: &str) -> Result<Option<String>> {
        let raw_config_file = Self::load()?;
        let mut config = Ini::new();

        // You can easily load a file to get a clone of the map:
        let _ = config.read(raw_config_file);

        Ok(config.get(section, key))
    }
/// read the confog file to string 
    fn load() -> Result<String> {
        // read the config file
        let config = OpenOptions::new()
            .read(true)
            .open(CONFIG_FILE.as_str())
            .unwrap();

        // read the config file
        let mut reader = BufReader::new(config);
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).unwrap();

        Ok(buffer)
    }
}
