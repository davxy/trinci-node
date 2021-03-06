// This file is part of TRINCI.
//
// Copyright (C) 2021 Affidaty Spa.
//
// TRINCI is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the
// Free Software Foundation, either version 3 of the License, or (at your
// option) any later version.
//
// TRINCI is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License
// for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with TRINCI. If not, see <https://www.gnu.org/licenses/>.

//! Node configuration
//!
//! Parameters to pragmatically tweak the core behaviour.

use std::{fs, path::Path};
use toml::Value;

/// TODO: add to configuration??? Maybe yes... maybe not.
pub const SERVICE_ACCOUNT_ID: &str = "QmfZy5bvk7a3DQAjCbGNtmrPXWkyVvPrdnZMyBZ5q5ieKG";

/// Default configuration file.
const DEFAULT_CONFIG_FILE: &str = "config.toml";

/// Default logger verbosity level.
pub const DEFAULT_LOG_LEVEL: &str = "info";

/// Default bootstrap file path.
pub const DEFAULT_BOOTSTRAP_PATH: &str = "data/bootstrap.wasm";

/// Default network identifier.
pub const DEFAULT_NETWORK_ID: &str = "skynet";

/// Default max transactions per block.
pub const DEFAULT_BLOCK_THRESHOLD: usize = 42;

/// Default block generation max time.
pub const DEFAULT_BLOCK_TIMEOUT: u16 = 3;

/// Default http service binding address.
pub const DEFAULT_HTTP_ADDR: &str = "127.0.0.1";

/// Default http service port.
pub const DEFAULT_HTTP_PORT: u16 = 8000;

/// Default bridge service binding address.
pub const DEFAULT_BRIDGE_ADDR: &str = "127.0.0.1";

/// Default bridge service port.
pub const DEFAULT_BRIDGE_PORT: u16 = 8001;

/// Default p2p service binding address.
pub const DEFAULT_P2P_ADDR: &str = "127.0.0.1";

/// Default database path.
pub const DEFAULT_DB_PATH: &str = "db";

/// Default smart contracts cache size.
pub const DEFAULT_WM_CACHE_MAX: usize = 10;

/// Core configuration structure.
#[derive(PartialEq, Debug, Clone)]
pub struct Config {
    /// Log level.
    pub log_level: String,
    /// Node started as a validator. A validator participates to the consensus
    /// algorithm for blocks production (default: false).
    pub validator: bool,
    /// Optional node keypair file.
    pub keypair_path: Option<String>,
    /// Network identifier.
    pub network: String,
    /// Max number of transactions within a block.
    pub block_threshold: usize,
    /// Max number of seconds to trigger block creation if the threshold has not
    /// been reached. Block is created with at least one transaction.
    pub block_timeout: u16,
    /// Http service address.
    pub rest_addr: String,
    /// Http service port.
    pub rest_port: u16,
    /// Bridge service address.
    pub bridge_addr: String,
    /// Bridge service port.
    pub bridge_port: u16,
    /// P2P service address.
    pub p2p_addr: String,
    /// Blockchain database folder path.
    pub db_path: String,
    /// Bootstrap wasm file path.
    pub bootstrap_path: String,
    /// WASM machine max cache size.
    pub wm_cache_max: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            log_level: DEFAULT_LOG_LEVEL.to_string(),
            validator: false,
            keypair_path: None,
            network: DEFAULT_NETWORK_ID.to_string(),
            block_threshold: DEFAULT_BLOCK_THRESHOLD,
            block_timeout: DEFAULT_BLOCK_TIMEOUT,
            rest_addr: DEFAULT_HTTP_ADDR.to_string(),
            rest_port: DEFAULT_HTTP_PORT,
            bridge_addr: DEFAULT_BRIDGE_ADDR.to_string(),
            bridge_port: DEFAULT_BRIDGE_PORT,
            p2p_addr: DEFAULT_P2P_ADDR.to_string(),
            db_path: DEFAULT_DB_PATH.to_string(),
            bootstrap_path: DEFAULT_BOOTSTRAP_PATH.to_string(),
            wm_cache_max: DEFAULT_WM_CACHE_MAX,
        }
    }
}

impl Config {
    /// Instance a new configuration using options found in the config file.
    /// If a config option is not found in the file, then the default one is used.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Option<Self> {
        let mut config = Self::default();

        let map = match fs::read_to_string(path) {
            Ok(content) => match content.parse::<Value>() {
                Ok(map) => map,
                Err(_err) => {
                    error!("Error: bad config file format");
                    return None;
                }
            },
            Err(_err) => {
                warn!("Warning: config file not found, using default options");
                return Some(config);
            }
        };

        if let Some(value) = map.get("validator").and_then(|value| value.as_bool()) {
            config.validator = value;
        }
        if let Some(value) = map.get("log-level").and_then(|value| value.as_str()) {
            config.log_level = value.to_owned()
        }
        if let Some(value) = map.get("keypair-path").and_then(|value| value.as_str()) {
            config.keypair_path = Some(value.to_owned())
        }
        if let Some(value) = map.get("network").and_then(|value| value.as_str()) {
            config.network = value.to_owned();
        }
        if let Some(value) = map.get("rest-addr").and_then(|value| value.as_str()) {
            config.rest_addr = value.to_owned();
        }
        if let Some(value) = map.get("rest-port").and_then(|value| value.as_integer()) {
            config.rest_port = value as u16;
        }
        if let Some(value) = map.get("bridge-addr").and_then(|value| value.as_str()) {
            config.bridge_addr = value.to_owned();
        }
        if let Some(value) = map.get("bridge-port").and_then(|value| value.as_integer()) {
            config.bridge_port = value as u16;
        }
        if let Some(value) = map.get("p2p-addr").and_then(|value| value.as_str()) {
            config.p2p_addr = value.to_owned();
        }
        if let Some(value) = map
            .get("block-threshold")
            .and_then(|value| value.as_integer())
        {
            config.block_threshold = value as usize;
        }
        if let Some(value) = map
            .get("block-timeout")
            .and_then(|value| value.as_integer())
        {
            config.block_timeout = value as u16;
        }
        if let Some(value) = map.get("db-path").and_then(|value| value.as_str()) {
            config.db_path = value.to_owned();
        }
        if let Some(value) = map.get("bootstrap-path").and_then(|value| value.as_str()) {
            config.bootstrap_path = value.to_owned();
        }
        if let Some(value) = map.get("wm-cache-max").and_then(|value| value.as_integer()) {
            config.wm_cache_max = value as usize;
        }

        Some(config)
    }
}

pub fn create_app_config() -> Config {
    let matches = clap::App::new("T2 Node")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            clap::Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Configuration file (default 'config.toml')")
                .value_name("CONFIG")
                .required(false),
        )
        .arg(
            clap::Arg::with_name("validator")
                .long("validator")
                .help("Start node as a validator"),
        )
        .arg(
            clap::Arg::with_name("log-level")
                .long("log-level")
                .help(&format!("Logger level (default '{}')", DEFAULT_LOG_LEVEL))
                .value_name("LEVEL")
                .required(false)
                .possible_values(&["off", "error", "warn", "info", "debug", "trace"]),
        )
        .arg(
            clap::Arg::with_name("network")
                .long("network")
                .help(&format!(
                    "Blockchain network identifier (default '{}')",
                    DEFAULT_NETWORK_ID
                ))
                .value_name("NETWORK-NAME")
                .required(false),
        )
        .arg(
            clap::Arg::with_name("db-path")
                .long("db-path")
                .help(&format!("Database folder (default '{}')", DEFAULT_DB_PATH))
                .value_name("PATH")
                .required(false),
        )
        .arg(
            clap::Arg::with_name("bootstrap-path")
                .long("bootstrap-path")
                .help(&format!(
                    "Bootstrap wasm file path (default '{}')",
                    DEFAULT_BOOTSTRAP_PATH
                ))
                .value_name("PATH")
                .required(false),
        )
        .arg(
            clap::Arg::with_name("http-addr")
                .long("http-addr")
                .help("Http service binding address (default '127.0.0.1')")
                .value_name("ADDRESS")
                .required(false),
        )
        .arg(
            clap::Arg::with_name("http-port")
                .long("http-port")
                .help("Http service listening port (default '8000')")
                .value_name("PORT")
                .required(false),
        )
        .arg(
            clap::Arg::with_name("bridge-addr")
                .long("bridge-addr")
                .help("Bridge service binding address (default '127.0.0.1')")
                .value_name("ADDRESS")
                .required(false),
        )
        .arg(
            clap::Arg::with_name("bridge-port")
                .long("bridge-port")
                .help("Bridge service listening port (default '8001')")
                .value_name("PORT")
                .required(false),
        )
        .arg(
            clap::Arg::with_name("p2p-addr")
                .long("p2p-addr")
                .help("Peer2Peer service binding address (default '127.0.0.1')")
                .value_name("ADDRESS")
                .required(false),
        )
        .get_matches();

    let config_file = matches.value_of("config").unwrap_or(DEFAULT_CONFIG_FILE);
    let mut config = Config::from_file(config_file).expect("Bad config file");

    // Tweak configuration using command line arguments.
    if matches.is_present("validator") {
        config.validator = true;
    }
    if let Some(value) = matches.value_of("log-level") {
        config.log_level = value.to_owned();
    }
    if let Some(value) = matches.value_of("network") {
        config.network = value.to_owned();
    }
    if let Some(value) = matches.value_of("db-path") {
        config.db_path = value.to_owned();
    }
    if let Some(value) = matches.value_of("boot-path") {
        config.bootstrap_path = value.to_owned();
    }
    if let Some(value) = matches.value_of("http-addr") {
        config.rest_addr = value.to_owned();
    }
    if let Some(value) = matches
        .value_of("http-port")
        .and_then(|value| value.parse::<u16>().ok())
    {
        config.rest_port = value;
    }
    if let Some(value) = matches.value_of("bridge-addr") {
        config.bridge_addr = value.to_owned();
    }
    if let Some(value) = matches
        .value_of("bridge-port")
        .and_then(|value| value.parse::<u16>().ok())
    {
        config.bridge_port = value;
    }
    if let Some(value) = matches.value_of("p2p-addr") {
        config.p2p_addr = value.to_owned();
    }
    config
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::{self, Display, Formatter};
    use std::io::Write;
    use tempfile::NamedTempFile;

    impl Display for Config {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "validator = {}\n\
                log-level = '{}'\n\
                network = '{}'\n\
                block-threshold = {}\n\
                block-timeout = {}\n\
                rest-addr = '{}'\n\
                rest-port = {}\n\
                bridge-addr = '{}'\n\
                bridge-port = {}\n\
                p2p-addr = '{}'\n\
                db-path = '{}'\n\
                bootstrap-path = '{}'\n\
                wm-cache-max = {}",
                self.validator,
                self.log_level,
                self.network,
                self.block_threshold,
                self.block_timeout,
                self.rest_addr,
                self.rest_port,
                self.bridge_addr,
                self.bridge_port,
                self.p2p_addr,
                self.db_path,
                self.bootstrap_path,
                self.wm_cache_max
            )
        }
    }

    fn create_test_config() -> Config {
        Config {
            log_level: "debug".to_string(),
            validator: true,
            keypair_path: None,
            network: "dummy_network".to_string(),
            block_threshold: 1234,
            block_timeout: 4321,
            rest_addr: "1.2.3.4".to_string(),
            rest_port: 123,
            bridge_addr: "5.6.7.8".to_string(),
            bridge_port: 987,
            p2p_addr: "9.1.2.3".to_string(),
            db_path: "dummy/db/path".to_string(),
            bootstrap_path: "dummy/boot/path".to_string(),
            wm_cache_max: 42,
        }
    }

    #[test]
    fn from_file() {
        let default_config = create_test_config();
        let mut file = NamedTempFile::new().unwrap();
        let _ = writeln!(&mut file, "{}", default_config);
        let filename = file.path().as_os_str().to_string_lossy().to_string();

        let config = Config::from_file(filename).unwrap();

        assert_eq!(config, default_config);
    }
}
