use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings
}

#[derive(Deserialize, Clone)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("configuration.rs - unable to determine current directory");
    let configuration_directory = base_path.join("configuration");

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml")
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
}