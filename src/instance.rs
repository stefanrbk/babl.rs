use std::{sync::{Mutex, atomic::AtomicU32}, any::Any};

use crate::Babl;

pub type BablClassType = i32;
pub type BablList = Vec<Mutex<Babl>>;

pub type BablEachFunction = fn(&mut Babl, &mut Box<dyn Any>);

pub static CURRENT_EXTENDER: AtomicU32 = AtomicU32::new(0);

#[repr(C)]
pub struct BablInstance {
    pub class_type: BablClassType,
    pub id: i32,
    pub creator: Option<usize>,
    pub name: String,
    _doc: String
}

impl BablInstance {
    /// Creates a new [`BablInstance`].
    pub fn new(class_type: BablClassType, id: i32, creator: Option<usize>, name: impl Into<String>, doc: impl Into<String>) -> Self {
        Self { class_type, id, creator, name: name.into(), _doc: doc.into() }
    }

    fn doc(&self) -> &String {
        &self._doc
    }
}
