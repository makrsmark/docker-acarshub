// Copyright (C) 2024 Fred Clausen

// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 3
// of the License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA

// ACARS Hub will be configured via using the following logic for ordering of configuration values:
//  * Default values (least priority)
//  * Configuration file (toml format)
//  * Environment variables
//  * Command line arguments (highest priority) TODO: Implement command line arguments. The config crate doesn't do this out of the box
//  If no config file is provided whatever value is picked will be written to the default config file
//  If a config file is provided, and a higher priority value is provided via environment variable or command line argument, the value will be written to the config file

/// ACARS Hub valid configuration options
/// database_url: The URL to the database
/// enable_acars: Enable ACARS processing
/// enable_vdlm2: Enable VDL-M2 processing
/// enable_hfdl: Enable HFDL processing
/// enable_iridium: Enable Iridium processing
/// enable_inmarsat: Enable Inmarsat processing
/// enable_adsb: Enable ADS-B processing
/// log_level: The log level. Valid values are: trace, debug, info, warn, error. Default is info. List is ordered from most verbose to least verbos
use config::Config;
use log::info;
use sdre_rust_logging::SetupLogging;
use std::collections::HashMap;

pub struct AhConfig {
    pub database_url: String,
    pub enable_acars: bool,
    pub enable_vdlm2: bool,
    pub enable_hfdl: bool,
    pub enable_iridium: bool,
    pub enable_inmarsat: bool,
    pub enable_adsb: bool,
    pub log_level: String,
    pub config_file: String,
}

impl Default for AhConfig {
    fn default() -> Self {
        AhConfig {
            database_url: "sqlite://acars.db".to_string(),
            enable_acars: false,
            enable_vdlm2: false,
            enable_hfdl: false,
            enable_iridium: false,
            enable_inmarsat: false,
            enable_adsb: false,
            log_level: "info".to_string(),
            config_file: AhConfig::get_file_path(),
        }
    }
}

impl AhConfig {
    pub fn new() -> Self {
        AhConfig::get_and_validate_config()
    }

    fn get_file_path() -> String {
        // if we are in a test env (denoted with AH_TEST_ENV_PATH) we will use the test config file
        // from the env variable. Otherwise, detect the platform and use "./ah_config.toml" for the config file

        if let Ok(path) = std::env::var("AH_TEST_ENV_PATH") {
            path
        } else if let Ok(path) = std::env::var("AH_CONFIG_PATH") {
            // this match arm is for docker specifically
            path
        } else {
            // FIXME: we should use platform specific paths
            match std::env::consts::OS {
                "linux" => "./ah_config.toml",
                "macos" => "./ah_config.toml",
                "windows" => "./ah_config.toml",
                _ => "./ah_config.toml",
            }
            .to_string()
        }
    }

    fn write_default_config(file_path: &str) {
        // Lets see if the file exists
        if !std::path::Path::new(&file_path).exists() {
            // if the file does not exist, we will write the default config to the file
            let default_config = r#"
                        database_url = "sqlite://acars.db"
                        enable_acars = false
                        enable_vdlm2 = false
                        enable_hfdl = false
                        enable_iridium = false
                        enable_inmarsat = false
                        enable_adsb = false
                        log_level = "info"
                    "#;

            std::fs::write(file_path, default_config).unwrap();

            println!(
                "Config file does not exist, creating it now at {}",
                std::fs::canonicalize(file_path)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            );
        }
    }

    fn get_config(file_path: &str) -> Option<HashMap<String, String>> {
        // if we are in a test env (denoted with AH_TEST_ENV_PATH) we will use the test config file
        // from the env variable. Otherwise, detect the platform and use "./ah_config.toml" for the config file

        AhConfig::write_default_config(file_path);

        let config = Config::builder()
            .add_source(config::File::with_name(file_path))
            .add_source(config::Environment::with_prefix("AH"))
            .build()
            .unwrap();

        config.try_deserialize().unwrap()
    }

    fn get_and_validate_config() -> AhConfig {
        let file_path = AhConfig::get_file_path();
        let config = AhConfig::get_config(&file_path).unwrap();

        let mut ah_config = AhConfig::default();

        if let Some(database_url) = config.get("database_url") {
            ah_config.database_url = database_url.to_string();
        }

        if let Some(enable_acars) = config.get("enable_acars") {
            ah_config.enable_acars = enable_acars.parse().unwrap();
        }

        if let Some(enable_vdlm2) = config.get("enable_vdlm2") {
            ah_config.enable_vdlm2 = enable_vdlm2.parse().unwrap();
        }

        if let Some(enable_hfdl) = config.get("enable_hfdl") {
            ah_config.enable_hfdl = enable_hfdl.parse().unwrap();
        }

        if let Some(enable_iridium) = config.get("enable_iridium") {
            ah_config.enable_iridium = enable_iridium.parse().unwrap();
        }

        if let Some(enable_inmarsat) = config.get("enable_inmarsat") {
            ah_config.enable_inmarsat = enable_inmarsat.parse().unwrap();
        }

        if let Some(enable_adsb) = config.get("enable_adsb") {
            ah_config.enable_adsb = enable_adsb.parse().unwrap();
        }

        if let Some(log_level) = config.get("log_level") {
            ah_config.log_level = log_level.to_string();
        }

        ah_config
    }

    pub fn show_config(&self) {
        info!("database_url: {}", self.database_url);
        info!("enable_acars: {}", self.enable_acars);
        info!("enable_vdlm2: {}", self.enable_vdlm2);
        info!("enable_hfdl: {}", self.enable_hfdl);
        info!("enable_iridium: {}", self.enable_iridium);
        info!("enable_inmarsat: {}", self.enable_inmarsat);
        info!("enable_adsb: {}", self.enable_adsb);
        info!("log_level: {}", self.log_level);
    }

    pub fn enable_logging(&self) {
        self.log_level.enable_logging();
    }

    pub fn get_config_as_toml_string(&self) -> String {
        let mut config = String::new();

        config.push_str(&format!("database_url = \"{}\"\n", self.database_url));
        config.push_str(&format!("enable_acars = {}\n", self.enable_acars));
        config.push_str(&format!("enable_vdlm2 = {}\n", self.enable_vdlm2));
        config.push_str(&format!("enable_hfdl = {}\n", self.enable_hfdl));
        config.push_str(&format!("enable_iridium = {}\n", self.enable_iridium));
        config.push_str(&format!("enable_inmarsat = {}\n", self.enable_inmarsat));
        config.push_str(&format!("enable_adsb = {}\n", self.enable_adsb));
        config.push_str(&format!("log_level = \"{}\"\n", self.log_level));

        config
    }

    pub fn write_config(&self) {
        let file_path = AhConfig::get_file_path();
        let config = self.get_config_as_toml_string();

        std::fs::write(file_path, config).unwrap();
    }
}
