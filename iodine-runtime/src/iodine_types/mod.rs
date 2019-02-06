pub mod iodine_code_object;
pub mod iodine_context;
pub mod iodine_module;
pub mod iodine_name;
pub mod iodine_string;

#[macro_export]
macro_rules! string {
    ($str: expr) => {
        $crate::iodine_types::iodine_string::create($str)
    };
}

#[macro_export]
macro_rules! name {
    ($str: expr) => {
        $crate::iodine_types::iodine_name::create($str)
    };
}

#[macro_export]
macro_rules! code_object {
    () => {
        $crate::iodine_types::iodine_code_object::create()
    };
}

#[macro_export]
macro_rules! module {
    ($name: expr, $code: expr) => {
        $crate::iodine_types::iodine_module::create($name, $code)
    };
}
