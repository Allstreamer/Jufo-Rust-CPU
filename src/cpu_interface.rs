use super::cpu::Cpu;
use super::instruction_table::draw_instruction_table;
use super::rom_editor::RomEditor;
use eframe::epi::{App, Frame};
use egui::Context;
pub struct CPUInterface {
    pub cpu: Cpu,
    pub editor: RomEditor,
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
