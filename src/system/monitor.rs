use sysinfo::{Disks, System};
use std::collections::VecDeque;

const HISTORY_SIZE: usize = 60;

#[derive(Debug, Clone)]
pub struct CpuStats {
    pub global_usage: f32,
    #[allow(dead_code)]
    pub per_core: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub used: u64,
    pub total: u64,
    pub swap_used: u64,
    pub swap_total: u64,
}

impl MemoryStats {
    pub fn used_percent(&self) -> f32 {
        if self.total == 0 {
            0.0
        } else {
            (self.used as f32 / self.total as f32) * 100.0
        }
    }

    pub fn swap_percent(&self) -> f32 {
        if self.swap_total == 0 {
            0.0
        } else {
            (self.swap_used as f32 / self.swap_total as f32) * 100.0
        }
    }

    pub fn used_gb(&self) -> f64 {
        self.used as f64 / 1024.0 / 1024.0 / 1024.0
    }

    pub fn total_gb(&self) -> f64 {
        self.total as f64 / 1024.0 / 1024.0 / 1024.0
    }

    pub fn swap_used_gb(&self) -> f64 {
        self.swap_used as f64 / 1024.0 / 1024.0 / 1024.0
    }

    pub fn swap_total_gb(&self) -> f64 {
        self.swap_total as f64 / 1024.0 / 1024.0 / 1024.0
    }
}

#[derive(Debug, Clone)]
pub struct DiskStats {
    pub name: String,
    #[allow(dead_code)]
    pub mount_point: String,
    pub total: u64,
    pub available: u64,
    pub disk_type: String,
    #[allow(dead_code)]
    pub file_system: String,
}

impl DiskStats {
    pub fn used(&self) -> u64 {
        self.total.saturating_sub(self.available)
    }

    pub fn used_percent(&self) -> f32 {
        if self.total == 0 {
            0.0
        } else {
            (self.used() as f32 / self.total as f32) * 100.0
        }
    }

    pub fn total_gb(&self) -> f64 {
        self.total as f64 / 1024.0 / 1024.0 / 1024.0
    }

    pub fn used_gb(&self) -> f64 {
        self.used() as f64 / 1024.0 / 1024.0 / 1024.0
    }

    #[allow(dead_code)]
    pub fn available_gb(&self) -> f64 {
        self.available as f64 / 1024.0 / 1024.0 / 1024.0
    }
}

pub struct SystemMonitor {
    system: System,
    disks: Disks,
    cpu_history: VecDeque<f32>,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        Self {
            system,
            disks: Disks::new_with_refreshed_list(),
            cpu_history: VecDeque::with_capacity(HISTORY_SIZE),
        }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_cpu_all();
        self.system.refresh_memory();
        self.disks.refresh();
    }

    pub fn cpu_stats(&self) -> CpuStats {
        let global_usage = self.system.global_cpu_usage();
        let per_core = self.system.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

        CpuStats {
            global_usage,
            per_core,
        }
    }

    pub fn update_cpu_history(&mut self) {
        let global_usage = self.system.global_cpu_usage();
        self.cpu_history.push_back(global_usage);
        if self.cpu_history.len() > HISTORY_SIZE {
            self.cpu_history.pop_front();
        }
    }

    pub fn cpu_history(&self) -> Vec<f32> {
        self.cpu_history.iter().copied().collect()
    }

    pub fn memory_stats(&self) -> MemoryStats {
        MemoryStats {
            used: self.system.used_memory(),
            total: self.system.total_memory(),
            swap_used: self.system.used_swap(),
            swap_total: self.system.total_swap(),
        }
    }

    pub fn disk_stats(&self) -> Vec<DiskStats> {
        self.disks
            .iter()
            .map(|disk| {
                let disk_type = match disk.kind() {
                    sysinfo::DiskKind::HDD => "HDD",
                    sysinfo::DiskKind::SSD => "SSD",
                    _ => "Unknown",
                }.to_string();

                let file_system = disk.file_system().to_string_lossy().to_string();

                DiskStats {
                    name: disk.name().to_string_lossy().to_string(),
                    mount_point: disk.mount_point().to_string_lossy().to_string(),
                    total: disk.total_space(),
                    available: disk.available_space(),
                    disk_type,
                    file_system,
                }
            })
            .collect()
    }
}

