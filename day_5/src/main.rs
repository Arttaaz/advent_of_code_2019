use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Read;

#[derive(Debug)]
struct Instruction {
    opcode: u8,
    op1: Option<Parameter>,
    op2: Option<Parameter>,
    dest: Option<Parameter>,
}

#[derive(Clone, Copy, Debug)]
struct Parameter {
    value: i64,
    mode: Mode,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Mode {
    Position,
    Immediate,
}

impl Instruction {
    fn new(slice: &[i64]) -> Result<Self, String> {
        let op = slice[0] % 100;
        let mut remain = slice[0] / 100;
        let mut params_modes = Vec::new();
        for _ in 0..3 {
            params_modes.push(remain % 10);
            remain /= 10;
        }

        match op {
            99 => Err("Program Halted!".to_string()),
            1 | 2 | 7 | 8 => {
                let param1 = match params_modes[0] {
                    0 => Mode::Position,
                    1 => Mode::Immediate,
                    _ => panic!("param1: {}", params_modes[0]),
                };
                let param2 = match params_modes[1] {
                    0 => Mode::Position,
                    1 => Mode::Immediate,
                    _ => panic!("param2: {}", params_modes[1]),

                };
                let param3 = match params_modes[2] {
                    0 => Mode::Position,
                    1 => Mode::Immediate,
                    _ => panic!("param3: {}", params_modes[2]),

                };
                if param3 != Mode::Position {
                    panic!()
                }
                Ok(Self {
                    opcode: op as u8,
                    op1: Some(Parameter {
                        value: slice[1] as i64,
                        mode: param1,
                    }),
                    op2: Some(Parameter {
                        value: slice[2] as i64,
                        mode: param2,
                    }),
                    dest: Some(Parameter {
                        value: slice[3] as i64,
                        mode: param3,
                    }),
                })
            },
            3 => {
                let param1 = match params_modes[0] {
                    0 => Mode::Position,
                    _ => panic!("param1: {}", params_modes[0]),
                };
                Ok(Self {
                    opcode: op as u8,
                    op1: None,
                    op2: None,
                    dest: Some(Parameter {
                        value: slice[1] as i64,
                        mode: param1,
                    }),
                })
            },
            4 => {
                let param1 = match params_modes[0] {
                    0 => Mode::Position,
                    1 => Mode::Immediate,
                    _ => panic!("param1: {}", params_modes[0]),
                };
                Ok(Self {
                    opcode: op as u8,
                    op1: Some(Parameter {
                        value: slice[1] as i64,
                        mode: param1
                    }),
                    op2: None,
                    dest: None,

                })
            },
            5 | 6 => {
                let param1 = match params_modes[0] {
                    0 => Mode::Position,
                    1 => Mode::Immediate,
                    _ => panic!("param1: {}", params_modes[0]),
                };
                let param2 = match params_modes[1] {
                    0 => Mode::Position,
                    1 => Mode::Immediate,
                    _ => panic!("param1: {}", params_modes[1]),
                };
                Ok(Self {
                    opcode: op as u8,
                    op1: Some(Parameter {
                        value: slice[1] as i64,
                        mode: param1,
                    }),
                    op2: Some(Parameter {
                        value: slice[2] as i64,
                        mode: param2,
                    }),
                    dest: None,
                })
            },
            _ => Err("Unknown Op code".to_string()),
        }
    }

    fn run(self, program: &mut Vec<i64>, pos: i64) -> i64 {
        match self.opcode {
            1 => {
                let op1 = match self.op1.unwrap().mode {
                    Mode::Immediate => self.op1.unwrap().value as i64,
                    Mode::Position => program[self.op1.unwrap().value as usize],
                };
                let op2 = match self.op2.unwrap().mode {
                    Mode::Immediate => self.op2.unwrap().value as i64,
                    Mode::Position => program[self.op2.unwrap().value as usize],
                };
                program[self.dest.unwrap().value as usize] = op1 + op2;
                pos + 4
            },
            2 => {
                let op1 = match self.op1.unwrap().mode {
                    Mode::Immediate => self.op1.unwrap().value as i64,
                    Mode::Position => program[self.op1.unwrap().value as usize],
                };
                let op2 = match self.op2.unwrap().mode {
                    Mode::Immediate => self.op2.unwrap().value as i64,
                    Mode::Position => program[self.op2.unwrap().value as usize],
                };
                program[self.dest.unwrap().value as usize] = op1 * op2;
                pos + 4
            },
            3 => {
                // program[self.dest.unwrap().value as usize] = 1; // part_1 only called once
                program[self.dest.unwrap().value as usize] = 5; // part_2 only called once
                pos + 2
            },
            4 => {
                let op1 = match self.op1.unwrap().mode {
                    Mode::Immediate => self.op1.unwrap().value as i64,
                    Mode::Position => program[self.op1.unwrap().value as usize],
                };
                println!("{}", op1);
                pos + 2
            },
            5 => {
                let op1 = match self.op1.unwrap().mode {
                    Mode::Immediate => self.op1.unwrap().value as i64,
                    Mode::Position => program[self.op1.unwrap().value as usize],
                };
                let op2 = match self.op2.unwrap().mode {
                    Mode::Immediate => self.op2.unwrap().value as i64,
                    Mode::Position => program[self.op2.unwrap().value as usize],
                };
                if op1 != 0 {
                    op2
                } else {
                    pos + 3
                }
            },
            6 => {
                let op1 = match self.op1.unwrap().mode {
                    Mode::Immediate => self.op1.unwrap().value as i64,
                    Mode::Position => program[self.op1.unwrap().value as usize],
                };
                let op2 = match self.op2.unwrap().mode {
                    Mode::Immediate => self.op2.unwrap().value as i64,
                    Mode::Position => program[self.op2.unwrap().value as usize],
                };
                if op1 == 0 {
                    op2
                } else {
                    pos + 3
                }
            },
            7 => {
                let op1 = match self.op1.unwrap().mode {
                    Mode::Immediate => self.op1.unwrap().value as i64,
                    Mode::Position => program[self.op1.unwrap().value as usize],
                };
                let op2 = match self.op2.unwrap().mode {
                    Mode::Immediate => self.op2.unwrap().value as i64,
                    Mode::Position => program[self.op2.unwrap().value as usize],
                };

                if op1 < op2 {
                    program[self.dest.unwrap().value as usize] = 1;
                } else {
                    program[self.dest.unwrap().value as usize] = 0;
                }

                pos + 4
            },
            8 => {
                let op1 = match self.op1.unwrap().mode {
                    Mode::Immediate => self.op1.unwrap().value as i64,
                    Mode::Position => program[self.op1.unwrap().value as usize],
                };
                let op2 = match self.op2.unwrap().mode {
                    Mode::Immediate => self.op2.unwrap().value as i64,
                    Mode::Position => program[self.op2.unwrap().value as usize],
                };

                if op1 == op2 {
                    program[self.dest.unwrap().value as usize] = 1;
                } else {
                    program[self.dest.unwrap().value as usize] = 0;
                }

                pos + 4

            },
            _ => unreachable!(),
        }
    }
}

struct Computer {
    program: Vec<i64>,
}

impl Computer {
    fn new(program: Vec<i64>) -> Self {
        Self {
            program,
        }
    }

    fn run(mut self) {
        let mut pos : i64 = 0;
        let r = loop {
            let inst = match Instruction::new(&self.program[pos as usize..pos as usize+4]) {
                Ok(i) => i,
                Err(s) => break format!("{}", s),
            };
            pos = inst.run(&mut self.program, pos);
        };
        println!("{}", r);
    }
}

fn main() -> () {
    let f = OpenOptions::new().read(true).open("input").unwrap();
    let mut buf = BufReader::new(f);
    let mut program : String = "".to_owned();
    match buf.read_to_string(&mut program) {
        Ok(n) => println!("read {} bytes from input file.", n),
        Err(e) => eprintln!("Error reading from input file: {:?}", e),
    }
    program.pop();
    let program = Computer::new(program.split(",").map(|x| x.parse::<i64>().expect(format!("{:?}", x).as_str())).collect());
    program.run();
}
