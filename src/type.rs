use crate::instance::{BablInstance, BablList};

#[repr(C)]
pub struct BablType {
    pub instance: BablInstance,
    pub from_list: BablList,
    pub bits: i32,
    pub min_val: f64,
    pub max_val: f64,
}
#[repr(C)]
pub struct BablTypeInteger {
    pub r#type: BablType,
    pub is_signed: bool,
    pub max: i64,
    pub min: i64,
}
#[repr(C)]
pub struct BablTypeFloat {
    pub r#type: BablType,
}
