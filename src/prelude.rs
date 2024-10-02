pub use crate::error::Error;
use crate::project_operation::Project;
use std::{fs, path::PathBuf};

pub type Result<T> = std::result::Result<T, Error>;

type FixMethod = fn(&CodingStyleAlert, &mut String, &Project) -> String;

#[derive(Clone, Debug)]
pub struct Rule {
    pub id: String,
    pub fix_method: FixMethod,
}

impl Rule {
    pub fn new(id: &str, fix_method: FixMethod) -> Self {
        Self {
            id: id.to_string(),
            fix_method,
        }
    }
}

pub fn rule_from_id(id: &str, rules: &[Rule]) -> Option<Rule> {
    rules.iter().find(|&rule| rule.id == id).cloned()
}

#[derive(Debug)]
pub struct CodingStyleAlert {
    pub file: String,
    pub line: usize,
    pub rule: Rule,
}

impl CodingStyleAlert {
    pub fn alerts_from_report(txt: &str, rules: &[Rule], dir: &PathBuf) -> Vec<Self> {
        txt.lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() < 4 {
                    return None;
                }
                let file = dir.join(parts[0]).to_string_lossy().to_string();
                let line = parts[1].parse().ok()?;
                let id = parts[3];

                rule_from_id(id, rules).map(|rule| Self { file, line, rule })
            })
            .collect()
    }

    pub fn fix_issue(&self, dir: &PathBuf, project: &Project) {
        if let Ok(mut file_content) = fs::read_to_string(dir.join(&self.file)) {
            let new_content = (self.rule.fix_method)(&self, &mut file_content, project);
            fs::write(dir.join(&self.file), new_content).unwrap();
        }
    }
}
