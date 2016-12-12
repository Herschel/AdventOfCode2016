use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug,PartialEq,Clone,Copy)]
enum Register {
    A,
    B,
    C,
    D,
}


#[derive(Debug,PartialEq,Clone)]
enum Op {
    Copy { src: Operand, dst: Register },
    Inc(Register),
    Dec(Register),
    Jnz { src: Operand, offset: isize },
}

#[derive(Debug,PartialEq,Clone)]
enum Operand {
    Register(Register),
    Immediate(i32),
}

impl FromStr for Register {
    type Err = String;
    fn from_str(s: &str) -> Result<Register, String> {
        let reg = match s {
            "a" => Register::A,
            "b" => Register::B,
            "c" => Register::C,
            "d" => Register::D,
            _ => return Err("Invalid register".to_string()),
        };
        Ok(reg)
    }
}

impl FromStr for Operand {
    type Err = String;
    fn from_str(s: &str) -> Result<Operand, String> {
        let src = if let Ok(r) = Register::from_str(s) {
            Operand::Register(r)
        } else {
            Operand::Immediate(s.parse().map_err(|_| "Invalid operand".to_string())?)
        };
        Ok(src)
    }
}

impl FromStr for Op {
    type Err = String;
    fn from_str(s: &str) -> Result<Op, String> {
        let parts = s.split(" ").collect::<Vec<_>>();
        let op = match parts[0] {
            "cpy" => Op::Copy {
                src: Operand::from_str(parts[1])?,
                dst: Register::from_str(parts[2])?,
            },
            "inc" => Op::Inc(Register::from_str(parts[1])?),
            "dec" => Op::Dec(Register::from_str(parts[1])?),
            "jnz" => Op::Jnz {
                src: Operand::from_str(parts[1])?,
                offset: parts[2].parse().map_err(|_| "Invalid operand".to_string())?,
            },
            _ => return Err("Invalid operation".to_string()),
        };
        Ok(op)
    }
}

#[derive(Debug,PartialEq,Clone)]
struct Vm {
    program: Vec<Op>,
    registers: [i32; 4],
    pc: usize,
}

impl Vm {
    fn new(program: Vec<Op>, initial_registers: [i32; 4]) -> Vm {
        Vm {
            program: program,
            registers: initial_registers,
            pc: 0,
        }
    }

    fn tick(&mut self) -> bool {
        let op = &self.program[self.pc];
        self.pc += 1;
        match op {
            &Op::Copy { ref src, dst } => {
                let val = self.get_operand(&src);
                self.registers[dst as usize] = val;
            },
            &Op::Inc(r) => self.registers[r as usize] += 1,
            &Op::Dec(r) => self.registers[r as usize] -= 1,
            &Op::Jnz { ref src, offset } => {
                let val = self.get_operand(&src);
                if val != 0 {
                    self.pc = (self.pc as isize + (offset - 1)) as usize;
                }
            },
        }

        self.pc != self.program.len()
    }

    fn get_operand(&self, operand: &Operand) -> i32 {
        match operand {
            &Operand::Register(r) => self.registers[r as usize],
            &Operand::Immediate(n) => n,
        }
    }
}

fn main() {
    // Run with 0 0 1 0 to initialize C register to 1
    let mut initial_registers = [0; 4];
    for (i, arg) in std::env::args().skip(1).take(4).enumerate() {
        initial_registers[i] = arg.parse().unwrap_or(0);
    }

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Invalid input");

    let program: Vec<Op> = input.lines().map(|l| Op::from_str(l).unwrap()).collect();
    let mut vm = Vm::new(program, initial_registers);

    while vm.tick() {
    }

    println!("Register A: {}", vm.registers[0]);
}