use clap::Parser;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::{u8};

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

#[inline]
const fn get_register(reg: u8, w: u8) -> &'static str {
    if w == 0 {
        match reg {
            0b000 => "al",
            0b001 => "cl",
            0b010 => "dl",
            0b011 => "bl",
            0b100 => "ah",
            0b101 => "ch",
            0b110 => "dh",
            0b111 => "bh",
            _ => panic!("not valid input"),
        }
    } else if w == 1 {
        match reg {
            0b000 => "ax",
            0b001 => "cx",
            0b010 => "dx",
            0b011 => "bx",
            0b100 => "sp",
            0b101 => "bp",
            0b110 => "sl",
            0b111 => "di",
            _ => panic!("not valid input"),
        }
    } else {
        panic!("not valid w")
    }
}


fn get_memory_mode_no_disp(rm:u8)->&'static str {
    match rm {
        0b000 => "bx + si",
        0b001 => "bx + di",
        0b010 => "bp + si",
        0b011 => "bp + di",
        0b100 => "si",
        0b101 => "di",
        0b110 => todo!(),
        0b111 => "bx",
        _ => panic!("not valid input"),
    }
    
}
fn parse_mov_command_from_or_to_register<'a, I>(iter: &mut I, first_byte: &u8) -> String
where
    I: Iterator<Item = &'a u8>,
{
    let w: u8 = first_byte & 1;
    let d: u8 = first_byte >> 1 & 1;
    let second_byte = iter.next().unwrap();
    let mode = second_byte >> 6;
    let reg = second_byte >> 3 & 0b111;
    let rm =second_byte&0b111;
    let first_operand = get_register(reg, w);

    let mod_or_reg = second_byte & 0b111;
    let second_operand = match mode {
        0b00 => get_memory_mode_no_disp(rm),
        0b01 => todo!(),
        0b10 => todo!(),
        0b11 => get_register(mod_or_reg, w),
        _ => panic!("not expected value"),
    };

    if d == 0 {
        return "mov ".to_owned() + second_operand + " ," + first_operand;
    } else {
        return "mov ".to_owned() + first_operand + " ," + second_operand;
    }
}



fn parse_mov_immediate_to_register<'a, I>(iter: &mut I, first_byte: &u8) -> String
where
    I: Iterator<Item = &'a u8>,
{
    let w: u8 =(first_byte >>3)&1;
    let reg = first_byte & 0b111;
    let target_register  = get_register(reg, w);
    let data_low: u8= iter.next().unwrap().to_owned() as u8;
    if w==1{
        let data_high: u8= iter.next().unwrap().to_owned();
        let data: i16 = ((data_high as u16) << 8 | data_low as u16) as i16;
        return "mov ".to_owned()+target_register+ " " + &data.to_string();
    }else{
        let data: i8 = data_low as i8;
        return "mov ".to_owned() +target_register+ " " +  &data.to_string();
    }
}

fn get_command<'a, I>(iter: &mut I) -> Option<String>
where
    I: Iterator<Item = &'a u8>,
{
    let first_byte: &u8 = iter.next()?;
    // let opcode = first_byte.to_owned() >> 2;
    if first_byte>>2==0b100010{
        return Some(parse_mov_command_from_or_to_register(iter, first_byte));
    }else if first_byte>>4==0b1011 {
        return Some(parse_mov_immediate_to_register(iter, first_byte));
    }
    panic!("unknown opcode")
}

fn main() {

    let args: Args = Args::parse();
    let output_path: String = match args.output_path {
        Some(ref path) => path.to_owned(),
        None => args.input_path.to_owned() + ".out",
    };

    println!("input: {:?}", args);

    let data: Vec<u8> = read_file(args.input_path).unwrap();
    let mut commands: Vec<String> = Vec::new();

    let mut iter = data.iter();
    loop {
        let command: Option<String> = get_command(&mut iter);
        if command.is_none() {
            break;
        } else {
            commands.push(command.unwrap());
        }
    }

    println!("read: {:?} ", commands);

    println!("Output path: {:?}", output_path);
}
