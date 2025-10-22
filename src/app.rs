use crate::config::Config;
use crate::system::{NetworkStats, SystemInfo, SystemMonitor};
use std::time::Instant;

pub struct App {
    pub config: Config,
    pub system_info: SystemInfo,
    pub monitor: SystemMonitor,
    pub network: NetworkStats,
    pub last_refresh: Instant,
    pub should_quit: bool,
    pub current_theme_index: usize,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            system_info: SystemInfo::new(),
            monitor: SystemMonitor::new(),
            network: NetworkStats::new(),
            last_refresh: Instant::now(),
            should_quit: false,
            current_theme_index: 0,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refresh);

        if elapsed.as_millis() >= self.config.refresh_rate as u128 {
            self.monitor.refresh();
            self.monitor.update_cpu_history();
            self.network.refresh();
            self.last_refresh = now;
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn force_refresh(&mut self) {
        self.monitor.refresh();
        self.monitor.update_cpu_history();
        self.network.refresh();
        self.last_refresh = Instant::now();
    }

    pub fn cycle_theme(&mut self) {
        let themes = [
            "default", "nord", "gruvbox", "dracula", "monokai",
            "cyberpunk", "tokyo-night", "solarized-dark", "solarized-light",
            "one-dark", "material", "ayu-dark", "rosepine"
        ];
        self.current_theme_index = (self.current_theme_index + 1) % themes.len();
        self.config.theme = crate::theme::Theme::from_name(themes[self.current_theme_index]);
    }
}

