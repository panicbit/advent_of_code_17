use {Reg,Op,Value};
use std::collections::HashMap;
use std::cell::RefCell;
use std::collections::VecDeque;

pub struct Cpu {
    pc: isize,
    registers: HashMap<Reg, isize>,
    sent: VecDeque<isize>,
    received: VecDeque<isize>,
    breakpoints: Vec<Box<RefCell<FnMut(&Cpu) -> bool>>>,
    code: Vec<Op>,
}

impl Cpu {
    pub fn with_code(code: Vec<Op>, id: isize) -> Self {
        let mut registers = HashMap::new();
        registers.insert(Reg::from_char('p').unwrap(), id);

        Cpu {
            pc: 0,
            registers,
            sent: VecDeque::new(),
            received: VecDeque::new(),
            breakpoints: vec! [
                Box::new(RefCell::new(break_on_pc_oob)),
            ],
            code
        }
    }

    pub fn pc(&self) -> isize {
        self.pc
    }

    pub fn registers(&self) -> &HashMap<Reg, isize> {
        &self.registers
    }

    pub fn registers_mut(&mut self) -> &mut HashMap<Reg, isize> {
        &mut self.registers
    }

    pub fn get_reg(&self, reg: Reg) -> isize {
        *self.registers().get(&reg).unwrap_or(&0)
    }

    pub fn set_reg(&mut self, reg: Reg, value: isize) {
        self.registers_mut().insert(reg, value);
    }

    pub fn get_value(&self, value: Value) -> isize {
        match value {
            Value::Imm(value) => value,
            Value::Reg(reg) => self.get_reg(reg)
        }
    }

    pub fn set_value(&mut self, reg: Reg, value: Value) {
        let value = self.get_value(value);
        self.set_reg(reg, value);
    }

    pub fn code(&self) -> &[Op] {
        &self.code
    }

    pub fn add_breakpoint<F>(&mut self, f: F) where
        F: FnMut(&Cpu) -> bool + 'static,
    {
        self.breakpoints.push(Box::new(RefCell::new(f)));
    }

    pub fn step(&mut self) -> Action {
        if self.breakpoints.iter().any(|b| (&mut *b.borrow_mut())(self)) {
            return Action::Break;
        }

        let op = self.code()[self.pc as usize];

        match op {
            Op::Snd(freq) => {
                let value = self.get_value(freq);
                self.sent.push_back(value);
            },
            Op::Set(reg, value) => self.set_value(reg, value),
            Op::Add(reg, value) => {
                let a = self.get_reg(reg);
                let b = self.get_value(value);
                self.set_reg(reg, a + b);
            },
            Op::Mul(reg, value) => {
                let a = self.get_reg(reg);
                let b = self.get_value(value);
                self.set_reg(reg, a * b);
            },
            Op::Mod(reg, value) => {
                let a = self.get_reg(reg);
                let b = self.get_value(value);
                self.set_reg(reg, a % b);
            },
            Op::Rcv(reg) => {
                if self.received.is_empty() {
                    return Action::Break;
                }

                let value = self.received.pop_front().unwrap();
                self.set_reg(reg, value);
            },
            Op::Jgz(cond, offset) => {
                let cond = self.get_value(cond);
                let offset = self.get_value(offset);
                if cond > 0 {
                    self.pc += offset - 1;
                }
            }
        }

        self.pc += 1;

        Action::Continue
    }

    pub fn run(&mut self) {
        while self.step() == Action::Continue {
        }
    }

    pub fn received(&self) -> &VecDeque<isize> {
        &self.received
    }

    pub fn received_mut(&mut self) -> &mut VecDeque<isize> {
        &mut self.received
    }

    pub fn sent_mut(&mut self) -> &mut VecDeque<isize> {
        &mut self.sent
    }
}

fn break_on_pc_oob(cpu: &Cpu) -> bool {
    cpu.pc() < 0 && cpu.code().get(cpu.pc() as usize).is_some()
}

#[derive(Copy,Clone,PartialEq,Eq)]
pub enum Action {
    Continue,
    Break,
}
