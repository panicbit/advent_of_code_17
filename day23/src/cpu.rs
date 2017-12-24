use {Reg,Op,Value};
use std::collections::HashMap;
use std::cell::RefCell;

pub struct Cpu {
    pc: isize,
    registers: HashMap<Reg, isize>,
    mul_count: usize,
    breakpoints: Vec<Box<RefCell<FnMut(&Cpu) -> bool>>>,
    code: Vec<Op>,
}

impl Cpu {
    pub fn with_code(code: Vec<Op>) -> Self {
        Cpu {
            pc: 0,
            registers: HashMap::new(),
            mul_count: 0,
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
        self.pc += 1;

        match op {
            Op::Set(reg, value) => self.set_value(reg, value),
            Op::Sub(reg, value) => {
                let a = self.get_reg(reg);
                let b = self.get_value(value);
                self.set_reg(reg, a - b);
            },
            Op::Mul(reg, value) => {
                let a = self.get_reg(reg);
                let b = self.get_value(value);
                self.set_reg(reg, a * b);
                self.mul_count += 1;
            },
            Op::Jnz(cond, offset) => {
                let cond = self.get_value(cond);
                let offset = self.get_value(offset);
                if cond != 0 {
                    self.pc += offset - 1;
                }
            }
        }

        Action::Continue
    }

    pub fn run(&mut self) {
        while self.step() == Action::Continue {
            // println!("pc: {}", self.pc());
        }
    }

    pub fn mul_count(&self) -> usize {
        self.mul_count
    }
}

fn break_on_pc_oob(cpu: &Cpu) -> bool {
    cpu.pc() < 0 || cpu.code().get(cpu.pc() as usize).is_none()
}

#[derive(Copy,Clone,PartialEq,Eq)]
pub enum Action {
    Continue,
    Break,
}
