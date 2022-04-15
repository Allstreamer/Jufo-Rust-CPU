use egui::{Context, RichText, ScrollArea, TextEdit, TextStyle, Window};

pub struct RomEditor {
    pub code: String,
    pub last_code: String,
    pub filtered: Vec<String>,
    pub rom: Vec<u16>,
    pub valid: bool,
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
                        .desired_rows(23),
                )
            });
        });

        Window::new("Filter View").show(ctx, |ui| {
            // Filter Preview
            ScrollArea::vertical().show(ui, |ui| {
                for (i, line) in (&self.filtered).iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}", i));
                        ui.label(RichText::new(line));
                    });
                }
            });
        });

        Window::new("Rom View").show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                for (i, n) in self.rom.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}", i));
                        ui.label(format!("{}", (n >> 8) as u8));
                        ui.label(format!("{}", *n as u8));
                    });
                }
            });
        });
    }

    pub fn update(&mut self) {
        if self.code != self.last_code {
            self.last_code = self.code.clone();

            self.filtered = self
                .code
                .lines()
                .into_iter()
                .filter(|x| !x.starts_with('#') && !x.is_empty())
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            let mut binary: Vec<u16> = Vec::new();
            for line in &self.filtered {
                let (operation, arg) = match line.split_once(' ') {
                    None => return,
                    Some(v) => v,
                };

                println!("{} {}", operation, arg);
                let arg_decode = match arg.trim().parse::<u16>() {
                    Ok(v) => v,
                    Err(_) => return,
                };
                let op: u16 = (match operation {
                    "noop" => 0x0,
                    "set" => 0x1,
                    "load" => 0x2,
                    "save" => 0x3,
                    "add" => 0x4,
                    "subtract" => 0x5,
                    "add_mem" => 0x6,
                    "subtract_mem" => 0x7,
                    "inc" => 0x8,
                    "dec" => 0x9,
                    "jmp_inz" => 0xA,
                    "jmp_iz" => 0xB,
                    "jmp_ic" => 0xC,
                    "jmp" => 0xD,
                    "wext" => 0xE,
                    "rext" => 0xF,
                    _ => {
                        continue;
                    }
                } as u16)
                    << 12;
                binary.push(op | arg_decode);
            }
            self.rom = binary;
        }
        self.valid = true;
    }
}

impl Default for RomEditor {
    fn default() -> Self {
        let default_code = String::from("\
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
dec 41
load 41

jmp_inz 4
load 42

# Save Result into Acc and stay
load 42
jmp 12");

        Self {
            code: default_code.clone(),
            last_code: default_code,
            filtered: vec![],
            rom: vec![],
            valid: false,
        }
    }
}
