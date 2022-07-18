use std::any::Any;

pub(crate) struct BablFormat {
    pub from_list: Vec<usize>,
    pub components: usize,
    pub component: Vec<usize>,
    pub r#type: Vec<usize>,
    pub model: Option<usize>,
    pub space: Option<usize>,
    pub model_data: Box<dyn Any>,
    pub image_template: Option<Box<dyn Any>>,
    pub sampling: Vec<usize>,
    pub bytes_per_pixel: usize,
    pub planar: bool,
    pub loss: f64,
    pub visited: bool,
    pub format_n: bool,
    pub palette: usize,
    pub encoding: Vec<usize>,
}
