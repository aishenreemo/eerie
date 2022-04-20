use crate::Error;

pub struct Settings {
    pub prefix: String,
}

pub fn initialize_config() -> Result<Settings, Error> {
    Ok(Settings {
        prefix: std::env::var("EERIE_PREFIX")?,
    })
}
