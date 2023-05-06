#![allow(non_snake_case)]
use std::str::FromStr;

use chrono::NaiveDateTime;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    AddOrder,
    CombinationProduct,
    DeleteOrder,
    EquilibriumPrice,
    Executed,
    ExecutionWithPriceInfo,
    LegPrice,
    ProductInfo,
    SecondTag,
    SystemEventInfo,
    TickSize,
    TradingStatusInfo,
};

macro_rules! dclr_message_enum {
    ($($ident:ident,)*) => {
        #[derive(Deserialize, Serialize, Debug, PartialEq, Eq, PartialOrd, Hash, Ord, Clone)]
        #[serde(tag = "tag")]
        pub enum MessageEnum {
            $( $ident(Box<$ident>), )*
        }
        impl MessageEnum {

            pub fn tag(&self) -> char {
                match self {
                    $( MessageEnum::$ident(_) => $ident::TAG, )*
                }
            }

            pub fn timestamp(&self) -> NaiveDateTime {
                match self {
                    $( MessageEnum::$ident(x) => x.timestamp, )*
                }
            }
        }

        $(
            impl TryFrom<MessageEnum> for $ident {
                type Error = &'static str;
                fn try_from(msg_enum: MessageEnum) -> Result<Self, Self::Error> {
                    match msg_enum {
                        MessageEnum::$ident(item) => Ok(*item),
                        _ => Err(stringify!($ident))
                    }
                }
            }
        ) *

        impl TryFrom<String> for MessageEnum {
            type Error = String;
            fn try_from(string: String) -> Result<Self, Self::Error> {
                $(
                    if let Ok(i) = $ident::try_from(string.as_str()) {
                        return Ok(MessageEnum::$ident(Box::new(i)))
                    }
                ) *
                return Err(string)
            }
        }

        impl FromStr for MessageEnum {
            type Err = String;
            fn from_str(string: &str) -> Result<Self, Self::Err> {
                $(
                    if let Ok(i) = $ident::try_from(string) {
                        return Ok(MessageEnum::$ident(Box::new(i)))
                    }
                ) *
                return Err(string.to_string())
            }
        }

    };
}

dclr_message_enum!(
    CombinationProduct,
    DeleteOrder,
    EquilibriumPrice,
    Executed,
    ExecutionWithPriceInfo,
    LegPrice,
    ProductInfo,
    AddOrder,
    SecondTag,
    SystemEventInfo,
    TickSize,
    TradingStatusInfo,
);

impl MessageEnum {
    pub fn struct_name_to_tag(struct_name: &str) -> Option<char> {
        match struct_name {
            "CombinationProduct" => 'M',
            "DeleteOrder" => 'D',
            "EquilibriumPrice" => 'Z',
            "Executed" => 'E',
            "ExecutionWithPriceInfo" => 'C',
            "LegPrice" => 'P',
            "ProductInfo" => 'R',
            "PutOrder" => 'A',
            "SecondTag" => 'T',
            "SystemEventInfo" => 'S',
            "TickSize" => 'L',
            "TradingStatusInfo" => 'O',
            _ => return None,
        }
        .into()
    }

    pub fn tag_to_struct_name(tag: char) -> Option<&'static str> {
        match tag {
            'M' => "CombinationProduct",
            'D' => "DeleteOrder",
            'Z' => "EquilibriumPrice",
            'E' => "Executed",
            'C' => "ExecutionWithPriceInfo",
            'P' => "LegPrice",
            'R' => "ProductInfo",
            'A' => "PutOrder",
            'T' => "SecondTag",
            'S' => "SystemEventInfo",
            'L' => "TickSize",
            'O' => "TradingStatusInfo",
            _ => return None,
        }
        .into()
    }
}
