use cfg_aliases::cfg_aliases;
use rayon::prelude::*;
use std::{env, error::Error, fs::File, io::Write};

/// All features that should not be shown in the build_data.rs file
const HIDDEN_FEATURES: [&str; 2] = ["default", "db"];

fn main() -> Result<(), Box<dyn Error>> {
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
                .par_iter()
                .map(|f| format!("\"{}\"", f))
                .collect::<Vec<_>>()
                .join(", ")
        )
        .as_bytes(),
    )?;

    cfg_aliases! {
        johnny: { feature = "johnny" },
        dev: { feature = "dev" },
        tui: { feature = "tui" },
        verbose: { feature = "verbose" },
        autorole: { feature = "autorole" },
        pride: { feature = "pride" },

        // db
        db: { feature = "db" },
        sqlite: { feature = "sqlite" },
        multiple_db: { any(all(feature = "postgres", feature = "mysql"), all(feature = "mysql", feature = "sqlite"), all(feature = "postgres", feature = "sqlite"), all(feature = "postgres", feature = "mysql", feature = "sqlite")) }
    }

    Ok(())
}
