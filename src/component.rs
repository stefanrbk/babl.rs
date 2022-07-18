pub(crate) struct BablComponent {
    pub luma: i32,
    pub chroma: i32,
    pub alpha: i32,
}

impl BablComponent {
    pub(crate) fn new(luma: i32, chroma: i32, alpha: i32) -> Self {
        Self {
            luma,
            chroma,
            alpha,
        }
    }
}
