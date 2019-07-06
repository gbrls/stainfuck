#[derive(Debug, Copy, Clone)]
enum Instr {
    MvPtrRight,
    MvPtrLeft,
    Add,
    Sub,
    Print,
    Replace,
    Open,
    Close,

    Unknwon,
}

fn instr(c: char) -> Instr {
    use Instr::*;

    match c {
        '>' => MvPtrRight,
        '<' => MvPtrLeft,
        '+' => Add,
        '-' => Sub,
        '.' => Print,
        ',' => Replace,
        '[' => Open,
        ']' => Close,

        _ => Unknwon,
    }
}

fn print(data: u8) {
    print!("{}", data as char);
}

struct program {
    pc: usize,
    pointer: usize,
    tape: Vec<u8>,

    file: Vec<Instr>,
}

impl program {
    fn new(tape_size: usize, file_data: String) -> program {
        let mut p = program {
            pc: 0,
            pointer: 0,
            tape: vec![0; tape_size],
            file: Vec::new(),
        };

        for i in file_data.chars() {
            p.file.push(instr(i));
        }

        p
    }

    fn run_intr(&mut self, instr: Instr) {
        use Instr::*;
        match instr {
            MvPtrLeft => {
                if self.pointer >= 1 {
                    self.pointer -= 1;
                } else {
                    self.pointer = self.tape.len() - 1;
                }
            }

            MvPtrRight => {
                if self.pointer < self.tape.len() - 1 {
                    self.pointer += 1;
                } else {
                    self.pointer = 0;
                }
            }

            Add => {
                let (a, _) = self.tape[self.pointer].overflowing_add(1);
                self.tape[self.pointer] = a;
            }

            Sub => {
                let (a, _) = self.tape[self.pointer].overflowing_sub(1);
                self.tape[self.pointer] = a;
            }

            Open => {
                self.jump_if_zero();
            }

            Close => {
                self.jump_if_not_zero();
            }

            Print => {
                print(self.tape[self.pointer]);
            }
            _ => {}
        }
    }

    fn start(&mut self) {
        while self.pc < self.file.len() {
            //self.print_debug();

            let instr = self.file[self.pc];
            self.pc += 1;

            self.run_intr(instr);
        }
    }

    fn jump_if_zero(&mut self) {
        if self.tape[self.pointer] != 0 {
            return;
        }

        let mut depth = 0;

        while self.pc < self.file.len() {
            match self.file[self.pc] {
                Instr::Open => {
                    depth += 1;
                }

                Instr::Close => {
                    if depth == 0 {
                        break;
                    }

                    depth -= 1;
                }
                _ => {}
            }

            self.pc += 1;
        }
    }

    fn jump_if_not_zero(&mut self) {
        if self.tape[self.pointer] == 0 {
            return;
        }

        let mut depth = 0;
        self.pc -= 2;

        while self.pc < self.file.len() {
            match self.file[self.pc] {
                Instr::Close => {
                    depth += 1;
                }

                Instr::Open => {
                    if depth == 0 {
                        break;
                    }

                    depth -= 1;
                }
                _ => {}
            }

            self.pc -= 1;
        }
    }

    fn print_debug(&self) {
        println!(
            "PC: {}, Instr: {:?}, {:?}",
            self.pc, self.file[self.pc], self.tape
        );
    }
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err("Not enough arguments");
    }

    let raw_contents =
        std::fs::read_to_string(std::path::Path::new(&args[1])).expect("Couldn't open file");

    let mut prog = program::new(2 << 16, raw_contents);
    prog.start();

    Ok(())
}
