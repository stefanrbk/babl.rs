use std::any::Any;

pub(crate) struct BablImage {
    pub format: Option<usize>,
    pub components: usize,
    pub component: Vec<usize>,
    pub r#type: Vec<usize>,
    pub model: usize,
    pub sampling: Vec<usize>,
    pub data: Box<dyn Any>,
    pub pitch: Vec<usize>,
    pub strice: Vec<usize>,
}
