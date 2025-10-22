use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Sparkline},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Percentage(60),
            Constraint::Min(3),
        ])
        .split(f.area());

    render_system_info(f, chunks[0], app);
    render_monitoring(f, chunks[1], app);
    render_footer(f, chunks[2], app);
}

fn render_system_info(f: &mut Frame, area: Rect, app: &App) {
    let theme = &app.config.theme;
    let info = &app.system_info;

    let border_type = if app.config.rounded_borders {
        ratatui::widgets::BorderType::Rounded
    } else {
        ratatui::widgets::BorderType::Plain
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(border_type)
        .border_style(Style::default().fg(theme.border()))
        .title(Span::styled(
            " System Information ",
            Style::default().fg(theme.primary()).add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(theme.bg()));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(12), Constraint::Min(30)])
        .split(inner);

    let dog_art = vec![
        Line::from(""),
        Line::from(Span::styled("   ʕ•ᴥ•ʔ", Style::default().fg(theme.secondary()).add_modifier(Modifier::BOLD))),
        Line::from(Span::styled("  ∪￣￣∪", Style::default().fg(theme.secondary()))),
        Line::from(""),
    ];

    let dog_paragraph = Paragraph::new(dog_art)
        .style(Style::default().bg(theme.bg()))
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(dog_paragraph, main_chunks[0]);

    let info_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[1]);

    let left_info = vec![
        Line::from(vec![
            Span::styled("User: ", Style::default().fg(theme.secondary()).add_modifier(Modifier::BOLD)),
            Span::styled(format!("{}@{}", info.username, info.hostname), Style::default().fg(theme.fg())),
        ]),
        Line::from(vec![
            Span::styled("OS: ", Style::default().fg(theme.secondary()).add_modifier(Modifier::BOLD)),
            Span::styled(&info.os_name, Style::default().fg(theme.fg())),
        ]),
        Line::from(vec![
            Span::styled("Distro: ", Style::default().fg(theme.secondary()).add_modifier(Modifier::BOLD)),
            Span::styled(&info.distribution, Style::default().fg(theme.fg())),
        ]),
        Line::from(vec![
            Span::styled("Kernel: ", Style::default().fg(theme.secondary()).add_modifier(Modifier::BOLD)),
            Span::styled(&info.kernel_version, Style::default().fg(theme.fg())),
        ]),
        Line::from(vec![
            Span::styled("Arch: ", Style::default().fg(theme.secondary()).add_modifier(Modifier::BOLD)),
            Span::styled(&info.architecture, Style::default().fg(theme.fg())),
        ]),
        Line::from(vec![
            Span::styled("Shell: ", Style::default().fg(theme.secondary()).add_modifier(Modifier::BOLD)),
            Span::styled(&info.shell, Style::default().fg(theme.fg())),
        ]),
        Line::from(vec![
            Span::styled("Processes: ", Style::default().fg(theme.secondary()).add_modifier(Modifier::BOLD)),
            Span::styled(format!("{}", info.processes_count), Style::default().fg(theme.fg())),
        ]),
        Line::from(vec![
            Span::styled("Boot Time: ", Style::default().fg(theme.secondary()).add_modifier(Modifier::BOLD)),
            Span::styled(&info.boot_time, Style::default().fg(theme.fg())),
        ]),
    ];

    let mut right_info = vec![
        Line::from(vec![
            Span::styled("CPU: ", Style::default().fg(theme.secondary()).add_modifier(Modifier::BOLD)),
            Span::styled(&info.cpu_name, Style::default().fg(theme.fg())),
        ]),
        Line::from(vec![
            Span::styled("Cores: ", Style::default().fg(theme.secondary()).add_modifier(Modifier::BOLD)),
            Span::styled(format!("{} ({} physical)", info.cpu_cores, info.cpu_physical_cores), Style::default().fg(theme.fg())),
        ]),
        Line::from(vec![
            Span::styled("Frequency: ", Style::default().fg(theme.secondary()).add_modifier(Modifier::BOLD)),
            Span::styled(&info.cpu_frequency, Style::default().fg(theme.fg())),
        ]),
        Line::from(vec![
            Span::styled("Memory: ", Style::default().fg(theme.secondary()).add_modifier(Modifier::BOLD)),
            Span::styled(format!("{:.1} GB", info.memory_gb()), Style::default().fg(theme.fg())),
        ]),
        Line::from(vec![
            Span::styled("Uptime: ", Style::default().fg(theme.secondary()).add_modifier(Modifier::BOLD)),
            Span::styled(info.uptime_string(), Style::default().fg(theme.fg())),
        ]),
    ];

    if info.display_info.resolution != "Unknown" && info.display_info.resolution != "N/A" {
        right_info.push(Line::from(vec![
            Span::styled("Display: ", Style::default().fg(theme.secondary()).add_modifier(Modifier::BOLD)),
            Span::styled(format!("{} @ {}", info.display_info.resolution, info.display_info.refresh_rate), Style::default().fg(theme.fg())),
        ]));
    }

    if info.audio_info.default_device != "Unknown" && info.audio_info.default_device != "N/A" {
        let audio_device = if info.audio_info.default_device.len() > 35 {
            format!("{}...", &info.audio_info.default_device[..32])
        } else {
            info.audio_info.default_device.clone()
        };
        right_info.push(Line::from(vec![
            Span::styled("Audio: ", Style::default().fg(theme.secondary()).add_modifier(Modifier::BOLD)),
            Span::styled(audio_device, Style::default().fg(theme.fg())),
        ]));
    }

    let left_paragraph = Paragraph::new(left_info).style(Style::default().bg(theme.bg()));
    let right_paragraph = Paragraph::new(right_info).style(Style::default().bg(theme.bg()));

    f.render_widget(left_paragraph, info_chunks[0]);
    f.render_widget(right_paragraph, info_chunks[1]);
}

fn render_monitoring(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    render_cpu_memory(f, chunks[0], app);
    render_disk_network(f, chunks[1], app);
}

fn render_cpu_memory(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    render_cpu(f, chunks[0], app);
    render_memory(f, chunks[1], app);
}

fn render_cpu(f: &mut Frame, area: Rect, app: &App) {
    let theme = &app.config.theme;
    let cpu_stats = app.monitor.cpu_stats();

    let border_type = if app.config.rounded_borders {
        ratatui::widgets::BorderType::Rounded
    } else {
        ratatui::widgets::BorderType::Plain
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(border_type)
        .border_style(Style::default().fg(theme.border()))
        .title(Span::styled(
            format!(" CPU Usage: {:.1}% ", cpu_stats.global_usage),
            Style::default().fg(theme.primary()).add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(theme.bg()));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let cpu_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Min(5)])
        .margin(1)
        .split(inner);

    let gauge_color = theme.usage_color(cpu_stats.global_usage);
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(gauge_color))
        .ratio(cpu_stats.global_usage as f64 / 100.0)
        .label(format!("{:.1}%", cpu_stats.global_usage));

    f.render_widget(gauge, cpu_chunks[0]);

    let history = app.monitor.cpu_history();
    if !history.is_empty() {
        let history_u64: Vec<u64> = history.iter().map(|&v| v as u64).collect();
        let sparkline = Sparkline::default()
            .data(&history_u64)
            .style(Style::default().fg(gauge_color))
            .max(100);

        f.render_widget(sparkline, cpu_chunks[1]);
    }
}

fn render_memory(f: &mut Frame, area: Rect, app: &App) {
    let theme = &app.config.theme;
    let mem_stats = app.monitor.memory_stats();

    let border_type = if app.config.rounded_borders {
        ratatui::widgets::BorderType::Rounded
    } else {
        ratatui::widgets::BorderType::Plain
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(border_type)
        .border_style(Style::default().fg(theme.border()))
        .title(Span::styled(
            format!(" Memory: {:.1}% ", mem_stats.used_percent()),
            Style::default().fg(theme.primary()).add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(theme.bg()));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let ram_color = theme.usage_color(mem_stats.used_percent());
    let ram_available = mem_stats.total_gb() - mem_stats.used_gb();
    
    if mem_stats.swap_total > 0 {
        let mem_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Length(2),
            ])
            .horizontal_margin(1)
            .vertical_margin(1)
            .split(inner);

        let ram_info = vec![
            Line::from(vec![
                Span::styled("RAM: ", Style::default().fg(theme.secondary()).add_modifier(Modifier::BOLD)),
                Span::styled(
                    format!("{:.1} GB used • {:.1} GB free • {:.1}%", 
                        mem_stats.used_gb(), 
                        ram_available,
                        mem_stats.used_percent()
                    ),
                    Style::default().fg(theme.fg()),
                ),
            ]),
        ];
        
        let ram_paragraph = Paragraph::new(ram_info).style(Style::default().bg(theme.bg()));
        f.render_widget(ram_paragraph, mem_chunks[0]);

        let ram_gauge = Gauge::default()
            .gauge_style(Style::default().fg(ram_color).add_modifier(Modifier::BOLD))
            .ratio((mem_stats.used_percent() as f64 / 100.0).clamp(0.0, 1.0))
            .label("");

        f.render_widget(ram_gauge, Rect::new(
            mem_chunks[0].x,
            mem_chunks[0].y + 1,
            mem_chunks[0].width,
            1,
        ));

        let swap_color = theme.usage_color(mem_stats.swap_percent());
        let swap_available = mem_stats.swap_total_gb() - mem_stats.swap_used_gb();
        
        let swap_info = vec![
            Line::from(vec![
                Span::styled("Swap: ", Style::default().fg(theme.secondary()).add_modifier(Modifier::BOLD)),
                Span::styled(
                    format!("{:.1} GB used • {:.1} GB free • {:.1}%", 
                        mem_stats.swap_used_gb(), 
                        swap_available,
                        mem_stats.swap_percent()
                    ),
                    Style::default().fg(theme.fg()),
                ),
            ]),
        ];
        
        let swap_paragraph = Paragraph::new(swap_info).style(Style::default().bg(theme.bg()));
        f.render_widget(swap_paragraph, mem_chunks[2]);

        let swap_gauge = Gauge::default()
            .gauge_style(Style::default().fg(swap_color).add_modifier(Modifier::BOLD))
            .ratio((mem_stats.swap_percent() as f64 / 100.0).clamp(0.0, 1.0))
            .label("");

        f.render_widget(swap_gauge, Rect::new(
            mem_chunks[2].x,
            mem_chunks[2].y + 1,
            mem_chunks[2].width,
            1,
        ));
    } else {
        let mem_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2)])
            .horizontal_margin(1)
            .vertical_margin(1)
            .split(inner);

        let ram_info = vec![
            Line::from(vec![
                Span::styled("RAM: ", Style::default().fg(theme.secondary()).add_modifier(Modifier::BOLD)),
                Span::styled(
                    format!("{:.1} GB used • {:.1} GB free • {:.1}%", 
                        mem_stats.used_gb(), 
                        ram_available,
                        mem_stats.used_percent()
                    ),
                    Style::default().fg(theme.fg()),
                ),
            ]),
        ];
        
        let ram_paragraph = Paragraph::new(ram_info).style(Style::default().bg(theme.bg()));
        f.render_widget(ram_paragraph, mem_chunks[0]);

        let ram_gauge = Gauge::default()
            .gauge_style(Style::default().fg(ram_color).add_modifier(Modifier::BOLD))
            .ratio((mem_stats.used_percent() as f64 / 100.0).clamp(0.0, 1.0))
            .label("");

        f.render_widget(ram_gauge, Rect::new(
            mem_chunks[0].x,
            mem_chunks[0].y + 1,
            mem_chunks[0].width,
            1,
        ));
    }
}

fn render_disk_network(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    render_disks(f, chunks[0], app);
    render_network(f, chunks[1], app);
}

fn render_disks(f: &mut Frame, area: Rect, app: &App) {
    let theme = &app.config.theme;
    let disk_stats = app.monitor.disk_stats();

    let border_type = if app.config.rounded_borders {
        ratatui::widgets::BorderType::Rounded
    } else {
        ratatui::widgets::BorderType::Plain
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(border_type)
        .border_style(Style::default().fg(theme.border()))
        .title(Span::styled(
            " Disk Usage ",
            Style::default().fg(theme.primary()).add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(theme.bg()));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let disk_items: Vec<ListItem> = disk_stats
        .iter()
        .enumerate()
        .map(|(idx, disk)| {
            let disk_label = if disk.name.is_empty() {
                format!("HD{}", idx + 1)
            } else {
                let name = disk.name.replace("\\", "").replace(".", "");
                if name.len() > 4 {
                    format!("{}", &name[..4])
                } else {
                    name
                }
            };

            let bar_width: usize = 15;
            let filled = ((disk.used_percent() / 100.0) * bar_width as f32) as usize;
            let empty = bar_width.saturating_sub(filled);
            let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));

            let color = theme.usage_color(disk.used_percent());

            ListItem::new(Line::from(vec![
                Span::styled(format!("{:<4} ", disk_label), Style::default().fg(theme.secondary()).add_modifier(Modifier::BOLD)),
                Span::styled(format!("[{}] ", disk.disk_type), Style::default().fg(theme.primary())),
                Span::styled(bar, Style::default().fg(color)),
                Span::styled(
                    format!(" {:.0}/{:.0}GB", disk.used_gb(), disk.total_gb()),
                    Style::default().fg(theme.fg()),
                ),
            ]))
        })
        .collect();

    let list = List::new(disk_items).style(Style::default().bg(theme.bg()));

    f.render_widget(list, inner);
}

fn render_network(f: &mut Frame, area: Rect, app: &App) {
    let theme = &app.config.theme;
    let network = &app.network;

    let border_type = if app.config.rounded_borders {
        ratatui::widgets::BorderType::Rounded
    } else {
        ratatui::widgets::BorderType::Plain
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(border_type)
        .border_style(Style::default().fg(theme.border()))
        .title(Span::styled(
            " Network ",
            Style::default().fg(theme.primary()).add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(theme.bg()));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let network_info = vec![
        Line::from(vec![
            Span::styled("↓ Download: ", Style::default().fg(theme.success()).add_modifier(Modifier::BOLD)),
            Span::styled(
                crate::system::NetworkStats::format_rate(network.rx_rate_mbps() * 1_024.0 * 1_024.0),
                Style::default().fg(theme.fg()),
            ),
        ]),
        Line::from(vec![
            Span::styled("↑ Upload: ", Style::default().fg(theme.danger()).add_modifier(Modifier::BOLD)),
            Span::styled(
                crate::system::NetworkStats::format_rate(network.tx_rate_mbps() * 1_024.0 * 1_024.0),
                Style::default().fg(theme.fg()),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Total RX: ", Style::default().fg(theme.secondary())),
            Span::styled(format!("{:.2} GB", network.total_received_gb()), Style::default().fg(theme.fg())),
        ]),
        Line::from(vec![
            Span::styled("Total TX: ", Style::default().fg(theme.secondary())),
            Span::styled(format!("{:.2} GB", network.total_transmitted_gb()), Style::default().fg(theme.fg())),
        ]),
    ];

    let paragraph = Paragraph::new(network_info)
        .style(Style::default().bg(theme.bg()))
        .block(Block::default());

    f.render_widget(paragraph, inner);
}

fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    let theme = &app.config.theme;

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(30), Constraint::Length(20)])
        .split(area);

    let footer_text = Line::from(vec![
        Span::styled(" [Q]", Style::default().fg(theme.primary()).add_modifier(Modifier::BOLD)),
        Span::styled(" Quit  ", Style::default().fg(theme.fg())),
        Span::styled("[R]", Style::default().fg(theme.primary()).add_modifier(Modifier::BOLD)),
        Span::styled(" Refresh  ", Style::default().fg(theme.fg())),
        Span::styled("[T]", Style::default().fg(theme.primary()).add_modifier(Modifier::BOLD)),
        Span::styled(" Theme ", Style::default().fg(theme.fg())),
    ]);

    let footer = Paragraph::new(footer_text)
        .style(Style::default().bg(theme.bg()).fg(theme.fg()))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(if app.config.rounded_borders {
                    ratatui::widgets::BorderType::Rounded
                } else {
                    ratatui::widgets::BorderType::Plain
                })
                .border_style(Style::default().fg(theme.border())),
        );

    let credit_text = Line::from(vec![
        Span::styled(" made by ", Style::default().fg(theme.fg())),
        Span::styled("gui기กีギ", Style::default().fg(theme.primary()).add_modifier(Modifier::BOLD)),
    ]);

    let credit = Paragraph::new(credit_text)
        .style(Style::default().bg(theme.bg()).fg(theme.fg()))
        .alignment(ratatui::layout::Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(if app.config.rounded_borders {
                    ratatui::widgets::BorderType::Rounded
                } else {
                    ratatui::widgets::BorderType::Plain
                })
                .border_style(Style::default().fg(theme.border())),
        );

    f.render_widget(footer, footer_chunks[0]);
    f.render_widget(credit, footer_chunks[1]);
}

