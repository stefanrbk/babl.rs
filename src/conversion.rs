use std::any::Any;

use crate::{Babl, BablFunc};

pub(crate) struct BablConversion {
    pub source: Option<usize>,
    pub destination: Option<usize>,
    pub dispatch: BablConversionDispatch,
    pub data: Box<dyn Any>,
    pub cost: i64,
    pub error: f64,
    pub function: BablFunc,
    pub pixels: i64,
}

pub(crate) type BablConversionDispatch = fn(
    babl: &Babl,
    src: &Box<dyn Any>,
    dst: &mut Box<dyn Any>,
    n: usize,
    user_data: &mut Box<dyn Any>,
) -> Box<dyn Any>;
