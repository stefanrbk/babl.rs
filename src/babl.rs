use crate::{r#type::BablType, extension::BablExtender};

pub struct Babl {
    pub(crate) id: i32,
    pub(crate) creator: Option<usize>,
    pub(crate) name: String,
    pub(crate) doc: String,
    data: InnerBabl,
}

pub(crate) enum InnerBabl {
    BablType(BablType),
    BablExtender(BablExtender),
}

impl InnerBabl {
    #[must_use]
    pub(crate) fn is_babl_extender(&self) -> bool {
        matches!(self, Self::BablExtender(..))
    }

    pub(crate) fn as_babl_extender(&self) -> Option<&BablExtender> {
        if let Self::BablExtender(v) = self {
            Some(v)
        } else {
            None
        }
    }
    #[must_use]
    pub(crate) fn is_babl_type(&self) -> bool {
        matches!(self, Self::BablType(..))
    }

    pub(crate) fn as_babl_type(&self) -> Option<&BablType> {
        if let Self::BablType(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl Babl {
    pub(crate) fn new(id: i32, name: impl Into<String>, doc: impl Into<String>, data: InnerBabl) -> Self {
        Self{
            id,
            creator: None,
            name: name.into(),
            doc: doc.into(),
            data,
        }
    }
    pub(crate) fn new_type(id: i32, name: impl Into<String>, doc: impl Into<String>, r#type: BablType) -> Babl {
        Self::new(id, name, doc, InnerBabl::BablType(r#type))
    }
    pub(crate) fn new_extension(id: i32, name: impl Into<String>, doc: impl Into<String>, extension: BablExtender) -> Babl {
        Self::new(id, name, doc,InnerBabl::BablExtender(extension))
    }
    pub(crate) fn unwrap_extender(&self) -> &BablExtender {
        self.data.as_babl_extender().unwrap()
    }
    pub(crate) fn unwrap_type(&self) -> &BablType {
        self.data.as_babl_type().unwrap()
    }
}
