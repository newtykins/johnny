pub use anyhow::{Context as AnyhowContext, Result};
#[cfg(dev)]
use dotenvy_macro::dotenv as env;
use serde::{Deserialize, Serialize};
#[cfg(not(dev))]
use std::fs::read_to_string;

#[cfg(db)]
#[derive(Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[cfg(johnny)]
#[derive(Serialize, Deserialize)]
pub struct JohnnyConfig {
    /// imgur client id
    pub imgur: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub token: String,

    #[cfg(db)]
    pub database: DatabaseConfig,

    #[cfg(johnny)]
    pub johnny: JohnnyConfig,
}

impl Config {
    pub fn load() -> Result<Self> {
        #[cfg(dev)]
        return Ok(Self {
            token: env!("DISCORD_TOKEN").into(),
            #[cfg(db)]
            database: DatabaseConfig {
                url: env!("DATABASE_URL").into(),
            },
            #[cfg(johnny)]
            johnny: JohnnyConfig {
                imgur: env!("IMGUR_CLIENT_ID").into(),
            },
        });

        #[cfg(not(dev))]
        toml::from_str::<Config>(
            read_to_string("config.toml")
                .context("config.toml should exist")?
                .as_str(),
        )
        .context("config.toml should be valid toml")
    }
}