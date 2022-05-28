use eframe::{run_native, NativeOptions};

mod cpu;
mod cpu_interface;
mod instruction_table;
mod rom_editor;

use cpu::Cpu;
use cpu_interface::CPUInterface;
use rom_editor::RomEditor;

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
