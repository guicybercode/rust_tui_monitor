use sysinfo::System;
use std::env;

#[derive(Debug, Clone)]
pub struct DisplayInfo {
    pub resolution: String,
    pub refresh_rate: String,
}

#[derive(Debug, Clone)]
pub struct AudioInfo {
    pub default_device: String,
    #[allow(dead_code)]
    pub output_devices: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub hostname: String,
    pub os_name: String,
    #[allow(dead_code)]
    pub os_version: String,
    pub kernel_version: String,
    pub cpu_name: String,
    pub cpu_cores: usize,
    pub cpu_physical_cores: usize,
    pub cpu_frequency: String,
    pub total_memory: u64,
    pub architecture: String,
    pub username: String,
    pub shell: String,
    pub distribution: String,
    pub display_info: DisplayInfo,
    pub audio_info: AudioInfo,
    pub boot_time: String,
    pub processes_count: usize,
}

impl SystemInfo {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());
        let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
        let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
        let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());

        let cpu_name = sys
            .cpus()
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "Unknown CPU".to_string());

        let cpu_cores = sys.cpus().len();
        let cpu_physical_cores = sys.physical_core_count().unwrap_or(cpu_cores);
        let cpu_frequency = sys
            .cpus()
            .first()
            .map(|cpu| format!("{:.2} GHz", cpu.frequency() as f64 / 1000.0))
            .unwrap_or_else(|| "N/A".to_string());
        let total_memory = sys.total_memory();
        
        let boot_time = Self::get_boot_time();
        let processes_count = sys.processes().len();

        let architecture = env::consts::ARCH.to_string();
        let username = env::var("USER")
            .or_else(|_| env::var("USERNAME"))
            .unwrap_or_else(|_| "Unknown".to_string());
        
        let shell = env::var("SHELL")
            .unwrap_or_else(|_| {
                if cfg!(target_os = "windows") {
                    "PowerShell".to_string()
                } else {
                    "Unknown".to_string()
                }
            });

        let distribution = System::long_os_version().unwrap_or_else(|| os_version.clone());

        let display_info = Self::get_display_info();
        let audio_info = Self::get_audio_info();

        Self {
            hostname,
            os_name,
            os_version,
            kernel_version,
            cpu_name,
            cpu_cores,
            cpu_physical_cores,
            cpu_frequency,
            total_memory,
            architecture,
            username,
            shell,
            distribution,
            display_info,
            audio_info,
            boot_time,
            processes_count,
        }
    }

    fn get_boot_time() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let uptime = System::uptime();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let boot_timestamp = now - uptime;
        
        let datetime = chrono::DateTime::from_timestamp(boot_timestamp as i64, 0);
        if let Some(dt) = datetime {
            dt.format("%Y-%m-%d %H:%M:%S").to_string()
        } else {
            "Unknown".to_string()
        }
    }

    fn get_display_info() -> DisplayInfo {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            
            let output = Command::new("powershell")
                .args(["-NoProfile", "-Command", 
                    "Add-Type -AssemblyName System.Windows.Forms; \
                    $screen = [System.Windows.Forms.Screen]::PrimaryScreen; \
                    Write-Output \"$($screen.Bounds.Width)x$($screen.Bounds.Height)\""])
                .output();

            if let Ok(output) = output {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !stdout.is_empty() && stdout.contains('x') {
                    return DisplayInfo {
                        resolution: stdout,
                        refresh_rate: "60Hz".to_string(),
                    };
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            
            let output = Command::new("xrandr")
                .output();

            if let Ok(output) = output {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if line.contains("*") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() > 1 {
                            return DisplayInfo {
                                resolution: parts[0].to_string(),
                                refresh_rate: if parts.len() > 1 { 
                                    parts[1].replace("*", "").replace("+", "")
                                } else { 
                                    "N/A".to_string() 
                                },
                            };
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            
            let output = Command::new("system_profiler")
                .args(["SPDisplaysDataType"])
                .output();

            if let Ok(output) = output {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if line.contains("Resolution:") {
                        let res = line.split(':').nth(1).unwrap_or("").trim();
                        if !res.is_empty() {
                            return DisplayInfo {
                                resolution: res.to_string(),
                                refresh_rate: "60Hz".to_string(),
                            };
                        }
                    }
                }
            }
        }

        DisplayInfo {
            resolution: "Unknown".to_string(),
            refresh_rate: "Unknown".to_string(),
        }
    }

    fn get_audio_info() -> AudioInfo {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            
            let output = Command::new("powershell")
                .args(["-NoProfile", "-Command", 
                    "(Get-WmiObject Win32_SoundDevice | Select-Object -First 1 -ExpandProperty Name)"])
                .output();

            if let Ok(output) = output {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !stdout.is_empty() {
                    return AudioInfo {
                        default_device: stdout,
                        output_devices: vec![],
                    };
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            
            let output = Command::new("pactl")
                .args(["list", "sinks", "short"])
                .output();

            if let Ok(output) = output {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Some(line) = stdout.lines().next() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() > 1 {
                        return AudioInfo {
                            default_device: parts[1].to_string(),
                            output_devices: vec![],
                        };
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            
            let output = Command::new("system_profiler")
                .args(["SPAudioDataType"])
                .output();

            if let Ok(output) = output {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if line.contains("Default Output Device:") {
                        let device = line.split(':').nth(1).unwrap_or("").trim();
                        if !device.is_empty() {
                            return AudioInfo {
                                default_device: device.to_string(),
                                output_devices: vec![],
                            };
                        }
                    }
                }
            }
        }

        AudioInfo {
            default_device: "Unknown".to_string(),
            output_devices: vec![],
        }
    }

    pub fn uptime_string(&self) -> String {
        let uptime = System::uptime();
        let days = uptime / 86400;
        let hours = (uptime % 86400) / 3600;
        let minutes = (uptime % 3600) / 60;

        if days > 0 {
            format!("{}d {}h {}m", days, hours, minutes)
        } else if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        }
    }

    pub fn memory_gb(&self) -> f64 {
        self.total_memory as f64 / 1024.0 / 1024.0 / 1024.0
    }
}

