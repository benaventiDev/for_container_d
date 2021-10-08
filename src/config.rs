use serde::Deserialize;
use config::ConfigError;
use std::path::PathBuf;


#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize)]
pub struct MySqlConfig {
    pub user:  String,
    pub password: String,
    pub host: String,
    pub port: i32,
    pub db_name: String

}

#[derive(Deserialize)]
pub struct CosmosConfig {
    pub account:  String,
    pub key: String,
    pub db_name: String,
    pub container: String

}

#[derive(Deserialize)]
pub struct Config{
    pub server: ServerConfig,
    pub sql_server: MySqlConfig,
    pub cosmos: CosmosConfig
}

impl Config{
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}

pub struct Path {
    pub prefix: String
}

impl Path {

    /// This function defines a full path based on the struct's prefix and the String passed in.
    ///
    /// # Arguments
    /// * following_path (String): the rest of the path to be appended to the self.prefix
    ///
    /// # Use
    /// To use this in a route, we have to reference it:
    ///
    /// ```rust
    /// let path = Path{base: String::from("/base/")};
    /// app.route(&path.define(String::from("tail/path")), web::get().to(login::login))
    /// ```
    pub fn define(&self, following_path: String) -> String {
        return self.prefix.to_owned() + &following_path
    }
}




