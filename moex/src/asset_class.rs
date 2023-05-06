use crate::derivative_type::DerivativeType;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum AssetClass {
    #[default]
    EquityOrFX,
    Derivative(DerivativeType),
}

impl AssetClass {
    pub fn new(derivative_type: impl TryInto<DerivativeType>) -> AssetClass {
        if let Ok(derivative_type) = derivative_type.try_into() {
            Self::Derivative(derivative_type)
        } else {
            Self::EquityOrFX
        }
    }
}
