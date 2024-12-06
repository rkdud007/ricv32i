use std::thread;

use cpu::CPU;
use glob::glob;
use rv32i_lib::*;

fn main() {
    let pattern = "riscv-tests/isa/rv32ui-*";
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if path.extension().and_then(|ext| ext.to_str()) == Some("dump") {
                    continue;
                } else {
                    let file_data = std::fs::read(&path).expect("Could not read file.");
                    let binary_data = file_data.as_slice();

                    let mut cpu = CPU::new();
                    cpu.load_elf(binary_data);

                    loop {
                        cpu.execute_ins();
                        if cpu.is_exited() {
                            break;
                        }
                    }

                    println!("CPU finished executing test: {:?}", path);
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
