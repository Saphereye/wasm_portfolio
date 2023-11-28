#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub image: Option<String>,
    pub description: String,
    pub year: u32,
}

impl Project {
    pub fn new(name: String, image: Option<String>, description: String, year: u32) -> Self {
        Project {
            name,
            image,
            description,
            year,
        }
    }
}
