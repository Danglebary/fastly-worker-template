use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum AppEnv {
    STAGING,
    PRODUCTION,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AppConfig {
    pub FASTLY_SERVICE_VERSION: String,

    pub app_name: String,
    pub app_version: String,
    pub app_env: AppEnv,
}

pub fn init() -> AppConfig {
    match envy::from_env::<AppConfig>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    }
}
