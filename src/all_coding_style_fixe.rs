use chrono::Datelike;

use crate::{prelude::Rule, project_operation::Project};

fn fix_c_g1(_: &Rule, file_content: &mut String, project: &Project) -> String {
    let year = chrono::Local::now().year().to_string();
    let header = format!(
        "/*
** EPITECH PROJECT, {}
** {}
** File description:
** {}
*/
",
        year, project.name, project.description
    );
    let mut new_content = header;
    let mut skip = false;
    for line in file_content.lines() {
        if line.starts_with("/*") {
            skip = true;
        }
        if !skip {
            new_content.push_str(line);
            new_content.push_str("\n");
        }
        if line.ends_with("*/") {
            skip = false;
        }
    }
    new_content
}

pub fn get_rules() -> Vec<Rule> {
    let rules = vec![Rule::new("C-G1", "This is a rule", fix_c_g1)];
    rules
}
