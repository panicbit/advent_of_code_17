use super::Reg;

#[derive(Copy,Clone,PartialEq,Eq,Debug)]
pub enum Value {
    Imm(isize),
    Reg(Reg),
}

impl Value {
    pub fn from_str(value: &str) -> Result<Self, String> {
        value.parse().map(Value::Imm)
        .or_else(|_| Reg::from_str(value).map(Value::Reg))
        .map_err(|_| format!("{:?} is neither a register nor an int", value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn immediate_value() {
        assert_eq!(
            Value::from_str("-1234"),
            Ok(Value::Imm(-1234))
        );
    }

    #[test]
    fn register_value() {
        let reg = Reg::from_char('r').unwrap();
        assert_eq!(
            Value::from_str("r"),
            Ok(Value::Reg(reg))
        );
    }

}
