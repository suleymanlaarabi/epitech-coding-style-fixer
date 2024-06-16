use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    pub description: String,
}

impl Project {
    pub fn default() -> Self {
        Self {
            name: "Nom du projet".to_string(),
            description: "Description du projet".to_string(),
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
