use std::{
    ffi::OsString,
    sync::{Mutex, RwLock},
};

use libloading::{Library, Symbol};
use once_cell::sync::Lazy;

use crate::{babl_log, db::BablDb, needs_db, Babl};

needs_db!();

pub static CURRENT_EXTENDER: Lazy<Mutex<Option<usize>>> = Lazy::new(|| Mutex::new(None));

#[repr(C)]
pub struct BablExtender {
    lib: Library,
    disposed: Mutex<bool>,
}

impl BablExtender {
    pub fn new(path: impl Into<String>, lib: Library) -> usize {
        {
            let babl = Babl::new_extension(
                0,
                path.into(),
                "",
                BablExtender {
                    lib,
                    disposed: Mutex::new(false),
                },
            );
            DB.write().unwrap().insert(Mutex::new(babl))
        }
    }
    pub fn get_current() -> Option<usize> {
        *CURRENT_EXTENDER.lock().unwrap()
    }
    pub fn set(new_extender: Option<usize>) {
        *CURRENT_EXTENDER.lock().unwrap() = new_extender;
    }
    pub fn destroy() {}
    fn load_failed() {
        Self::set(None);
    }
    pub fn load(path: impl Into<String>) -> Option<usize> {
        let path: String = path.into();
        let os_path: OsString = OsString::from(path.clone());
        unsafe {
            let lib = match Library::new(os_path) {
                Ok(result) => result,
                Err(e) => {
                    babl_log!("{}", e);
                    Self::load_failed();
                    return None;
                }
            };
            let idx = Self::new(path.clone(), lib);

            Self::set(Some(idx));

            let mut db = DB.write().unwrap();
            let ext = db
                .get(Self::get_current().unwrap())
                .unwrap()
                .lock()
                .unwrap();

            let init: Symbol<unsafe extern "C" fn() -> i32> = match ext.unwrap_extender().lib.get(b"init\0")
            {
                Ok(result) => result,
                Err(e) => {
                    babl_log!("{}", e);
                    Self::load_failed();
                    return None;
                }
            };

            if init() != 0 {
                babl_log!("init() in extension '{}' failed (return!=0)", path.clone());
                drop(ext);
                db.remove(idx);
                Self::load_failed();
                return None;
            }
        }

        None
    }
}

impl Drop for BablExtender {
    fn drop(&mut self) {
        unsafe {
            let disposed = self.disposed.lock();
            let mut disposed = disposed.unwrap();
            if !disposed.to_owned() {
                let destroy: Result<Symbol<unsafe extern "C" fn()>, _> = self.lib.get(b"destroy\0");
                if let Ok(destroy) = destroy {
                    destroy();
                }
                *disposed = true;
            }
        }
    }
}
