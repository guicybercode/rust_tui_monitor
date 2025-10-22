use sysinfo::Networks;
use std::time::{Duration, Instant};

pub struct NetworkStats {
    networks: Networks,
    last_update: Instant,
    total_received: u64,
    total_transmitted: u64,
    rx_rate: f64,
    tx_rate: f64,
}

impl NetworkStats {
    pub fn new() -> Self {
        let mut networks = Networks::new_with_refreshed_list();
        networks.refresh();

        let (total_received, total_transmitted) = Self::calculate_totals(&networks);

        Self {
            networks,
            last_update: Instant::now(),
            total_received,
            total_transmitted,
            rx_rate: 0.0,
            tx_rate: 0.0,
        }
    }

    pub fn refresh(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update);

        let old_rx = self.total_received;
        let old_tx = self.total_transmitted;

        self.networks.refresh();
        let (new_rx, new_tx) = Self::calculate_totals(&self.networks);

        self.total_received = new_rx;
        self.total_transmitted = new_tx;

        if elapsed > Duration::from_millis(100) {
            let seconds = elapsed.as_secs_f64();
            self.rx_rate = (new_rx.saturating_sub(old_rx)) as f64 / seconds;
            self.tx_rate = (new_tx.saturating_sub(old_tx)) as f64 / seconds;
        }

        self.last_update = now;
    }

    fn calculate_totals(networks: &Networks) -> (u64, u64) {
        networks
            .iter()
            .fold((0u64, 0u64), |(rx, tx), (_name, data)| {
                (rx + data.received(), tx + data.transmitted())
            })
    }

    pub fn rx_rate_mbps(&self) -> f64 {
        self.rx_rate / 1_024.0 / 1_024.0
    }

    pub fn tx_rate_mbps(&self) -> f64 {
        self.tx_rate / 1_024.0 / 1_024.0
    }

    pub fn total_received_gb(&self) -> f64 {
        self.total_received as f64 / 1_024.0 / 1_024.0 / 1_024.0
    }

    pub fn total_transmitted_gb(&self) -> f64 {
        self.total_transmitted as f64 / 1_024.0 / 1_024.0 / 1_024.0
    }

    pub fn format_rate(bytes_per_sec: f64) -> String {
        if bytes_per_sec < 1_024.0 {
            format!("{:.1} B/s", bytes_per_sec)
        } else if bytes_per_sec < 1_024.0 * 1_024.0 {
            format!("{:.1} KB/s", bytes_per_sec / 1_024.0)
        } else {
            format!("{:.2} MB/s", bytes_per_sec / 1_024.0 / 1_024.0)
        }
    }
}

