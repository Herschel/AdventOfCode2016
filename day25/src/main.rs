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
    Copy { src: Operand, dst: Operand },
    Inc(Register),
    Dec(Register),
    Jnz { src: Operand, offset: Operand },
    //Tgl(Register),
    Out(Operand),
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
                dst: Operand::from_str(parts[2])?,
            },
            "inc" => Op::Inc(Register::from_str(parts[1])?),
            "dec" => Op::Dec(Register::from_str(parts[1])?),
            "jnz" => Op::Jnz {
                src: Operand::from_str(parts[1])?,
                offset: Operand::from_str(parts[2])?,
            },
            //"tgl" => Op::Tgl(Register::from_str(parts[1])?),
            "out" => Op::Out(Operand::from_str(parts[1])?),
            _ => return Err("Invalid operation".to_string()),
        };
        Ok(op)
    }
}

#[derive(Debug,PartialEq,Clone)]
struct Vm<'a> {
    program: &'a Vec<Op>,
    registers: [i32; 4],
    pc: usize,
    out: Vec<i32>,
}

impl<'a> Vm<'a> {
    fn new(program: &'a Vec<Op>, initial_registers: [i32; 4]) -> Vm<'a> {
        Vm {
            program: program,
            registers: initial_registers,
            pc: 0,
            out: vec![],
        }
    }

    fn tick(&mut self) -> bool {
        let op = self.program[self.pc].clone();
        self.pc += 1;

        match op {
            Op::Copy { ref src, ref dst } => {
                if let &Operand::Register(r) = dst {
                    let val = self.get_operand(&src);
                    self.registers[r as usize] = val;
                }
            },
            Op::Inc(r) => self.registers[r as usize] += 1,
            Op::Dec(r) => self.registers[r as usize] -= 1,
            Op::Jnz { ref src, ref offset } => {
                let val = self.get_operand(src);
                let offset_val = self.get_operand(offset);
                if val != 0 {
                    self.pc = (self.pc as i32 + (offset_val - 1)) as usize;
                }
            },
            /*Op::Tgl(r) => {
                let offset = self.registers[r as usize] as isize;
                let i = (self.pc as isize + (offset - 1)) as usize;
                if let Some(op_to_modify) = self.program.get_mut(i) {
                    *op_to_modify = match op_to_modify {
                        &mut Op::Inc(r) => Op::Dec(r),
                        &mut Op::Dec(r) => Op::Inc(r),
                        &mut Op::Jnz { ref src, ref offset } => Op::Copy { src: src.clone(), dst: offset.clone() },
                        &mut Op::Copy { ref src, ref dst } => Op::Jnz { src: src.clone(), offset: dst.clone() },
                        &mut Op::Tgl(r) => Op::Inc(r),
                        _ => panic!("Unhandled instruction for tgl"),
                    };
                }

            },*/
            Op::Out(ref src) => {
                let val = self.get_operand(src);
                self.out.push(val);
            }
        }

        self.pc < self.program.len()
    }

    fn get_operand(&self, operand: &Operand) -> i32 {
        match operand {
            &Operand::Register(r) => self.registers[r as usize],
            &Operand::Immediate(n) => n,
        }
    }
}

fn main() {
    let mut program_src = String::new();
    io::stdin().read_to_string(&mut program_src).expect("Invalid input");

    let program: Vec<Op> = program_src.lines().map(|l| Op::from_str(l).unwrap()).collect();
    
    let a = (0..).into_iter()
        .filter(|&i| {
            let mut initial_registers = [0; 4];
            initial_registers[0] = i;
            let mut vm = Vm::new(&program, initial_registers);
            while is_output_cycle(&vm.out) && vm.out.len() < 1024 && vm.tick() {
            }
            
            is_output_cycle(&vm.out)
        })
        .next()
        .unwrap();

    println!("Register A: {}", a);
}

fn is_output_cycle(out: &Vec<i32>) -> bool {
    let mut val = 0;
    for i in 0..out.len() {

        if out[i] != val {
            return false;
        }

        val = 1 - val;
    }

    true
}