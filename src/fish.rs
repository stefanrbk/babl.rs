use std::any::Any;

use crate::Babl;

pub(crate) type BablFishDispatch = fn(
    babl: &Babl,
    src: &Box<dyn Any>,
    dst: &mut Box<dyn Any>,
    n: usize,
    data: &mut Box<dyn Any>,
);

pub(crate) struct BablFishSimple {
    pub conversion: usize,
    pub cost: f64,
    pub source_bpp: usize,
    pub dest_bpp: usize,
    pub foo: Option<Box<dyn Any>>,
}

pub(crate) struct BablFishPath {
    pub cost: f64,
    pub source_bpp: usize,
    pub dest_bpp: usize,
    pub conversion_list: Vec<usize>,
}

pub(crate) enum BablFishVariant {
    Simple(BablFishSimple),
    Path(BablFishPath),
    Reference,
    None,
}

pub(crate) struct BablFish {
    pub source: usize,
    pub destination: usize,
    pub dispatch: BablFishDispatch,
    pub data: Box<dyn Any>,
    pub pixels: usize,
    pub error: f64,
}
