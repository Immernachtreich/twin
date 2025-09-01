use std::{ fs, io::{ self, Result, Write }, path::PathBuf };

use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub storage_path: PathBuf,
}

impl Config {
    fn get_config_path() -> PathBuf {
        let config_dir = dirs
            ::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("twin");

        fs::create_dir_all(&config_dir).expect("Failed to create config directory");

        config_dir.join("config.toml")
    }

    pub fn load() -> Result<Config> {
        let config_path = Config::get_config_path();

        println!("🔎 Reading config from: {}", config_path.display());

        if config_path.exists() {
            let file_contents = fs::read_to_string(&config_path)?;
            let config: Config = toml
                ::from_str(&file_contents)
                .expect("Failed to parse config file");

            Ok(config)
        } else {
            println!("🐣 No config file found, creating a new config file.");

            println!("💿 Please enter the storage path for storing your notes");
            io::stdout().flush().unwrap();

            let mut storage_path: String = String::new();
            io::stdin().read_line(&mut storage_path).unwrap();

            // Convert string to PathBuf
            let storage_path_buf: PathBuf = PathBuf::from(storage_path.trim().to_string());

            // Make sure path exists, if not create it.
            fs::create_dir_all(&storage_path_buf).expect("Failed to storage directory");

            let config = Config { storage_path: storage_path_buf };
            let config_toml = toml::to_string(&config).unwrap();
            let mut file = fs::File::create(config_path)?;

            file.write_all(config_toml.as_bytes())?;

            Ok(config)
        }
    }
}
