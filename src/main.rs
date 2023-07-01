use clap::Parser;
use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::{BitAnd, Index};
use std::vec;

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Args {
    /// Name of the person to greet
    #[arg(short = 'f')]
    input_path: String,
    //output path
    #[arg(short = 'o')]
    output_path: Option<String>,
}

fn read_file(file_path: String) -> std::io::Result<Vec<u8>> {
    let f: File = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    return Ok(buffer);
}

#[derive(Clone, Copy, Debug)]
enum Opcode {
    Mov,
}
enum Direction {
    ToRegister = 0,
    FromRegister = 1,
}

enum OperationSize {
    Word,
    Byte,
}

enum CommandMode {
    MemoryMode,
    MemoryMode8BitDisp,
    MemoryMode16BitDisp,
    RegisterMode,
}

struct Command {
    operation: Opcode,
    direction: Direction,
    operation_size: OperationSize,
    command_mode: CommandMode,
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcode::Mov => write!(f, "mov"),
        }
    }
}

fn get_command<'a, I>(vals: &mut I) -> Option<Command>
where
    I: Iterator<Item = &'a u8>,
{
    let tmp = vals.next().to_owned();
    // tmp.copied()
    None
}

// fn get_command(iter: dyn Iterator<Item = u8>) -> Option<Command> {
//     return None;
// }

fn main() {
    let mut opcodes: [Option<Opcode>; 64] = [None; 64];

    opcodes[0b100010] = Some(Opcode::Mov);

    let args: Args = Args::parse();
    let output_path: String = match args.output_path {
        Some(ref path) => path.to_owned(),
        None => args.input_path.to_owned() + ".out",
    };

    println!("input: {:?}", args);

    let data: Vec<u8> = read_file(args.input_path).unwrap();
    let mut commands: Vec<Command> = Vec::new();

    let mut iter = data.iter();
    loop {
        let command: Option<Command> = get_command(&mut iter);
        if command.is_none() {
            break;
        } else {
            commands.push(command.unwrap());
        }
    }
    // while index != data.len() {
    //     let first_byte: u8 = data[index];

    //     let op_code_index = first_byte >> 2;

    //     let opcode = opcodes.index(op_code_index as usize).unwrap();
    //     let dir = (first_byte & 0b10) >> 1;
    //     let command_dir: Direction = match dir {
    //         0 => Direction::FromRegister,
    //         1 => Direction::ToRegister,
    //         _ => panic!("unexpected case"),
    //     };
    //     index += 1;
    // }
    // loop {
    //     let b = iter.next();
    //     if b.is_none() {
    //         break;
    //     }
    //     first_byte=b.
    // }

    println!("read: {:?} ", data);

    println!("Output path: {:?}", output_path);
}
