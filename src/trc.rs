use crate::{Babl, polynomial::BablPolynomial};

pub(crate) enum BablTRCType {
    Linear,
    FormulaGamma,
    Srgb,
    FormulaSrgb,
    Lut,
    FormulaCie,
}
pub(crate) type LinearFn = fn(trc: &Babl, val: f32) -> f32;
pub(crate) type BufferedLinearFn = fn(
    trc: &Babl,
    r#in: &[f32],
    out: &mut [f32],
    in_gap: usize,
    out_gap: usize,
    components: usize,
    count: usize,
);

pub(crate) struct BablTRC {
    pub r#type: BablTRCType,
    pub lut_size: usize,
    pub gamma: f64,
    pub rgamma: f32,
    pub fn_to_linear: LinearFn,
    pub fn_from_linear: LinearFn,
    pub fn_to_linear_buf: BufferedLinearFn,
    pub fn_from_linear_buf: BufferedLinearFn,
    pub poly_gamma_to_linear: BablPolynomial,
    pub poly_gamma_to_linear_x0: f32,
    pub poly_gamma_to_linear_x1: f32,
    pub poly_gamma_from_linear: BablPolynomial,
    pub poly_gamma_from_linear_x0: f32,
    pub poly_gamma_from_linear_x1: f32,
    pub lut: Box<[f32]>,
    pub inv_lut: Box<[f32]>,
    pub name: String,
}
