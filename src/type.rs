use std::sync::{RwLock, Mutex};

use once_cell::sync::Lazy;

use crate::{
    db::{BablDb, BablList},
    Babl, needs_db,
};

needs_db!();

#[repr(C)]
pub struct BablType {
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

pub struct BablTypeBuilder {
    pub name: String,
    pub bits: i32,
    pub id: i32,
    pub min_val: Option<f64>,
    pub max_val: Option<f64>,
    pub is_signed: Option<bool>,
    pub max: Option<i64>,
    pub min: Option<i64>,
    pub integer: bool,
    pub doc: Option<String>,
}

impl BablType {
    pub const fn is_type_duplicate(&self, bits: i32) -> bool {
        self.bits == bits
    }
    pub fn new(name: impl Into<String>) -> BablTypeBuilder {
        BablTypeBuilder {
            name: name.into(),
            bits: 0,
            id: 0,
            min_val: None,
            max_val: None,
            min: None,
            max: None,
            is_signed: None,
            integer: false,
            doc: None,
        }
    }
}

impl BablTypeBuilder {
    fn id(mut self, value: i32) -> Self {
        self.id = value;
        self
    }
    fn bits(mut self, value: i32) -> Self {
        self.bits = value;
        self
    }
    fn integer(mut self) -> Self {
        self.integer = true;
        self
    }
    fn min(mut self, value: i64) -> Self {
        self.min = Some(value);
        self
    }
    fn max(mut self, value: i64) -> Self {
        self.max = Some(value);
        self
    }
    fn doc(mut self, value: impl Into<String>) -> Self {
        self.doc = Some(value.into());
        self
    }
    fn min_val(mut self, value: f64) -> Self {
        self.min_val = Some(value);
        self
    }
    fn max_val(mut self, value: f64) -> Self {
        self.max_val = Some(value);
        self
    }
    fn build(self) -> Option<usize> {
        let mut db = DB.write().unwrap();
        let babl = db.exist(self.id, &self.name);

        if self.id != 0 && babl.is_none() && db.exist(0, &self.name).is_some() {
            println!("Trying to reregister BablType '{}' with different id!", &self.name);
        }

        match babl {
            Some(babl) => {
                if !&db.babl_list[babl].lock().unwrap().unwrap_type().is_type_duplicate(self.bits) {
                    println!("BablType '{}' already registered as different type!", self.name);
                }
                Some(babl)
            },
            None => {
                let bt = BablType {
                    from_list: Vec::new(),
                    bits: self.bits,
                    min_val: self.min_val.unwrap(),
                    max_val: self.max_val.unwrap(),
                };
                let babl = Mutex::new(Babl::new_type(self.id, self.name, self.doc.unwrap_or_default(), bt));
                let idx = db.count();
                db.insert(babl);

                Some(idx)
            },
        }
    }
}
