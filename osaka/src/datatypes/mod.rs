#[macro_export]
macro_rules! impl_message {
    (
        name: $name:ident $char:literal;
        pub timestamp: $_:ty,
        $ ( pub $field:ident: $dt:ty, ) *
    ) => {
        use chrono::NaiveDateTime;

        impl_message!(set_tag @ $name, $char);
    };
    (set_tag @ $name:ident, $char:literal) => {
        impl $name {
            pub const TAG: char = $char;
        }
    };
}


mod unique_id;
pub use unique_id::UniqueId;

mod a;
pub use a::AddOrder;
mod c;
pub use c::ExecutionWithPriceInfo;
mod d;
pub use d::DeleteOrder;
mod e;
pub use e::Executed;
mod l;
pub use l::TickSize;
mod m;
pub use m::CombinationProduct;
mod o;
pub use o::TradingStatusInfo;
mod p;
pub use p::LegPrice;
mod r;
pub use r::ProductInfo;
mod s;
pub use s::SystemEventInfo;
mod t;
pub use t::SecondTag;
mod z;
pub use z::EquilibriumPrice;

mod message_enum;
pub use message_enum::MessageEnum;

mod financial_product;
pub use financial_product::FinancialProduct;

mod put_or_call;
pub use put_or_call::PutOrCall;

mod side;
pub use side::Side;

mod leg_side;
pub use leg_side::LegSide;

pub mod util;

/// exports chrono
pub use chrono;

#[cfg(test)]
mod test;

pub mod alias;
