use crate::theme::Theme;
use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "rust-tui-monitor")]
#[command(about = "A lightweight TUI system monitor", long_about = None)]
pub struct CliArgs {
    #[arg(short, long, help = "Theme name (default, nord, gruvbox, dracula, monokai)")]
    pub theme: Option<String>,

    #[arg(short, long, help = "Path to custom config file")]
    pub config: Option<PathBuf>,

    #[arg(short, long, help = "Refresh rate in milliseconds")]
    pub refresh_rate: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub theme: Theme,
    pub refresh_rate: u64,
    pub show_cpu: bool,
    pub show_memory: bool,
    pub show_disk: bool,
    pub show_network: bool,
    pub rounded_borders: bool,
    pub temp_unit: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            refresh_rate: 1000,
            show_cpu: true,
            show_memory: true,
            show_disk: true,
            show_network: true,
            rounded_borders: true,
            temp_unit: "celsius".to_string(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let args = CliArgs::parse();
        let mut config = Self::load_from_file(args.config.as_deref())?;

        if let Some(theme_name) = args.theme {
            config.theme = Theme::from_name(&theme_name);
        }

        if let Some(rate) = args.refresh_rate {
            config.refresh_rate = rate;
        }

        Ok(config)
    }

    fn load_from_file(path: Option<&std::path::Path>) -> Result<Self> {
        let config_path = path
            .map(|p| p.to_path_buf())
            .or_else(|| {
                dirs::config_dir().map(|mut p| {
                    p.push("rust-tui-monitor");
                    p.push("config.toml");
                    p
                })
            })
            .unwrap_or_else(|| PathBuf::from("config.toml"));

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }
}

fn dirs() -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        std::env::var("APPDATA").ok().map(PathBuf::from)
    } else if cfg!(target_os = "macos") {
        std::env::var("HOME")
            .ok()
            .map(|h| PathBuf::from(h).join("Library/Application Support"))
    } else {
        std::env::var("HOME")
            .ok()
            .map(|h| PathBuf::from(h).join(".config"))
    }
}

mod dirs {
    use std::path::PathBuf;
    pub fn config_dir() -> Option<PathBuf> {
        super::dirs()
    }
}

