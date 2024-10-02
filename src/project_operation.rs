use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub delivery_dir: PathBuf,
    pub reports_dir: PathBuf,
}

impl Project {
    pub fn default() -> Self {
        Self {
            name: "Nom du projet".to_string(),
            description: "Description du projet".to_string(),
            delivery_dir: PathBuf::from("delivery"),
            reports_dir: PathBuf::from("reports"),
        }
    }
    pub fn from_dir(project_dir: &PathBuf) -> Result<Self> {
        let project_name = project_dir
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let project_description = "Description du projet".to_string();
        Ok(Self {
            name: project_name,
            description: project_description,
            delivery_dir: project_dir.join("delivery"),
            reports_dir: project_dir.join("reports"),
        })
    }
    pub fn save(&self, project_dir: &PathBuf) -> Result<()> {
        let project_file = project_dir.join("project.json");
        let project_content = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(project_file, project_content).unwrap();
        Ok(())
    }
}

pub fn project_parse(project_dit: &PathBuf) -> Result<Project> {
    let project_file = project_dit.join("project.json");

    let project_content = match std::fs::read_to_string(project_file) {
        Ok(content) => content,
        Err(_) => return Err(Error::NotFound),
    };

    match serde_json::from_str(&project_content) {
        Ok(project) => Ok(project),
        Err(_) => return Err(Error::InvalidData),
    }
}

const CLANG_FORMAT_FILE: &[u8] = include_bytes!("../resources/.clang-format");
const MAIN_C_FILE: &[u8] = include_bytes!("../resources/main.c");
const GITIGNORE_FILE: &[u8] = include_bytes!("../resources/.gitignore");

pub fn init_new_project(project_dir: &PathBuf) -> Result<Project> {
    let project = Project::from_dir(project_dir)?;
    let delivery_dir = project_dir.join("delivery");
    let reports_dir = project_dir.join("reports");
    std::fs::create_dir_all(&delivery_dir)?;
    std::fs::create_dir_all(&reports_dir)?;

    let clang_format_file_path = project_dir.join(".clang-format");
    std::fs::write(clang_format_file_path, CLANG_FORMAT_FILE)?;

    let main_c_file_path = delivery_dir.join("main.c");
    std::fs::write(main_c_file_path, MAIN_C_FILE)?;

    println!("Project initialized with success!");
    let gitignore_file_path = project_dir.join(".gitignore");
    std::fs::write(gitignore_file_path, GITIGNORE_FILE)?;

    project.save(project_dir)?;
    Ok(project)
}

pub fn print_project(project: &Project) {
    println!("Project name: {}", project.name);
    println!("Project description: {}", project.description);
    println!("Delivery directory: {:?}", project.delivery_dir);
    println!("Reports directory: {:?}", project.reports_dir);
}