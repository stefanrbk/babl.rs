use std::any::Any;

use crate::db::BablList;

pub(crate) enum BablModelFlag {
    Alpha = 1<<1,
    Associated = 1<<2,
    Inverted = 1<<3,

    Linear = 1<<10,
    Nonlinear = 1<<11,
    Perceptual = 1<<12,

    Gray = 1<<20,
    Rgb = 1<<21,
    Spectral = 1<<22,
    Cie = 1<<23,
    Cmyk = 1<<24,
    Luz = 1<<25,
}

pub(crate) struct BablModel {
    pub from_list: Vec<usize>,
    pub components: usize,
    pub component: Vec<usize>,
    pub r#type: Vec<usize>,
    pub data: Box<dyn Any>,
    pub space: Vec<usize>,
    pub model: Vec<usize>,
    pub flags: BablModelFlag,
}
