use egui::{Context, Grid, Window};

pub fn draw_instruction_table(ctx: &Context) {
    Window::new("Instruction Table").show(ctx, |ui| {
        Grid::new("instruction_table").show(ui, |ui| {
            let ops = vec![
                ("noop", "Does nothing"),
                ("set", "Inserts Value into accumulator"),
                ("load", ""),
                ("save", ""),
                ("add", ""),
                ("sub", ""),
                ("add_mem", ""),
                ("sub_mem", ""),
                ("inc", ""),
                ("dec", ""),
                ("jmp_inz", ""),
                ("jmp_iz", ""),
                ("jmp", ""),
                ("jmp_ic", ""),
                ("wext", ""),
                ("rext", ""),
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
