use {Reg, Value};

#[derive(Copy,Clone,PartialEq,Eq,Debug)]
pub enum Op {
    Snd(Value),
    Set(Reg, Value),
    Add(Reg, Value),
    Mul(Reg, Value),
    Mod(Reg, Value),
    Rcv(Reg),
    Jgz(Value, Value),
}

impl Op {
    pub fn from_str(op: &str) -> Result<Op, String> {
        let ref mut it = op.split_whitespace();
        let eof_err = || format!("Unexpected EOF while parsing op: {:?}", op);
        let value = |it: &mut Iterator<Item=&str>| Value::from_str(it.next().ok_or_else(&eof_err)?);
        let reg = |it: &mut Iterator<Item=&str>| Reg::from_str(it.next().ok_or_else(&eof_err)?);
        let name = it.next().ok_or_else(&eof_err)?;

        Ok(match name {
            "snd" => Op::Snd(value(it)?),
            "set" => Op::Set(reg(it)?, value(it)?),
            "add" => Op::Add(reg(it)?, value(it)?),
            "mul" => Op::Mul(reg(it)?, value(it)?),
            "mod" => Op::Mod(reg(it)?, value(it)?),
            "rcv" => Op::Rcv(reg(it)?),
            "jgz" => Op::Jgz(value(it)?, value(it)?),
            _ => return Err(format!("Invalid op: {:?}", name)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value_42() -> Value {
        Value::from_str("42").unwrap()
    }

    fn value_777() -> Value {
        Value::from_str("777").unwrap()
    }

    fn reg_x() -> Reg {
        Reg::from_str("x").unwrap()
    }

    #[test]
    fn parse_snd() {
        assert_eq!(
            Op::from_str("snd 42"),
            Ok(Op::Snd(value_42()))
        );
    }

    #[test]
    fn parse_set() {
        assert_eq!(
            Op::from_str("set x 42"),
            Ok(Op::Set(reg_x(), value_42()))
        );
    }

    #[test]
    fn parse_add() {
        assert_eq!(
            Op::from_str("add x 42"),
            Ok(Op::Add(reg_x(), value_42()))
        );
    }

    #[test]
    fn parse_mul() {
        assert_eq!(
            Op::from_str("mul x 42"),
            Ok(Op::Mul(reg_x(), value_42()))
        );
    }

    #[test]
    fn parse_mod() {
        assert_eq!(
            Op::from_str("mod x 42"),
            Ok(Op::Mod(reg_x(), value_42()))
        );
    }

    #[test]
    fn parse_rcv() {
        assert_eq!(
            Op::from_str("rcv x"),
            Ok(Op::Rcv(reg_x()))
        );
    }

    #[test]
    fn parse_jgz() {
        assert_eq!(
            Op::from_str("jgz 42 777"),
            Ok(Op::Jgz(value_42(), value_777()))
        );
    }
}
