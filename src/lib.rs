
const BABL_MAGIC: i32 = 0xbab100;
const BABL_INSTANCE: i32 = BABL_MAGIC;
const BABL_TYPE: i32 = BABL_INSTANCE + 1;
const BABL_TYPE_INTEGER: i32 = BABL_TYPE + 1;
const BABL_TYPE_FLOAT: i32 = BABL_TYPE_INTEGER + 1;
const BABL_SAMPLING: i32 = BABL_TYPE_FLOAT + 1;
const BABL_TRC: i32 = BABL_SAMPLING + 1;
const BABL_COMPONENT: i32 = BABL_TRC + 1;
const BABL_MODEL: i32 = BABL_COMPONENT + 1;
const BABL_FORMAT: i32 = BABL_MODEL + 1;
const BABL_SPACE: i32 = BABL_FORMAT + 1;
const BABL_CONVERSION: i32 = BABL_SPACE + 1;
const BABL_CONVERSION_LINEAR: i32 = BABL_CONVERSION + 1;
const BABL_CONVERSION_PLANE: i32 = BABL_CONVERSION_LINEAR + 1;
const BABL_CONVERSION_PLANAR: i32 = BABL_CONVERSION_PLANE + 1;
const BABL_FISH: i32 = BABL_CONVERSION_PLANAR + 1;
const BABL_FISH_REFERENCE: i32 = BABL_FISH + 1;
const BABL_FISH_SIMPLE: i32 = BABL_FISH_REFERENCE + 1;
const BABL_FISH_PATH: i32 = BABL_FISH_SIMPLE + 1;
const BABL_IMAGE: i32 = BABL_FISH_PATH + 1;
const BABL_EXTENSION: i32 = BABL_IMAGE + 1;
const BABL_SKY: i32 = BABL_EXTENSION + 1;

mod db;
mod extension;
mod r#type;
mod babl;

pub use babl::Babl;

pub fn babl_extension(name: impl Into<String>) {}

#[macro_export]
macro_rules! babl_log {
    ($($arg:tt)*) => {
        let line = line!();
        let file = file!();

        eprint!("{}:{}\n\t", line, file);
        eprintln!($($arg)*);
    };
}

#[macro_export]
macro_rules! babl_fatal {
    ($($arg:tt)*) => {
        let line = line!();
        let file = file!();

        eprint!("{}:{}\n\t", line, file);
        eprintln!($($arg)*);

        panic!();
    };
}
