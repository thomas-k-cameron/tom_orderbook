#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum DerivativeType {
    Put,
    Call,
    Future,
}

impl TryFrom<char> for DerivativeType {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            'C' => Self::Call,
            'P' => Self::Put,
            'F' => Self::Future,
            _ => return Err(()),
        })
    }
}

impl TryFrom<&str> for DerivativeType {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, ()> {
        Ok(s.chars().next().ok_or(())?.try_into()?)
    }
}
