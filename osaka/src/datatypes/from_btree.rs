/// notes
/// This macro exists because it was doning more thing before
/// i dont really need it anymore, but refactoring the whole thing is a time consuming task so i will just leave it here

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
