#[derive(Debug)]
pub struct Project<'a> {
    pub name: String,
    pub image: Option<egui::Image<'a>>,
    pub description: String,
    pub year: u32,
}

impl<'a> Project<'a> {
    pub fn new(name: String, image: Option<egui::Image<'a>>, description: String, year: u32) -> Self {
        Project {
            name,
            image,
            description,
            year,
        }
    }
}
