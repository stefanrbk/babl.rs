pub(crate) type BablPolynomialEvalFunc = fn(
    poly: &BablPolynomial,
    x: f64,
) -> f64;

pub(crate) const MIN_DEGREE: usize = 0;
pub(crate) const MAX_DEGREE: usize = 10;

pub(crate) const MIN_SCALE: usize = 1;
pub(crate) const MAX_SCALE: usize = 2;

pub(crate) struct BablPolynomial {
    pub eval: BablPolynomialEvalFunc,
    pub degree: usize,
    pub scale: usize,
    pub coeff: [f64; MAX_DEGREE + 1],
}
