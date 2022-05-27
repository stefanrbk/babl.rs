use crate::Babl;

pub type BablClassType = i32;
pub type BablList = Vec<Babl>;

pub type BablEachFunction = fn(&Babl, Box<[u8]>);

#[repr(C)]
pub struct BablInstance {
    pub class_type: BablClassType,
    pub id: i32,
    pub creator: Box<[u8]>,
    pub name: String,
    _doc: String
}

impl BablInstance {
    /// Creates a new [`BablInstance`].
    fn new(class_type: BablClassType, id: i32, creator: Box<[u8]>, name: String, doc: String) -> Self {
        Self { class_type, id, creator, name, _doc: doc }
    }

    fn doc(&self) -> &String {
        &self._doc
    }
}
