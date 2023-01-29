#[macro_export]
macro_rules! generateColumnType {
    ($name: ident, $type: literal, $pkg: expr) => {
        pub const $name: DbColumnType = DbColumnType($type, $pkg);
    };
}
