use std::fs::File;
use std::io::Read;
use std::path::Path;

use machine::{InstructionManager, Memset, Jump, Halt, Continue};
use Args;


const MEMORY_SIZE: usize = 256;


pub fn main(args: Args) {
    // Read binary file
    let path = Path::new(&args.arg_input);
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(err) => { panic!("Can't open {}: {}", path.display(), err) }
    };

    let mut source = vec![];
    match file.read_to_end(&mut source) {
        Ok(v)  => v,
        Err(err) => { panic!("Can't read {}: {}", path.display(), err) }
    };

    // Run virtual machine
    run(&source);
}

fn run(source: &[u8]) {
    let mut memory = [0u8; MEMORY_SIZE];
    let mut pc = 0;
    let im = InstructionManager::new();

    loop {
        debug!("");
        debug!("source: {:?}@{}", source, source.len());
        debug!("memory: {:?}@{}", &memory[..], memory.len());
        debug!("pc: {}", pc);

        // Read & decode opcode
        let opcode = source[pc]; debug!("opcode: {:#04X}", opcode);
        let ref instruction = im.decode_opcode(opcode);

        // Increment programm counter (skip opcode)
        pc += 1;

        // Read arguments
        let argc = instruction.argc; debug!("argc: {}", argc);
        if pc + argc >= source.len() { panic!("Reached end of input without HALT!") }

        let argv: &[u8] = if argc == 0 { &[] }
                          else { &source[pc .. pc + argc] };
        debug!("argv: {:?}", argv);

        // Increment programm counter (skip args)
        pc += argc;

        // Execute instruction
        match instruction.execute(argv, &memory) {
            Continue => {},
            Jump { address } => {
                debug!("Jumping to {}", address);
                pc = address as usize;
            },
            Memset { address, value } => {
                debug!("Setting m[{}] = {}", address, value);
                memory[address as usize] = value;
            },
            Halt => break
        }
    }
}