
#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub image: String,
    pub description: String,
    pub year: u32,
}

impl Default for Project {
    fn default() -> Self {
        Project {
            name: String::new(),
            image: String::new(),
            description: String::new(),
            year: 2023,
        }
    }
}

#[derive(Debug)]
pub struct ProjectsApp {
    pub projects: Vec<Project>,
}

impl Default for ProjectsApp {
    fn default() -> Self {
        ProjectsApp { projects: vec![] }
    }
}

impl ProjectsApp {
    pub fn add_project(&mut self, new_project: Project) {
        self.projects.push(new_project);
    }
}
