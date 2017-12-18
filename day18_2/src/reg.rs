
#[derive(Copy,Clone,Hash,PartialEq,Eq,Debug)]
pub struct Reg(char);

impl Reg {
    pub fn from_char(reg: char) -> Result<Self, String> {
        if reg.is_alphabetic() {
            Ok(Reg(reg))
        } else {
            Err(format!("Register must be alphabetic: {:?}", reg))
        }
    }

    pub fn from_str(reg: &str) -> Result<Self, String> {
        if reg.len() != 1 {
            return Err(format!("Register length need to be 1: {:?}", reg));
        }

        let reg = reg.chars().next().unwrap();
        Reg::from_char(reg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reg() {
        assert_eq!(Reg::from_char('a'), Ok(Reg('a')));
    }

    #[test]
    #[should_panic]
    fn invalid_reg() {
        Reg::from_char('1').unwrap();
    }
}