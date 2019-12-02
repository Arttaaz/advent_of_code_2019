use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Read;

struct Instruction {
    opcode: u8,
    op1: usize,
    op2: usize,
    dest: usize,
}

impl Instruction {
    fn new(slice: &[u64]) -> Result<Self, String> {
        match slice[0] {
            99 => Err("Program Halted!".to_string()),
            1 | 2 => {
                Ok(Self {
                    opcode: slice[0] as u8,
                    op1: slice[1] as usize,
                    op2: slice[2] as usize,
                    dest: slice[3] as usize,
                })
            },
            _ => Err("Unknown Op code".to_string()),
        }
    }

    fn run(self, program: &mut Vec<u64>) {
        let op1 = program[self.op1];
        let op2 = program[self.op2];
        match self.opcode {
            1 => {
                program[self.dest] = op1 + op2;
            },
            2 => {
                program[self.dest] = op1 * op2;
            },
            _ => unreachable!(),
        }
    }
}

struct Computer {
    program: Vec<u64>,
}

impl Computer {
    fn new(program: Vec<u64>) -> Self {
        Self {
            program,
        }
    }

    fn run(mut self, noun: u8, verb: u8) -> u64 {
        let mut pos : usize = 0;
        self.program[1] = noun as u64;
        self.program[2] = verb as u64;
        let r = loop {
            match Instruction::new(&self.program[pos..pos+4]) {
                Ok(i) => i,
                Err(s) => break format!("{}", s),
            }.run(&mut self.program);
            pos += 4;
        };
        println!(
            "noun: {}, verb: {} \n\
            exit reason: {} \n\
            result: {}"
        , noun, verb, r, self.program[0]);
        self.program[0]
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
    let program : Vec<u64> = program.split(",").map(|x| x.parse().unwrap()).collect();
    for i in 0..100 {
        for j in 0..100 {
            match Computer::new(program.clone()).run(i, j) {
                19690720 => {
                    println!("RESULT: {}", 100u64*i as u64 + j as u64);
                    return ()
                },
                _ => (),
            }
        }
    }
}
