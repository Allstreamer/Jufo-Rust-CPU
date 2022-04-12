use eframe::epi::{App, Frame};
use eframe::{NativeOptions, run_native};
use egui::Context;

mod rom_editor;
mod cpu;

use cpu::Cpu;
use rom_editor::RomEditor;

struct CPUInterface {
    cpu: Cpu,
    editor: RomEditor
}

impl App for CPUInterface {
    fn update(&mut self, ctx: &Context, frame: &Frame) {
        ctx.request_repaint();

        self.cpu.draw(ctx);
        self.cpu.ui_update();

        self.editor.draw(ctx);
    }

    fn name(&self) -> &str {
        "Cpu Interface"
    }
}

fn main() {
    let mut app = CPUInterface {
        cpu: Cpu::new(),
        editor: RomEditor {
            ..Default::default()
        }
    };
    app.cpu.rom[0] = 16;
    app.cpu.rom[1] = 5;
    let window_options = NativeOptions {
        ..Default::default()
    };
    run_native(Box::new(app), window_options);
}
