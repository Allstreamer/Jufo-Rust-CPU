use eframe::epi::{App, Frame};
use eframe::{run_native, NativeOptions};
use egui::Context;

mod cpu;
mod instruction_table;
mod rom_editor;

use cpu::Cpu;
use instruction_table::draw_instruction_table;
use rom_editor::RomEditor;

struct CPUInterface {
    cpu: Cpu,
    editor: RomEditor,
}

impl App for CPUInterface {
    fn update(&mut self, ctx: &Context, _frame: &Frame) {
        ctx.request_repaint();

        self.cpu.draw(ctx);
        self.cpu.ui_update();

        self.editor.draw(ctx);
        if self.editor.code != self.editor.last_code {
            self.editor.update();
        }
        if self.editor.valid {
            self.cpu.load_sixteen_bit_rom(&self.editor.rom);
        }

        draw_instruction_table(ctx);
    }

    fn name(&self) -> &str {
        "Cpu Interface"
    }
}

fn main() {
    let app = CPUInterface {
        cpu: Cpu::new(),
        editor: RomEditor {
            ..Default::default()
        },
    };
    let window_options = NativeOptions {
        ..Default::default()
    };
    run_native(Box::new(app), window_options);
}
