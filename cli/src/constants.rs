use include_dir::{include_dir, Dir};

pub const SOURCE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

lazy_static::lazy_static! {
    pub static ref DB_URL: std::string::String = {
        /* create "utils" directory in the home dir and / save files to $HOME utils
        * this would hold the sqlite database which would contain configuration and app data*/

        let os_default_home_dir = dirs::home_dir().unwrap();
        let db_path = format!(
            "{home_dir}/{upload_dir}",
            home_dir = os_default_home_dir.display(),
            upload_dir = ".utils"
        );

        // sea-orm-cli generate entity -u sqlite:///Users/USER/.utils/utils.db -o entity/src
        // create the path if not exist path if not exist
        let _ = std::fs::create_dir_all(&db_path);
    format!("sqlite://{db_path}/utils.db")
    };

    // the path to the config file
    pub static ref CONFIG_FILE: std::string::String = {
        let os_default_home_dir = dirs::home_dir().unwrap();
        let config_path = format!(
            "{home_dir}/{upload_dir}",
            home_dir = os_default_home_dir.display(),
            upload_dir = ".utils"
        );

        // create the path if not exist path if not exist
        let _ = std::fs::create_dir_all(&config_path);
        format!("{config_path}/utils.conf")
    };
}