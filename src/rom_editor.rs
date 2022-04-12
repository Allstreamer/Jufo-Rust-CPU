use egui::{Context, Id, ScrollArea, TextEdit, TextStyle, Window};
use egui::plot::Text;

pub struct RomEditor {
    pub code: String
}

impl RomEditor {
    pub fn draw(&mut self, ctx: &Context) {
        Window::new("Rom Writer").show(ctx, |ui| {
            // Editor
            ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    TextEdit::multiline(&mut self.code)
                        .font(TextStyle::Monospace)
                        .code_editor()
                        .lock_focus(true)
                        .desired_width(f32::INFINITY)
                        .desired_rows(23)
                )
            });

            let mut filtered: String =
                self.code.lines().into_iter()
                .filter(|x| {
                    !x.starts_with("#") && !x.is_empty()
                }).collect::<Vec<&str>>()
                    .join("\n");

            // Filter Preview
            ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    TextEdit::multiline(&mut filtered)
                        .font(TextStyle::Monospace)
                        .code_editor()
                        .lock_focus(true)
                        .desired_width(f32::INFINITY)
                        .desired_rows(23)
                )
            });
        });
    }
}

impl Default for RomEditor {
    fn default() -> Self {
        Self {
            code: "\
# Multiplyer

# Set First Number to multiply into register 40
set 7
save 40

# Set Second Number to multiply into register 41
set 10
save 41

load 42
add_mem 40
save 42
decrease 41
load 41

jmp_inz 4
load 42

# Save Result into Acc and stay
load 42".into()
        }
    }
}
