use chrono::Datelike;

use crate::{project_operation::Project, CodingStyleAlert};

pub fn fix_c_g1(_: &CodingStyleAlert, file_content: &mut String, project: &Project) -> String {
    let year = chrono::Local::now().year().to_string();
    println!("year: {}", year);
    let new_content = format!(
        "/*
** EPITECH PROJECT, {}
** {}
** File description:
** {}
*/

{}
",
        year, project.name, project.description, file_content
    );

    new_content
}
