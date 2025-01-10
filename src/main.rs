// Copyright (c) 2025 Earture
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#![windows_subsystem = "windows"]

use eframe::egui;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};
use calamine::{Reader, Xlsx, DataType};
use std::thread;
use crossbeam_channel::{unbounded, Sender};
use threadpool::ThreadPool;
use chrono::Local;
use std::time::Instant;
use dns_lookup::lookup_host; // 用于解析域名为 IP 地址

use ping as ping_crate; // 避免跟本地 fn ping 重名
use std::net::IpAddr;
use std::time::Duration;

#[derive(Clone, Debug)]
struct IpData {
    ip: String,
    name: String,
    location: String,
    ping_result: String,
    last_updated: String,
    ping_time: String,
}

#[derive(Default)]
struct AppState {
    ip_table: Arc<Mutex<Vec<IpData>>>,
    enable_monitoring: bool,
    is_monitoring_active: bool,
}

struct PingMonitorApp {
    state: Arc<Mutex<AppState>>,
    sender: Sender<IpData>,
}

impl Clone for PingMonitorApp {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
            sender: self.sender.clone(),
        }
    }
}

impl PingMonitorApp {
    fn new() -> Self {
        let state = Arc::new(Mutex::new(AppState::default()));
        let (sender, receiver) = unbounded();

        let app = Self {
            state,
            sender: sender.clone(),
        };

        let state = app.state.clone();
        // 后台线程: 接收由监控线程发回的 IpData，更新全局表
        thread::spawn(move || {
            for ip_data in receiver {
                let state_lock = state.lock().unwrap();
                let mut ip_table = state_lock.ip_table.lock().unwrap();

                if let Some(entry) = ip_table.iter_mut().find(|data| data.ip == ip_data.ip) {
                    *entry = ip_data;
                }
            }
        });

        app
    }

    fn load_excel(&self, file_path: &str) {
        let state = self.state.clone();
        let file_path = file_path.to_string();
        thread::spawn(move || {
            let mut excel: Xlsx<_> =
                calamine::open_workbook(file_path).expect("Cannot open Excel file");
            if let Some(Ok(range)) = excel.worksheet_range("Sheet1") {
                let state_lock = state.lock().unwrap();
                let mut ip_table = state_lock.ip_table.lock().unwrap();
                ip_table.clear();
                for row in range.rows().skip(1) {
                    // 假设 Excel A列=IP, B列=Name C列=Location,
                    if let [DataType::String(ip),DataType::String(name),DataType::String(location), ] =
                        row
                    {
                        ip_table.push(IpData {
                            ip: ip.clone(),
                            location: location.clone(),
                            name: name.clone(),
                            ping_result: "Pending".to_string(),
                            last_updated: "N/A".to_string(),
                            ping_time: "N/A".to_string(),
                        });
                    }
                }
            }
        });
    }

    fn toggle_monitoring(&self) {
        let state = self.state.clone();
        let app = self.clone();
        thread::spawn(move || {
            let mut state_lock = state.lock().unwrap();
            state_lock.enable_monitoring = !state_lock.enable_monitoring;
            state_lock.is_monitoring_active = state_lock.enable_monitoring;

            if state_lock.enable_monitoring {
                println!("Monitoring started");
                drop(state_lock);
                app.start_monitoring();
            } else {
                println!("Monitoring stopped");
            }
        });
    }

    fn start_monitoring(&self) {
        let state = self.state.clone();
        let sender = self.sender.clone();
        let pool = ThreadPool::new(8);

        thread::spawn(move || loop {
            {
                let state_lock = state.lock().unwrap();
                if !state_lock.enable_monitoring {
                    break;
                }

                let ip_table = state_lock.ip_table.lock().unwrap().clone();
                for ip_data in ip_table {
                    let sender = sender.clone();
                    pool.execute(move || {
                        let start_time = Instant::now();
                        let (result, is_timeout) = PingMonitorApp::ping(&ip_data.ip);
                        let duration = start_time.elapsed().as_millis();

                        // 如果这次 ping 超时，但上次状态不是 "Timeout" 时，才更新 last_updated
                        // 如果连续超时，则 last_updated 不变
                        let last_updated = if !is_timeout && ip_data.ping_result != "Timeout" {
                            Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
                        } else if is_timeout {
                            ip_data.last_updated.clone()
                        } else {
                            // ping 成功，也更新 last_updated
                            Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
                        };

                        let ping_time = if is_timeout {
                            "Timeout".to_string()
                        } else {
                            format!("{} ms", duration)
                        };

                        let updated_data = IpData {
                            ping_result: result,
                            last_updated,
                            ping_time,
                            ..ip_data
                        };

                        sender.send(updated_data).unwrap();
                    });
                }
            }

            // 每 5 秒循环一次
            thread::sleep(std::time::Duration::from_secs(5));
        });
    }

    fn ping(ip_or_domain: &str) -> (String, bool) {
        // 先尝试解析为 IP 地址
        if let Ok(ip_addr) = ip_or_domain.parse::<IpAddr>() {
            // 如果是 IP 地址，直接进行 ping
            return Self::ping_ip(ip_addr);
        }
    
        // 如果不是 IP 地址，尝试将其解析为域名
        match lookup_host(ip_or_domain) {
            Ok(addrs) => {
                // 解析成功后，取第一个 IP 地址进行 ping
                if let Some(ip_addr) = addrs.into_iter().next() {
                    return Self::ping_ip(ip_addr);
                } else {
                    return ("No valid IP found".to_string(), true);
                }
            }
            Err(_) => ("Invalid domain".to_string(), true),
        }
    }
    
    // 辅助方法，用于 ping 具体的 IP 地址
    fn ping_ip(ip_addr: IpAddr) -> (String, bool) {
        // 设置超时 700ms
        let timeout = Some(Duration::from_millis(700));
        // 这里全部使用 None，让库自动填充默认值。
        let ret = ping_crate::ping(ip_addr, timeout, None, None, None, None);
    
        match ret {
            // 成功时返回 Ok(())，说明目标可达
            Ok(()) => ("Success".to_string(), false),
            // 失败时可能是 "timed out" 或其他错误
            Err(e) => {
                let err_str = e.to_string();
                if err_str.contains("timed out") {
                    ("Timeout".to_string(), true)
                } else {
                    // 其他错误也可视作超时/不可达
                    ("Error".to_string(), true)
                }
            }
        }
    }
    
}

impl eframe::App for PingMonitorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let state = self.state.clone();

        // 获取当前窗口的大小
        let available_size = ctx.available_rect().size();
        let table_width = available_size.x ; // 表格宽度为窗口宽度的95%
        let table_height = available_size.y ; // 表格高度为窗口高度的80%

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // 动态更新按钮文本
                let state_lock = state.lock().unwrap();
                let button_label = if state_lock.is_monitoring_active {
                    egui::RichText::new("Stop Monitoring").color(egui::Color32::from_rgb(88, 178, 220)).strong()
                    
                } else {
                    egui::RichText::new("Start Monitoring").color(egui::Color32::from_rgb(254, 223, 255)).strong()
                    
                };

                if ui.button(button_label).clicked() {
                    self.clone().toggle_monitoring();
                }

                if ui.button("Load Excel").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.clone().load_excel(path.to_str().unwrap());
                    }
                }

            });

            // 动态调整表格区域大小
            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .max_width(table_width) // 设置表格最大宽度
                .max_height(table_height) // 设置表格最大高度
                .show(ui, |ui| {
                    let state_lock = state.lock().unwrap();
                    let ip_table = state_lock.ip_table.lock().unwrap();
                    let mut sorted_table = ip_table.clone();
                    sorted_table.sort_by(|a, b| {
                        if a.ping_time == "Timeout" && b.ping_time != "Timeout" {
                            std::cmp::Ordering::Less
                        } else if a.ping_time != "Timeout" && b.ping_time == "Timeout" {
                            std::cmp::Ordering::Greater
                        } else {
                            std::cmp::Ordering::Equal
                        }
                    });

                    egui::Grid::new("ip_table")
    .striped(true)
    .min_col_width(table_width / 6.0) // 动态调整每列的宽度
    .show(ui, |ui| {
        ui.label("IP/Web");
        ui.label("Name");
        ui.label("Location");
        ui.label("Ping Time");
        ui.label("Last Updated");
        ui.end_row();

        for ip_data in sorted_table.iter() {
            ui.label(&ip_data.ip);
            ui.label(&ip_data.name);
            ui.label(&ip_data.location);
            // Ping Time 颜色逻辑
            if let Ok(ping_time) = ip_data.ping_time.trim_end_matches(" ms").parse::<u32>() {
                if ping_time < 100 {
                    ui.colored_label(egui::Color32::from_rgb(22, 198, 12), &ip_data.ping_time);
                } else {
                    ui.colored_label(egui::Color32::from_rgb(251, 226, 81), &ip_data.ping_time);
                }
            } else { 
                // 如果无法解析 ping_time，例如 "Timeout"
                if &ip_data.ping_time == "Timeout" {
                    ui.colored_label(egui::Color32::from_rgb(232, 48, 21),&ip_data.ping_time);
                } else {
                    ui.colored_label(egui::Color32::from_rgb(224, 60, 138),&ip_data.ping_time);
                }
            }
            
            ui.label(&ip_data.last_updated);
            ui.end_row();
        }
    });
                });
        });

        ctx.request_repaint();
    }
}

fn setup_system_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    let font_path = if cfg!(target_os = "windows") {
        "C:\\Windows\\Fonts\\msyh.ttc"
    } else if cfg!(target_os = "macos") {
        "/System/Library/Fonts/PingFang.ttc"
    } else if cfg!(target_os = "linux") {
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc"
    } else {
        panic!("Unsupported operating system");
    };

    if !Path::new(font_path).exists() {
        panic!("System font not found at: {}", font_path);
    }

    let mut font_data = Vec::new();
    File::open(font_path)
        .expect("Failed to open font file")
        .read_to_end(&mut font_data)
        .expect("Failed to read font file");

    fonts.font_data.insert(
        "system_font".to_owned(),
        egui::FontData::from_owned(font_data),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "system_font".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "system_font".to_owned());

    ctx.set_fonts(fonts);
}

fn main() -> Result<(), eframe::Error> {
    // 图标嵌入
    let icon_data = include_bytes!("app.png");
    let icon_image = image::load_from_memory(icon_data)
        .expect("Failed to load app icon")
        .into_rgba8();

    let (width, height) = icon_image.dimensions();
    let icon_rgba = icon_image.into_raw();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(700.0, 300.0)),
        icon_data: Some(eframe::IconData {
            rgba: icon_rgba,
            width,
            height,
        }),
        ..Default::default()
    };

    let app = PingMonitorApp::new();

    eframe::run_native(
        // You Can Change It，of course.
        "KaniPing v0.1.0 - A Ping Tool Written in Rust.",  
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            setup_system_fonts(&cc.egui_ctx);
            Box::new(app)
        })
    )
}
