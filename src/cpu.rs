use egui::{Color32, Context, RichText, ScrollArea, Window};
use egui::Label;

/// Enum used to control the program-counter after instruction execution
pub enum PCAction {
    Step,
    Jump(usize),
}

// Defines Cpu Structure and it's values
pub struct Cpu {
    pub pc: usize,
    pub memory: [u8; 4096],
    pub rom: [u8; 4096],
    pub acc: u8,
    pub running: bool,
}

impl Cpu {
    /// Used to instantiate a new cpu instance
    pub fn new() -> Cpu {
        Cpu {
            pc: 0,
            rom: [0; 4096],
            memory: [0; 4096],
            acc: 0,
            running: false,
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.memory = self.rom;
        self.acc = 0;
        self.running = false;
    }

    pub fn draw(&mut self, ctx: &Context) {
        Window::new("CPU Debug Interface").show(ctx, |ui| {

            // Controls
            ui.horizontal(|ui| {
                if ui.button(
                    RichText::new("Step").size(20.0).strong()
                ).clicked() {
                    self.step();
                }
                if ui.button(
                    RichText::new("Start").size(20.0).strong()
                ).clicked() {
                    self.running = true;
                }
                if ui.button(
                    RichText::new("Stop").size(20.0).strong()
                ).clicked() {
                    self.running = false;
                }
                if ui.button(
                    RichText::new("Reset").size(20.0).strong()
                        .background_color(Color32::from_rgb(55,0,0))
                ).clicked() {
                    self.reset()
                }
            });

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("Accumulator: {}", self.acc))
                        .size(28.0)
                );
                ui.label(
                    RichText::new(format!("Program Counter: {}", self.pc*2))
                        .size(28.0)
                );
            });
            ui.add_space(10.0);

            let current_address = self.pc * 2;
            let operation: u8 = (self.memory[current_address] & 0xF0) >> 4;
            let value: u8 = self.memory[current_address + 1];
            let address: u16 = ((self.memory[current_address] & 0x0F) as u16) << 8 | value as u16;
            ui.label(
                RichText::new(format!("Befehl: {:08b}", operation))
                    .size(20.0)
            );
            ui.label(
                RichText::new(format!("Wert: {:08b}", value))
                    .size(20.0)
            );
            ui.label(
                RichText::new(format!("Adresse: {:012b}", address))
                    .size(20.0)
            );

            ui.add_space(10.0);

            // Ram Watch
            ScrollArea::vertical().show(ui, |ui| {
                for (i, mem_val)in self.memory.iter().enumerate().step_by(16) {
                    ui.horizontal_top(|ui| {
                        ui.label(
                            RichText::new(format!("{:#06x}", i))
                                .raised()
                        );
                        for j in 0..16 {
                            let mut txt = RichText::new(format!("{}", self.memory[i+j]))
                                .size(21.0);

                            if i+j == self.pc * 2 || (i+j).wrapping_sub(1) == (self.pc * 2) {
                                txt = txt.background_color(
                                    Color32::from_rgb(0,100,0)
                                )
                            }
                            ui.label(
                                txt
                            );
                        }
                    });
                }
            });
        });
    }

    /// For ui related Data Updates / Function calls
    pub fn ui_update(&mut self) {
        if self.running {
            self.step()
        }
    }

    /// This is the heart of the cpu it performs the
    /// Fetch Decode Execute Cycle
    pub fn step(&mut self) {
        // Since we use two bytes as our address
        // we need to double out pc
        let current_address = self.pc * 2;

        // Fetch
        let operation: u8 = (self.memory[current_address] & 0xF0) >> 4;
        let value: u8 = self.memory[current_address + 1];
        let address: u16 = ((self.memory[current_address] & 0x0F) as u16) << 8 | value as u16;

        // Decode & Execute
        // Opcode check
        let pc_op = match operation {
            0x0 => { PCAction::Step } // No Op

            // Enter a Value into the math Register
            0x1 => { self.set_value(value) } // set-value

            // Load a value From ram into the math register
            0x2 => { self.load(address) } // Load
            // Save a value from the math register into ram
            0x3 => { self.save(address) } // Save

            // Add a number to the math register
            0x4 => { self.add(value) } // Add
            // Subtract a number from the math register
            0x5 => { self.subtract(value) } // Subtract

            // Add number at address to the math register
            0x6 => { self.add_mem(address) } // Add
            // Subtract number at address from the math register
            0x7 => { self.subtract_mem(address) } // Subtract

            // Increase the value at address
            0x8 => { self.increase(address) } // Increase Memory Value
            // Decrease Value at address
            0x9 => { self.decrease(address) } // Decrease Memory Value

            // Jump If the number in the math Register is not Zero
            0xA => { self.jmp_inz(address) } // Jump If not Zero
            // Jump if the number in the Math register is zero
            0xB => { self.jmp_iz(address) } // Jump If Zero
            // Jump If Carry
            0xC => {
                PCAction::Step
            }
            // Unconditional Jump
            0xD => {
                PCAction::Jump(address as usize)
            }

            // Write External
            0xE => {
                PCAction::Step
            }
            // Read External
            0xF => {
                PCAction::Step
            }
            _ => { PCAction::Step }
        };

        // Check what action to take next
        // based on the result of the opcode check
        match pc_op {
            PCAction::Step => {
                if self.pc+1 > 4095 / 2 {
                    self.pc = 0
                }else {
                    self.pc += 1;
                }
            }
            PCAction::Jump(addr) => {
                if addr > 4095 / 2 {
                    self.pc = 0;
                }else {
                    self.pc = addr;
                }
            }
        }
        if self.pc > self.memory.len() {
            self.pc = 0;
        }
    }

    /// Takes a reference to an u8 array and loads it's values
    /// into memory
    pub fn load_rom(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.memory[i] = byte;
        }
    }

    /// Gets the last 8 bits of the current instruction and
    /// inserts them into the math register
    pub fn set_value(&mut self, value: u8) -> PCAction {
        println!("set_value");
        self.acc = value;
        return PCAction::Step;
    }

    /// Loads value from memory, based on the 12-bit address provided,
    /// into math register
    pub fn load(&mut self, address: u16) -> PCAction {
        println!("load");
        self.acc = self.memory[address as usize];
        return PCAction::Step;
    }

    /// Saves Value from math register into memory,
    /// based on the 12-bit address provided.
    pub fn save(&mut self, address: u16) -> PCAction {
        println!("save");
        self.memory[address as usize] = self.acc;
        return PCAction::Step;
    }

    /// Adds Provided 8-bit number to the math register
    pub fn add(&mut self, value: u8) -> PCAction {
        println!("add");
        self.acc += value;
        return PCAction::Step;
    }

    /// Subtracts Provided 8-bit number to the math register
    pub fn subtract(&mut self, value: u8) -> PCAction {
        println!("subtract");
        self.acc -= value;
        return PCAction::Step;
    }

    /// Adds Provided 8-bit number to the math register
    pub fn add_mem(&mut self, address: u16) -> PCAction {
        println!("add_mem");
        self.acc += self.memory[address as usize];
        return PCAction::Step;
    }

    /// Subtracts Provided 8-bit number to the math register
    pub fn subtract_mem(&mut self, address: u16) -> PCAction {
        println!("subtract_mem");
        self.acc -= self.memory[address as usize];
        return PCAction::Step;
    }

    /// Increase Value in memory, based on the 12-bit address provided.
    pub fn increase(&mut self, address: u16) -> PCAction {
        println!("increase");
        self.memory[address as usize] += 1;
        return PCAction::Step;
    }

    /// Decrease Value in memory, based on the 12-bit address provided.
    pub fn decrease(&mut self, address: u16) -> PCAction {
        println!("decrease");
        self.memory[address as usize] -= 1;
        return PCAction::Step;
    }

    /// Jumps to the 12-bit address if the number in the math register is not zero
    pub fn jmp_inz(&mut self, address: u16) -> PCAction {
        println!("jmp_inz");
        if self.acc != 0 {
            return PCAction::Jump(address as usize);
        }

        return PCAction::Step;
    }

    /// Jumps to the 12-bit address if the number in the math register is zero
    pub fn jmp_iz(&mut self, address: u16) -> PCAction {
        println!("jmp_iz");
        if self.acc == 0 {
            return PCAction::Jump(address as usize);
        }

        return PCAction::Step;
    }
}