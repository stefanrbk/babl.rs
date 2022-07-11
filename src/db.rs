use std::{collections::HashMap, sync::Mutex, any::Any};

use crate::{instance::{BablList, BablEachFunction}, Babl};

#[macro_export]
macro_rules! needs_db {
    () => {
        static DB: Lazy<RwLock<BablDb>> = Lazy::new(|| RwLock::new(BablDb::new()));
    };
}
pub struct BablDb {
    pub name_hash: HashMap<String, usize>,
    pub id_hash: HashMap<i32, usize>,
    pub babl_list: BablList,
}

impl BablDb {
    pub fn new() -> Self {
        Self {
            name_hash: HashMap::new(),
            id_hash: HashMap::new(),
            babl_list: BablList::with_capacity(512),
        }
    }
    pub fn find(&self, name: impl Into<String>) -> Option<usize> {
        if let Some(index) = self.name_hash.get(&name.into()) {
            Some(*index)
        } else {
            None
        }
    }
    pub fn count(&self) -> usize {
        self.babl_list.len()
    }
    pub fn insert(&mut self, item: Mutex<Babl>) -> usize {
        let babl = item.lock().unwrap();
        unsafe {
            let len = self.babl_list.len();
            if babl.instance.id != 0 {
                self.id_hash.insert(babl.instance.id, len);
            }
            self.name_hash.insert(babl.instance.name.clone(), len);
            drop(babl);
            self.babl_list.push(item);
            len
        }
    }
    pub fn each(&mut self, func: BablEachFunction, user_data: &mut Box<dyn Any>) {
        for item in self.babl_list.iter_mut() {
            func(item.lock().as_mut().unwrap(), user_data);
        }
    }
    pub fn exist(&self, id: i32, name: impl Into<String>) -> Option<usize> {
        if id != 0 {
            Some(*self.id_hash.get(&id)?)
        } else {
            Some(*self.name_hash.get(&name.into())?)
        }
    }
    pub fn exist_by_id(&self, id: i32) -> Option<usize> {
        Some(*self.id_hash.get(&id)?)
    }
    pub fn exist_by_name(&self, name: impl Into<String>) -> Option<usize> {
        Some(*self.name_hash.get(&name.into())?)
    }
}
