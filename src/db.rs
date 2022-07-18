use std::{any::Any, collections::HashMap, sync::Mutex};

use crate::{extension::BablExtender, Babl};

pub type BablEachFunction = fn(&mut Babl, &mut Box<dyn Any>);
pub type BablList = Vec<Mutex<Babl>>;

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
        let mut babl = item.lock().unwrap();

        let len = self.babl_list.len();
        if babl.id != 0 {
            self.id_hash.insert(babl.id, len);
        }
        self.name_hash.insert(babl.name.clone(), len);

        babl.creator = BablExtender::get_current();

        drop(babl);
        self.babl_list.push(item);
        len
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
    pub fn get(&self, idx: usize) -> Option<&Mutex<Babl>> {
        self.babl_list.get(idx)
    }
    pub fn remove(&mut self, idx: usize) {
        let babl = self.babl_list.remove(idx);
        let babl = babl.lock().unwrap();
        let id = babl.id;
        if self.id_hash.get(&id).is_some() {
            self.id_hash.remove(&id);
        }
        let name = &babl.name;
        if self.name_hash.get(name).is_some() {
            self.name_hash.remove(name);
        }
    }
}
