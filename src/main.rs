#![allow(non_snake_case, non_camel_case_types)]

use eframe::egui;
use sysinfo::System;
use std::fs;
use std::io::{Read, Write};
use std::process::Command;

fn main() -> eframe::Result<()> {
    Command::new("sudo")
        .arg("-v")
        .output()
        .expect("Please run as root.");
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("CPU_Controller", 
        native_options, 
        Box::new(|cc| 
                Box::new(CPU_Controller::new(cc))))
}

#[derive(Default)]
struct CPU_Controller {}

impl CPU_Controller {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self::default()
    }
}

impl eframe::App for CPU_Controller {
    fn update(&mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let sys = System::new();
            let cores = sys.physical_core_count()
                .expect("idk");
            ui.heading("CPU Controller!");
            for i in 1..cores {
                let mut ibool = checkCore(&i);
                ui.checkbox(&mut ibool, format!("CPU Core {}", i));
                if ibool != checkCore(&i) {
                    toggleCore(&i); 
                } else {}
            }
        });
    }
}

fn checkCore(core: &usize) -> bool {
    let content: String = fs::read_to_string(format!("/sys/devices/system/cpu/cpu{core}/online"))
        .expect("save me");
    let content: u8 = content.trim()
        .parse()
        .expect("Idk");
    if content == 1 { 
        return true 
    } else { 
        return false
     }
}

fn toggleCore(core: &usize) {
    let mut file1 = fs::OpenOptions::new().write(true)
        .read(true)
        .open(format!("/sys/devices/system/cpu/cpu{core}/online"))
        .expect("Failed for reasons");
    let mut content = String::new();
    file1.read_to_string(&mut content)
        .expect("Save me");
    let content: u8 = content.trim().parse()
        .expect("idk");
    if content == 1 {
        file1.write_all(b"0")
            .expect("whar?");
    } else {
        file1.write_all(b"1")
            .expect("whar?");
    }
}