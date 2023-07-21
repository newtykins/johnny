#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
use migration::TABLES;
#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
use rust_format::{Formatter, RustFmt};
#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
use sea_orm_codegen::{DateTimeCrate, EntityTransformer, EntityWriterContext, WithSerde};
use std::{env, fs::File, io::Write};

/// Create an alias for `#[cfg]` attributes to use
macro_rules! cfg_aliases {
    ($($alias:tt = $config:meta),* $(,)*) => {
        $(
            if cfg!($config) {
                println!("cargo:rustc-cfg={}", stringify!($alias));
            }
        )*
    };
}

/// All features that should not be shown in the build_data.rs file
const HIDDEN_FEATURES: [&str; 2] = ["default", "db"];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // figure out enabled features
    let features: Vec<_> = env::vars()
        .map(|(k, _)| k)
        .filter(|k| k.starts_with("CARGO_FEATURE_"))
        .map(|k| {
            k.split("CARGO_FEATURE_")
                .last()
                .expect("there should be something after CARGO_FEATURE_")
                .to_lowercase()
        })
        .filter(|k| !HIDDEN_FEATURES.contains(&k.as_str()))
        .collect();

    println!("cargo:warning=enabled features: {}", features.join(", "));

    // save all of this information in in build_data.rs
    let mut file = File::create("src/build_data.rs")?;
    file.write_all(
        format!(
            "//! This file is generated by build.rs, do not edit it manually
pub const FEATURES: [&str; {}] = [{}];",
            features.len(),
            features
                .iter()
                .map(|f| format!("\"{}\"", f))
                .collect::<Vec<_>>()
                .join(", ")
        )
        .as_bytes(),
    )?;

    // generate entity files
    #[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
    let files = EntityTransformer::transform(TABLES.clone())?
        .generate(&EntityWriterContext::new(
            false,
            WithSerde::None,
            false,
            DateTimeCrate::Time,
            Some("public".into()),
            false,
            false,
            false,
            vec![],
            vec![],
        ))
        .files;

    #[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
    for out_file in files {
        let mut file = File::create(format!("src/db/entity/{}", out_file.name))?;
        let formatted = RustFmt::default().format_str(out_file.content)?;
        file.write_all(formatted.as_bytes())?;
    }

    cfg_aliases! {
        // ! general

        // is the tui enabled?
        tui = feature = "tui",
        // is the bot johnny?
        johnny = feature = "johnny",
        // should the logger be verbose?
        verbose = feature = "verbose",

        // ! database drivers

        // does the bot use sqlite?
        sqlite = feature = "sqlite",
        // is a single database driver enabled?
        db = any(feature = "postgres", feature = "mysql", feature = "sqlite"),
        // are multiple of the database drivers enabled?
        multiple_db = any(all(feature = "postgres", feature = "mysql"), all(feature = "mysql", feature = "sqlite"), all(feature = "postgres", feature = "sqlite"), all(feature = "postgres", feature = "mysql", feature = "sqlite")),

        // ! modules

        // is autorole enabled?
        autorole = feature = "autorole",

        // are pride commands enabled?
        pride = feature = "pride",

        // are animal commands enabled?
        animals = feature = "animals",

        // ! other

        // is the bot running in a development environment?
        dev = feature = "dev",
    }

    Ok(())
}
