use egui::{Context, Grid, Window};

pub fn draw_instruction_table(ctx: &Context) {
    Window::new("Instruction Table").show(ctx, |ui| {
        Grid::new("instruction_table").show(ui, |ui| {
            let ops = vec![
                ("noop", "Does nothing"),
                ("set", "Inserts Value into accumulator"),
                ("load", "Loads value from memory, based on the 12-bit address provided, into math register"),
                ("save", "Saves Value from math register into memory, based on the 12-bit address provided."),
                ("add", "Adds Provided 8-bit number to the math register"),
                ("sub", "Subtracts Provided 8-bit number to the math register"),
                ("add_mem", "Add number at address to the math register"),
                ("sub_mem", "Subtract number at address from the math register"),
                ("inc", "Increase Value in memory, based on the 12-bit address provided."),
                ("dec", "Decrease Value in memory, based on the 12-bit address provided."),
                ("jmp_inz", "Jumps to the 12-bit address if the number in the math register is not zero"),
                ("jmp_iz", "Jumps to the 12-bit address if the number in the math register is zero"),
                ("jmp", "Jumps to the 12-bit address unconditionally"),
                ("jmp_ic", "Jumps to the 12-bit address if the carry flag is set"),
                ("wext", "(WIP) Write to external device"),
                ("rext", "(WIP) Read from external device"),
            ];

            for (i, val) in ops.iter().enumerate() {
                ui.label(val.0);
                ui.label(format!("0x{:1x}", i));
                ui.label(val.1);
                ui.end_row();
            }
        });
    });
}
