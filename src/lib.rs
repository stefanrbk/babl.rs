#![allow(dead_code)]
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

mod babl;
mod component;
mod conversion;
mod db;
mod extension;
mod fish;
mod format;
mod image;
mod model;
mod polynomial;
mod sampling;
mod trc;
mod r#type;

use std::any::Any;

pub use babl::Babl;

type BablFuncLinear = fn(
    conversion: &mut Babl,
    src: &Box<dyn Any>,
    dst: &mut Box<dyn Any>,
    n: usize,
    user_data: &mut Box<dyn Any>,
) -> Box<dyn Any>;
type BablFuncPlanar = fn(
    conversion: &mut Babl,
    src_bands: usize,
    src: &Box<dyn Any>,
    src_pitch: &[usize],
    dst_bands: usize,
    dst: &mut Box<dyn Any>,
    dst_pitch: &[usize],
    n: usize,
    user_data: &mut Box<dyn Any>,
) -> Box<dyn Any>;
type BablFuncPlane = fn(
    conversion: &mut Babl,
    src: &Box<dyn Any>,
    dst: &mut Box<dyn Any>,
    src_pitch: usize,
    dst_pitch: usize,
    n: usize,
    user_data: &mut Box<dyn Any>,
) -> Box<dyn Any>;
enum BablFunc {
    Linear(BablFuncLinear),
    Planar(BablFuncPlanar),
    Plane(BablFuncPlane),
}

impl BablFunc {
    /// Returns `true` if the babl func is [`Linear`].
    ///
    /// [`Linear`]: BablFunc::Linear
    #[must_use]
    fn is_linear(&self) -> bool {
        matches!(self, Self::Linear(..))
    }

    fn as_linear(&self) -> Option<&BablFuncLinear> {
        if let Self::Linear(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the babl func is [`Planar`].
    ///
    /// [`Planar`]: BablFunc::Planar
    #[must_use]
    fn is_planar(&self) -> bool {
        matches!(self, Self::Planar(..))
    }

    fn as_planar(&self) -> Option<&BablFuncPlanar> {
        if let Self::Planar(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the babl func is [`Plane`].
    ///
    /// [`Plane`]: BablFunc::Plane
    #[must_use]
    fn is_plane(&self) -> bool {
        matches!(self, Self::Plane(..))
    }

    fn as_plane(&self) -> Option<&BablFuncPlane> {
        if let Self::Plane(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

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
