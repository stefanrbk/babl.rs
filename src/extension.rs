use std::sync::{Mutex, RwLock};

use libloading::Library;
use once_cell::sync::Lazy;

use crate::{instance::BablInstance, Babl, db::BablDb, needs_db};

needs_db!();

pub static CURRENT_EXTENDER: Lazy<Mutex<Option<Mutex<Babl>>>> = Lazy::new(|| {
    Mutex::new(None)
});

#[repr(C)]
pub struct BablExtender {
    instance: BablInstance,
    lib: Library,
}

impl BablExtender {
}
