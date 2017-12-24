use {Reg, Value};

#[derive(Copy,Clone,PartialEq,Eq,Debug)]
pub enum Op {
    Set(Reg, Value),
    Sub(Reg, Value),
    Mul(Reg, Value),
    Jnz(Value, Value),
}

impl Op {
    pub fn from_str(op: &str) -> Result<Op, String> {
        let ref mut it = op.split_whitespace();
        let eof_err = || format!("Unexpected EOF while parsing op: {:?}", op);
        let value = |it: &mut Iterator<Item=&str>| Value::from_str(it.next().ok_or_else(&eof_err)?);
        let reg = |it: &mut Iterator<Item=&str>| Reg::from_str(it.next().ok_or_else(&eof_err)?);
        let name = it.next().ok_or_else(&eof_err)?;

        Ok(match name {
            "set" => Op::Set(reg(it)?, value(it)?),
            "sub" => Op::Sub(reg(it)?, value(it)?),
            "mul" => Op::Mul(reg(it)?, value(it)?),
            "jnz" => Op::Jnz(value(it)?, value(it)?),
            _ => return Err(format!("Invalid op: {:?}", name)),
        })
    }
}
